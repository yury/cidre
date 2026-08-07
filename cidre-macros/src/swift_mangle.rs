//! Mangling a Swift declaration into the symbol its entry point is exported as.
//!
//! This is the half of `#[swift::call]` that replaces the hand-written
//! `#[link_name = "$s..."]` block. It covers the shapes these bindings actually
//! name — getters, setters, initializers, and methods on nominal types — and
//! reports an error rather than guessing on anything else, since a symbol it
//! got wrong is a link error rather than a call into the wrong function.
//!
//! Declarations are written close to Swift source, with each nominal type's
//! kind in parentheses because the symbol encodes it and the source spelling
//! does not:
//!
//! ```text
//! Foundation.UUID(struct).uuidString: String { get }
//! Foundation.UUID(struct).init(uuidString: __shared String) -> Foundation.UUID(struct)?
//! static Speech.SpeechTranscriber(class).isAvailable: Bool { get }
//! DockKit.DockAccessoryManager(class).isSystemTrackingEnabled: Bool { get } thunk
//! ```

/// The words already mangled in this symbol, which later identifiers refer back
/// to instead of spelling out again.
///
/// Swift's mangling is canonical: an identifier that repeats a word another one
/// already introduced *must* use the substitution, so this table is not an
/// optimization but part of producing the right symbol at all.
enum Chunk {
    Word(usize),
    Literal(String),
}

#[derive(Default)]
struct Words {
    words: Vec<String>,
    /// The module the declaration itself lives in.
    ///
    /// A type reference back into that module is reached by substitution rather
    /// than by name, which this cannot reproduce, so naming one is refused.
    module: Option<String>,
}

impl Words {
    /// Splits an identifier the way Swift's mangler does: at each transition
    /// into a capital letter, keeping only words of at least two characters.
    fn split(identifier: &str) -> Vec<String> {
        let chars: Vec<char> = identifier.chars().collect();
        let mut words = Vec::new();
        let mut start = 0usize;
        for index in 1..chars.len() {
            let previous = chars[index - 1];
            let current = chars[index];
            // A word starts at a capital that follows a lowercase letter, or at
            // the last capital of a run that is followed by a lowercase one.
            let starts_word = current.is_uppercase()
                && (previous.is_lowercase()
                    || (index + 1 < chars.len() && chars[index + 1].is_lowercase()));
            if starts_word {
                if index - start >= 2 {
                    words.push(chars[start..index].iter().collect());
                }
                start = index;
            }
        }
        if chars.len() - start >= 2 {
            words.push(chars[start..].iter().collect());
        }
        words
    }

    /// Mangles one identifier, substituting every word already seen.
    ///
    /// The encoding is a `0` followed by chunks: a letter stands for a word the
    /// symbol has already spelled out, and a length-prefixed run stands for
    /// everything between them. The last letter is capitalized, and a trailing
    /// `0` stands for an empty final run.
    fn mangle(&mut self, identifier: &str) -> String {
        let words = Self::split(identifier);

        // Walk the identifier, replacing each word already in the table and
        // accumulating everything else into literal runs.
        let mut chunks: Vec<Chunk> = Vec::new();
        let mut literal = String::new();
        let mut rest = identifier;
        let mut new_words = Vec::new();

        for word in &words {
            let Some(offset) = rest.find(word.as_str()) else {
                continue;
            };
            match self.words.iter().position(|w| w == word) {
                Some(index) => {
                    literal.push_str(&rest[..offset]);
                    if !literal.is_empty() {
                        chunks.push(Chunk::Literal(std::mem::take(&mut literal)));
                    }
                    chunks.push(Chunk::Word(index));
                }
                None => {
                    literal.push_str(&rest[..offset + word.len()]);
                    new_words.push(word.clone());
                }
            }
            rest = &rest[offset + word.len()..];
        }
        literal.push_str(rest);

        for word in new_words {
            if self.words.len() < 26 && !self.words.iter().any(|w| *w == word) {
                self.words.push(word);
            }
        }

        if chunks.is_empty() {
            return format!("{}{}", identifier.len(), identifier);
        }
        chunks.push(Chunk::Literal(literal));

        let last_word = chunks
            .iter()
            .rposition(|c| matches!(c, Chunk::Word(_)))
            .unwrap();
        let mut out = String::from("0");
        for (position, chunk) in chunks.iter().enumerate() {
            match chunk {
                Chunk::Word(index) => {
                    let letter = (b'a' + *index as u8) as char;
                    out.push(if position == last_word {
                        letter.to_ascii_uppercase()
                    } else {
                        letter
                    });
                }
                Chunk::Literal(text) if text.is_empty() => out.push('0'),
                Chunk::Literal(text) => out.push_str(&format!("{}{}", text.len(), text)),
            }
        }
        out
    }
}

/// The kind letter Swift mangles a nominal type with.
pub fn kind_letter(kind: &str) -> Result<char, String> {
    match kind {
        "struct" => Ok('V'),
        "class" => Ok('C'),
        "enum" => Ok('O'),
        "protocol" => Ok('P'),
        other => Err(format!("unknown type kind `{other}`")),
    }
}

/// One `Name(kind)` component of a declaration's context path.
struct Component {
    name: String,
    kind: char,
}

/// Splits `A.B(class).C(struct)` into its components, respecting nesting.
fn parse_context(text: &str) -> Result<(String, Vec<Component>), String> {
    let parts = split_path(text);
    let mut iter = parts.into_iter();
    let module = iter.next().ok_or("empty context")?;
    let mut components = Vec::new();
    for part in iter {
        let (name, kind) = match part.split_once('(') {
            Some((name, rest)) => {
                let kind = rest.trim_end_matches(')');
                (name.to_string(), kind_letter(kind)?)
            }
            None => return Err(format!("`{part}` needs a kind, as in `{part}(struct)`")),
        };
        components.push(Component { name, kind });
    }
    Ok((module, components))
}

/// Splits on `.` at nesting depth zero.
fn split_path(text: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut current = String::new();
    for ch in text.chars() {
        match ch {
            '(' | '<' | '[' => depth += 1,
            ')' | '>' | ']' => depth -= 1,
            '.' if depth == 0 => {
                parts.push(current.trim().to_string());
                current = String::new();
                continue;
            }
            _ => {}
        }
        current.push(ch);
    }
    if !current.trim().is_empty() {
        parts.push(current.trim().to_string());
    }
    parts
}

/// Mangles a type reference.
fn mangle_type(text: &str, words: &mut Words) -> Result<String, String> {
    let text = text.trim();

    if let Some(inner) = text.strip_suffix('?') {
        return Ok(format!("{}Sg", mangle_type(inner, words)?));
    }
    // The standard library's sugared generics: a `y` opens the argument list
    // and a `G` closes it, the same as any other bound generic type.
    if text.starts_with('[') && text.ends_with(']') {
        let inner = &text[1..text.len() - 1];
        if let Some((key, value)) = inner.split_once(':') {
            return Ok(format!(
                "SDy{}{}G",
                mangle_type(key, words)?,
                mangle_type(value, words)?
            ));
        }
        return Ok(format!("Say{}G", mangle_type(inner, words)?));
    }
    if let Some(inner) = text.strip_prefix("Set<").and_then(|t| t.strip_suffix('>')) {
        return Ok(format!("Shy{}G", mangle_type(inner, words)?));
    }
    // A type imported from C lives in the `__C` pseudo-module, and a C struct
    // arrives as a typealias rather than a nominal type of its own.
    if let Some(rest) = text.strip_prefix("__C.") {
        let (name, kind) = match rest.split_once('(') {
            Some((name, kind)) => (name, kind_letter(kind.trim_end_matches(')'))?),
            None => (rest, 'a'),
        };
        return Ok(format!("So{}{}", words.mangle(name), kind));
    }

    Ok(match text {
        "Void" | "()" => "yt".to_string(),
        "Int" => "Si".to_string(),
        "UInt" => "Su".to_string(),
        "Bool" => "Sb".to_string(),
        "Double" => "Sd".to_string(),
        "Float" => "Sf".to_string(),
        "String" => "SS".to_string(),
        "T" => "x".to_string(),
        _ => {
            let (module, components) = parse_context(text)?;
            if components.is_empty() {
                return Err(format!("`{text}` is not a known type"));
            }
            if words.module.as_deref() == Some(module.as_str()) {
                return Err(format!(
                    "`{text}` is in the declaration's own module, which the symbol reaches \
                     by back reference rather than by name; give this one as `sym = \"$s...\"`"
                ));
            }
            let mut out = words.mangle(&module);
            for component in &components {
                out.push_str(&words.mangle(&component.name));
                out.push(component.kind);
            }
            out
        }
    })
}

/// A parameter of a declaration.
struct Param {
    label: Option<String>,
    ty: String,
    /// `n` for `__owned`, `h` for `__shared`, `z` for `inout`.
    ownership: Option<char>,
}

fn parse_params(text: &str) -> Result<Vec<Param>, String> {
    let text = text.trim();
    if text.is_empty() {
        return Ok(Vec::new());
    }
    let mut params = Vec::new();
    for part in split_commas(text) {
        let (names, ty) = part
            .split_once(':')
            .ok_or_else(|| format!("parameter `{part}` needs a type"))?;
        let label = names.split_whitespace().next().unwrap_or("_");
        let mut ty = ty.trim();
        let mut ownership = None;
        for (keyword, letter) in [("__owned", 'n'), ("__shared", 'h'), ("inout", 'z')] {
            if let Some(rest) = ty.strip_prefix(keyword) {
                ownership = Some(letter);
                ty = rest.trim();
            }
        }
        params.push(Param {
            label: (label != "_").then(|| label.to_string()),
            ty: ty.to_string(),
            ownership,
        });
    }
    Ok(params)
}

fn split_commas(text: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut current = String::new();
    for ch in text.chars() {
        match ch {
            '(' | '<' | '[' => depth += 1,
            ')' | '>' | ']' => depth -= 1,
            ',' if depth == 0 => {
                parts.push(current.trim().to_string());
                current = String::new();
                continue;
            }
            _ => {}
        }
        current.push(ch);
    }
    if !current.trim().is_empty() {
        parts.push(current.trim().to_string());
    }
    parts
}

/// Mangles the parameter list, which is one type bare and several as a tuple.
fn mangle_params(params: &[Param], words: &mut Words) -> Result<String, String> {
    if params.is_empty() {
        return Ok("y".to_string());
    }
    // A lone unlabelled parameter is the argument type itself; anything else is
    // a tuple, including a single parameter that carries a label. A tuple marks
    // only its first element with `_`, whatever it goes on to hold, and closes
    // with `t`.
    let tuple = params.len() > 1 || params[0].label.is_some();
    let mut out = String::new();
    for (index, param) in params.iter().enumerate() {
        out.push_str(&mangle_type(&param.ty, words)?);
        if let Some(letter) = param.ownership {
            out.push(letter);
        }
        if tuple && index == 0 {
            out.push('_');
        }
    }
    if tuple {
        out.push('t');
    }
    Ok(out)
}

/// The label list, which is empty when every label is.
fn mangle_labels(params: &[Param], words: &mut Words) -> String {
    if params.iter().all(|p| p.label.is_none()) {
        return String::new();
    }
    let mut out = String::new();
    for param in params {
        match &param.label {
            Some(label) => out.push_str(&words.mangle(label)),
            None => out.push('_'),
        }
    }
    out
}

/// Mangles the symbol of a type's metadata accessor.
///
/// `path` names the type as Swift spells it, with a kind on every component
/// but the last, whose kind is `kind`. This is the whole of a nominal type's
/// symbol — context and nothing else — so unlike a member it needs none of the
/// substitution bookkeeping a function signature does.
pub fn mangle_metadata_accessor(path: &str, kind: char) -> Result<String, String> {
    let parts = split_path(path.trim());
    if parts.len() < 2 {
        return Err(format!("`{path}` must name a module and a type"));
    }

    let mut words = Words::default();
    let mut out = format!("$s{}", words.mangle(&parts[0]));

    let last = parts.len() - 1;
    for (index, part) in parts[1..].iter().enumerate() {
        let (name, kind) = match part.split_once('(') {
            Some((name, rest)) => (name, kind_letter(rest.trim_end_matches(')'))?),
            None if index + 1 == last => (part.as_str(), kind),
            None => {
                return Err(format!(
                    "`{part}` encloses the type, so it needs a kind, as in `{part}(class)`"
                ));
            }
        };
        out.push_str(&words.mangle(name));
        out.push(kind);
    }

    out.push_str("Ma");
    Ok(out)
}

/// Whether the callee takes ownership of each parameter, in declaration order.
///
/// Swift's defaults differ by member: an initializer's parameters arrive at
/// `+1` unless marked `__shared`, while a method borrows unless marked
/// `__owned`. A caller that gets this backwards either leaks the value or
/// releases one it never owned, and neither shows up as a compile error, so it
/// is read off the declaration rather than assumed.
pub fn param_conventions(decl: &str) -> Result<Vec<bool>, String> {
    let mut decl = decl.trim();
    decl = decl.strip_prefix("static ").unwrap_or(decl).trim();
    decl = decl.strip_suffix(" thunk").unwrap_or(decl).trim();

    let (_, member) = split_member(decl)?;
    Ok(match member {
        Member::Property { .. } => Vec::new(),
        Member::Init { params, .. } => params
            .iter()
            .map(|p| p.ownership != Some('h'))
            .collect(),
        Member::Method { params, .. } => params
            .iter()
            .map(|p| p.ownership == Some('n'))
            .collect(),
    })
}

/// Whether the declaration names a suspending function.
///
/// What tells `#[swift::call]` to await the call on a Swift task rather than
/// make it directly, since a suspending function cannot be called at all
/// without the caller allocating its context first.
pub fn is_async(decl: &str) -> Result<bool, String> {
    let mut decl = decl.trim();
    decl = decl.strip_prefix("static ").unwrap_or(decl).trim();
    decl = decl.strip_suffix(" thunk").unwrap_or(decl).trim();

    Ok(match split_member(decl)?.1 {
        Member::Property { .. } => false,
        Member::Init { is_async, .. } | Member::Method { is_async, .. } => is_async,
    })
}

/// Mangles a Swift declaration into its exported symbol.
///
/// A suspending function has a second symbol beside this one: its async
/// function pointer, the record holding the entry point and the size of the
/// context a caller has to allocate. That symbol is this one with `Tu`
/// appended — after the `Tj` of a dispatch thunk, since what a caller awaits
/// through a thunk is the thunk.
pub fn mangle(decl: &str) -> Result<String, String> {
    let mut decl = decl.trim();
    let mut is_static = false;
    if let Some(rest) = decl.strip_prefix("static ") {
        is_static = true;
        decl = rest.trim();
    }
    // A class member reached through the vtable is called through its dispatch
    // thunk, which is a distinct symbol.
    let mut thunk = false;
    if let Some(rest) = decl.strip_suffix(" thunk") {
        thunk = true;
        decl = rest.trim();
    }

    let mut words = Words::default();

    // Split the declaration into the context path and the member.
    let (head, member) = split_member(decl)?;
    let (module, components) = parse_context(&head)?;

    let mut context = words.mangle(&module);
    for component in &components {
        context.push_str(&words.mangle(&component.name));
        context.push(component.kind);
    }
    words.module = Some(module.clone());

    let body = match member {
        Member::Property { name, ty, accessor } => {
            let name = words.mangle(&name);
            let ty = mangle_type(&ty, &mut words)?;
            let accessor = match accessor.as_str() {
                "get" => "vg",
                "set" => "vs",
                other => return Err(format!("unknown accessor `{other}`")),
            };
            format!("{name}{ty}{accessor}")
        }
        Member::Init {
            params,
            failable,
            is_async,
            throws,
        } => {
            // An initializer's result is the enclosing type, which the symbol
            // reaches by back reference rather than by name. Only a type
            // directly inside its module gets this reference; one nested in
            // another type is reached differently, and guessing would produce
            // a symbol that links to nothing.
            if components.len() != 1 {
                return Err(format!(
                    "`{head}` is nested, and a nested type's initializer needs \
                     its symbol given as `sym = \"$s...\"`"
                ));
            }
            let labels = mangle_labels(&params, &mut words);
            let result = if failable { "ACSg" } else { "AC" };
            let params = mangle_params(&params, &mut words)?;
            let is_async = if is_async { "Ya" } else { "" };
            let throws = if throws { "K" } else { "" };
            format!("{labels}{result}{params}{is_async}{throws}cfC")
        }
        Member::Method {
            name,
            params,
            result,
            is_async,
            throws,
        } => {
            let name = words.mangle(&name);
            let labels = mangle_labels(&params, &mut words);
            let labels = if labels.is_empty() {
                "y".to_string()
            } else {
                labels
            };
            let result = match result {
                Some(ty) => mangle_type(&ty, &mut words)?,
                None => "y".to_string(),
            };
            let params = mangle_params(&params, &mut words)?;
            let is_async = if is_async { "Ya" } else { "" };
            let throws = if throws { "K" } else { "" };
            format!("{name}{labels}{result}{params}{is_async}{throws}F")
        }
    };

    let mut out = format!("$s{context}{body}");
    if is_static {
        out.push('Z');
    }
    if thunk {
        out.push_str("Tj");
    }
    Ok(out)
}

enum Member {
    Property {
        name: String,
        ty: String,
        accessor: String,
    },
    Init {
        params: Vec<Param>,
        /// `init?`, whose result is the enclosing type wrapped in an optional.
        failable: bool,
        is_async: bool,
        throws: bool,
    },
    Method {
        name: String,
        params: Vec<Param>,
        result: Option<String>,
        is_async: bool,
        throws: bool,
    },
}

/// Splits the declaration into the context path and the member it names.
fn split_member(decl: &str) -> Result<(String, Member), String> {
    // A property: `<path>.<name>: <Type> { get }`
    if let Some(brace) = decl.rfind('{') {
        let accessor = decl[brace + 1..].trim_end_matches('}').trim().to_string();
        let head = decl[..brace].trim();
        // The type may itself contain a colon, as a dictionary does, so the
        // one that separates it from the name is the first at depth zero.
        let split = depth_zero_colon(head)
            .ok_or_else(|| format!("`{decl}` is not a property declaration"))?;
        let (path, ty) = (&head[..split], &head[split + 1..]);
        let parts = split_path(path.trim());
        let name = parts
            .last()
            .cloned()
            .ok_or_else(|| format!("`{decl}` has no member name"))?;
        let context = parts[..parts.len() - 1].join(".");
        return Ok((
            context,
            Member::Property {
                name,
                ty: ty.trim().to_string(),
                accessor,
            },
        ));
    }

    // A function: `<path>.<name>(<params>) [async] [throws] [-> Result]`
    let (before_result, result) = match decl.split_once("->") {
        Some((before, result)) => (before.trim(), Some(result.trim().to_string())),
        None => (decl.trim(), None),
    };
    let mut before_result = before_result;
    let mut throws = false;
    if let Some(rest) = before_result.strip_suffix("throws") {
        throws = true;
        before_result = rest.trim();
    }
    // Stripped after `throws`, since Swift writes the effects in that order.
    let mut is_async = false;
    if let Some(rest) = before_result.strip_suffix("async") {
        is_async = true;
        before_result = rest.trim();
    }

    let open = before_result
        .find('(')
        .ok_or_else(|| format!("`{decl}` is neither a property nor a function"))?;
    // The parenthesis that opens the parameter list is the last one that is not
    // part of a `(kind)` marker.
    let open = before_result[..open + 1]
        .rfind('(')
        .map(|_| find_param_paren(before_result))
        .unwrap_or(open);

    let path = before_result[..open].trim();
    let params_text = before_result[open + 1..]
        .trim()
        .trim_end_matches(')')
        .to_string();
    let params = parse_params(&params_text)?;

    let parts = split_path(path);
    let name = parts
        .last()
        .cloned()
        .ok_or_else(|| format!("`{decl}` has no member name"))?;
    let context = parts[..parts.len() - 1].join(".");

    // `init?` is a failable initializer, whose result is the optional.
    if name == "init" || name == "init?" {
        let failable = name.ends_with('?');
        return Ok((
            context,
            Member::Init {
                params,
                failable,
                is_async,
                throws,
            },
        ));
    }
    Ok((
        context,
        Member::Method {
            name,
            params,
            result,
            is_async,
            throws,
        },
    ))
}

/// The offset of the colon separating a property's name from its type.
fn depth_zero_colon(text: &str) -> Option<usize> {
    let mut depth = 0i32;
    for (offset, ch) in text.char_indices() {
        match ch {
            '(' | '<' | '[' => depth += 1,
            ')' | '>' | ']' => depth -= 1,
            ':' if depth == 0 => return Some(offset),
            _ => {}
        }
    }
    None
}

/// Finds the parenthesis that opens the parameter list, skipping the `(kind)`
/// markers attached to type names.
fn find_param_paren(text: &str) -> usize {
    let bytes: Vec<char> = text.chars().collect();
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] == '(' {
            // A kind marker is a parenthesized word followed by `.`.
            let close = text[index..].find(')').map(|offset| index + offset);
            let is_kind = match close {
                Some(close) => {
                    let inner = &text[index + 1..close];
                    matches!(inner, "struct" | "class" | "enum" | "protocol")
                }
                None => false,
            };
            if !is_kind {
                return index;
            }
            index = close.unwrap() + 1;
            continue;
        }
        index += 1;
    }
    text.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every case here is a symbol the bindings already link against, taken
    /// from their `#[link_name]` attributes, so the mangler is measured against
    /// what the frameworks really export.
    #[test]
    fn mangles_symbols_the_bindings_already_use() {
        let cases = [
            (
                "Foundation.UUID(struct).uuidString: String { get }",
                "$s10Foundation4UUIDV10uuidStringSSvg",
            ),
            (
                "Foundation.Locale(struct).identifier: String { get }",
                "$s10Foundation6LocaleV10identifierSSvg",
            ),
            (
                "Foundation.Date(struct).timeIntervalSinceReferenceDate: Double { get }",
                "$s10Foundation4DateV026timeIntervalSinceReferenceB0Sdvg",
            ),
            (
                "Foundation.UUID(struct).init()",
                "$s10Foundation4UUIDVACycfC",
            ),
            (
                "Foundation.Date(struct).init()",
                "$s10Foundation4DateVACycfC",
            ),
            (
                "Foundation.UUID(struct).init?(uuidString: __shared String)",
                "$s10Foundation4UUIDV10uuidStringACSgSSh_tcfC",
            ),
            (
                "Foundation.Locale(struct).init(identifier: String)",
                "$s10Foundation6LocaleV10identifierACSS_tcfC",
            ),
            (
                "MusicUnderstanding.RhythmResult(struct).beatsPerMinute: Float? { get }",
                "$s18MusicUnderstanding12RhythmResultV14beatsPerMinuteSfSgvg",
            ),
            (
                "MusicUnderstanding.RhythmResult(struct).beats: [__C.CMTime] { get }",
                "$s18MusicUnderstanding12RhythmResultV5beatsSaySo6CMTimeaGvg",
            ),
            (
                "MusicUnderstanding.RhythmResult(struct).bars: [__C.CMTime] { get }",
                "$s18MusicUnderstanding12RhythmResultV4barsSaySo6CMTimeaGvg",
            ),
            (
                "static Speech.SpeechTranscriber(class).isAvailable: Bool { get }",
                "$s6Speech0A11TranscriberC11isAvailableSbvgZ",
            ),
            (
                "DockKit.DockAccessoryManager(class).isSystemTrackingEnabled: Bool { get } thunk",
                "$s7DockKit0A16AccessoryManagerC23isSystemTrackingEnabledSbvgTj",
            ),
            // Suspending functions, whose effects mangle as `Ya` before `K`.
            (
                "DockKit.DockAccessory(class).setAngularVelocity(_: __C.SPVector3D) async throws",
                "$s7DockKit0A9AccessoryC18setAngularVelocityyySo10SPVector3DaYaKF",
            ),
            (
                "DockKit.DockAccessory(class).selectSubject(at: __C.CGPoint(struct)) async throws",
                "$s7DockKit0A9AccessoryC13selectSubject2atySo7CGPointV_tYaKF",
            ),
            (
                "DockKit.DockAccessory(class).selectSubjects(_: [Foundation.UUID(struct)]) async throws",
                "$s7DockKit0A9AccessoryC14selectSubjectsyySay10Foundation4UUIDVGYaKF",
            ),
            (
                "DockKit.DockAccessory(class).setRegionOfInterest(_: __C.CGRect(struct)) async throws",
                "$s7DockKit0A9AccessoryC19setRegionOfInterestyySo6CGRectVYaKF",
            ),
            (
                "DockKit.DockAccessoryManager(class).setSystemTrackingEnabled(_: Bool) async throws thunk",
                "$s7DockKit0A16AccessoryManagerC24setSystemTrackingEnabledyySbYaKFTj",
            ),
        ];

        let mut failures = Vec::new();
        for (decl, expected) in cases {
            match mangle(decl) {
                Ok(actual) if actual == expected => {}
                Ok(actual) => failures.push(format!("{decl}\n  want {expected}\n  got  {actual}")),
                Err(e) => failures.push(format!("{decl}\n  error {e}")),
            }
        }
        assert!(failures.is_empty(), "\n{}", failures.join("\n"));
    }

    /// A suspending call needs the function's own symbol and the async function
    /// pointer that sizes its context, and both come from the one declaration.
    #[test]
    fn mangles_the_async_function_pointer_beside_the_entry_point() {
        let decl =
            "DockKit.DockAccessory(class).selectSubjects(_: [Foundation.UUID(struct)]) async throws";
        assert_eq!(
            "$s7DockKit0A9AccessoryC14selectSubjectsyySay10Foundation4UUIDVGYaKFTu",
            format!("{}Tu", mangle(decl).unwrap())
        );

        // Through a dispatch thunk the pointer is the thunk's, not the
        // function's, so `Tu` goes after `Tj` rather than before it.
        let thunked =
            "DockKit.DockAccessoryManager(class).setSystemTrackingEnabled(_: Bool) async throws thunk";
        assert_eq!(
            "$s7DockKit0A16AccessoryManagerC24setSystemTrackingEnabledyySbYaKFTjTu",
            format!("{}Tu", mangle(thunked).unwrap())
        );
    }

    /// `async` is what picks the expansion, so it has to be read off exactly the
    /// declarations that carry it.
    #[test]
    fn reads_async_off_the_declaration() {
        let cases = [
            (
                "DockKit.DockAccessory(class).selectSubject(at: __C.CGPoint(struct)) async throws",
                true,
            ),
            (
                "DockKit.DockAccessory(class).setLimits(_: __C.CGRect(struct)) throws",
                false,
            ),
            (
                "DockKit.DockAccessoryManager(class).setSystemTrackingEnabled(_: Bool) async throws thunk",
                true,
            ),
            ("Foundation.UUID(struct).init()", false),
            ("Foundation.UUID(struct).uuidString: String { get }", false),
        ];

        for (decl, expected) in cases {
            assert_eq!(Ok(expected), is_async(decl), "{decl}");
        }
    }


    /// Every metadata accessor the bindings link against today, so the mangler
    /// is measured against what the frameworks really export rather than
    /// against a rule written down from the same guess it encodes.
    #[test]
    fn mangles_every_metadata_accessor_the_bindings_use() {
        let cases = [
            ("Foundation.Notification", 'V', "$s10Foundation12NotificationVMa"),
            ("Foundation.AttributedString(struct).CharacterView", 'V', "$s10Foundation16AttributedStringV13CharacterViewVMa"),
            ("Foundation.AttributedString", 'V', "$s10Foundation16AttributedStringVMa"),
            ("Foundation.Date", 'V', "$s10Foundation4DateVMa"),
            ("Foundation.UUID", 'V', "$s10Foundation4UUIDVMa"),
            ("Foundation.Locale", 'V', "$s10Foundation6LocaleVMa"),
            ("MusicUnderstanding.MusicUnderstandingSession(class).SessionResult", 'V', "$s18MusicUnderstanding0aB7SessionC0C6ResultVMa"),
            ("MusicUnderstanding.MusicUnderstandingSession", 'C', "$s18MusicUnderstanding0aB7SessionCMa"),
            ("MusicUnderstanding.RhythmResult", 'V', "$s18MusicUnderstanding12RhythmResultVMa"),
            ("MusicUnderstanding.LoudnessResult", 'V', "$s18MusicUnderstanding14LoudnessResultVMa"),
            ("MusicUnderstanding.InstrumentActivityResult", 'V', "$s18MusicUnderstanding24InstrumentActivityResultVMa"),
            ("Speech.SpeechTranscriber(class).Result", 'V', "$s6Speech0A11TranscriberC6ResultVMa"),
            ("Speech.SpeechTranscriber", 'C', "$s6Speech0A11TranscriberCMa"),
            ("Speech.SpeechAnalyzer(class).Options", 'V', "$s6Speech0A8AnalyzerC7OptionsVMa"),
            ("Speech.SpeechAnalyzer", 'C', "$s6Speech0A8AnalyzerCMa"),
            ("Speech.SpeechDetector(class).DetectionOptions", 'V', "$s6Speech0A8DetectorC16DetectionOptionsVMa"),
            ("Speech.SpeechDetector(class).SensitivityLevel", 'O', "$s6Speech0A8DetectorC16SensitivityLevelOMa"),
            ("Speech.SpeechDetector", 'C', "$s6Speech0A8DetectorCMa"),
            ("Speech.DictationTranscriber(class).Result", 'V', "$s6Speech20DictationTranscriberC6ResultVMa"),
            ("Speech.DictationTranscriber", 'C', "$s6Speech20DictationTranscriberCMa"),
            ("Speech.CaptureInputSequenceProvider", 'C', "$s6Speech28CaptureInputSequenceProviderCMa"),
            ("DockKit.DockAccessoryManager", 'C', "$s7DockKit0A16AccessoryManagerCMa"),
            ("DockKit.DockAccessory(class).AccessoryEvent", 'O', "$s7DockKit0A9AccessoryC0C5EventOMa"),
            ("DockKit.DockAccessory(class).Identifier", 'V', "$s7DockKit0A9AccessoryC10IdentifierVMa"),
            ("DockKit.DockAccessory(class).MotionState", 'V', "$s7DockKit0A9AccessoryC11MotionStateVMa"),
            ("DockKit.DockAccessory(class).Observation", 'V', "$s7DockKit0A9AccessoryC11ObservationVMa"),
            ("DockKit.DockAccessory(class).StateChange", 'V', "$s7DockKit0A9AccessoryC11StateChangeVMa"),
            ("DockKit.DockAccessory(class).BatteryState", 'V', "$s7DockKit0A9AccessoryC12BatteryStateVMa"),
            ("DockKit.DockAccessory(class).TrackedObject", 'V', "$s7DockKit0A9AccessoryC13TrackedObjectVMa"),
            ("DockKit.DockAccessory(class).TrackedPerson", 'V', "$s7DockKit0A9AccessoryC13TrackedPersonVMa"),
            ("DockKit.DockAccessory(class).TrackingState", 'V', "$s7DockKit0A9AccessoryC13TrackingStateVMa"),
            ("DockKit.DockAccessory(class).CameraInformation", 'V', "$s7DockKit0A9AccessoryC17CameraInformationVMa"),
            ("DockKit.DockAccessory(class).TrackedSubjectType", 'O', "$s7DockKit0A9AccessoryC18TrackedSubjectTypeOMa"),
            ("DockKit.DockAccessory(class).Limits(struct).Limit", 'V', "$s7DockKit0A9AccessoryC6LimitsV5LimitVMa"),
            ("DockKit.DockAccessory(class).Limits", 'V', "$s7DockKit0A9AccessoryC6LimitsVMa"),
        ];

        let mut failures = Vec::new();
        for (path, kind, expected) in cases {
            match mangle_metadata_accessor(path, kind) {
                Ok(actual) if actual == expected => {}
                Ok(actual) => failures.push(format!("{path}\n  want {expected}\n  got  {actual}")),
                Err(e) => failures.push(format!("{path}\n  error {e}")),
            }
        }
        assert!(failures.is_empty(), "\n{}", failures.join("\n"));
    }

    /// A tuple marks only its first element, which is what makes a
    /// one-parameter initializer `Si_t` rather than `Sit`.
    #[test]
    fn a_parameter_tuple_marks_only_its_first_element() {
        let mut words = Words::default();
        let params = parse_params("a: Int, b: Double, c: Bool, d: String").unwrap();
        assert_eq!("Si_SdSbSSt", mangle_params(&params, &mut words).unwrap());

        let mut words = Words::default();
        let one = parse_params("a: Int").unwrap();
        assert_eq!("Si_t", mangle_params(&one, &mut words).unwrap());

        // Without a label there is no tuple at all.
        let mut words = Words::default();
        let bare = parse_params("_ a: Int").unwrap();
        assert_eq!("Si", mangle_params(&bare, &mut words).unwrap());
    }

    /// The two initializers below differ only in `__shared`, and getting the
    /// convention backwards is a leak or a double release rather than a
    /// compile error, so the rule itself is pinned here.
    #[test]
    fn a_declaration_says_who_owns_each_argument() {
        assert_eq!(
            vec![false],
            param_conventions("Foundation.UUID(struct).init?(uuidString: __shared String)")
                .unwrap(),
            "`__shared` is borrowed"
        );
        assert_eq!(
            vec![true],
            param_conventions("Foundation.Locale(struct).init(identifier: String)").unwrap(),
            "an initializer otherwise takes its argument at +1"
        );
        // A method is the other way round.
        assert_eq!(
            vec![false],
            param_conventions("Some.Type(class).take(_ value: String)").unwrap(),
            "a method borrows by default"
        );
        assert_eq!(
            vec![true],
            param_conventions("Some.Type(class).take(_ value: __owned String)").unwrap(),
        );
    }


    /// The `DockAccessory` getters, against the symbols the bindings linked
    /// against before they were declared.
    #[test]
    fn mangles_the_dock_accessory_getters() {
        let cases = [
            (
                "DockKit.DockAccessory(class).firmwareVersion: String? { get }",
                "$s7DockKit0A9AccessoryC15firmwareVersionSSSgvg",
            ),
            (
                "DockKit.DockAccessory(class).hashValue: Int { get }",
                "$s7DockKit0A9AccessoryC9hashValueSivg",
            ),
            (
                "DockKit.DockAccessory(class).hardwareModel: String? { get }",
                "$s7DockKit0A9AccessoryC13hardwareModelSSSgvg",
            ),
            (
                "DockKit.DockAccessory(class).debugDescription: String { get }",
                "$s7DockKit0A9AccessoryC16debugDescriptionSSvg",
            ),
            (
                "DockKit.DockAccessory(class).regionOfInterest: __C.CGRect(struct) { get }",
                "$s7DockKit0A9AccessoryC16regionOfInterestSo6CGRectVvg",
            ),
        ];
        for (decl, expected) in cases {
            assert_eq!(expected, mangle(decl).unwrap(), "{decl}");
        }
    }

    /// Naming a type from the declaration's own module has to be refused rather
    /// than spelled out, since the real symbol reaches it by back reference and
    /// a spelled-out one links to nothing.
    #[test]
    fn refuses_a_type_in_the_declarations_own_module() {
        let err = mangle(
            "DockKit.DockAccessory(class).limits: DockKit.DockAccessory(class).Limits(struct) { get }",
        )
        .expect_err("must not guess a back reference");
        assert!(err.contains("back reference"), "{err}");
    }
}
