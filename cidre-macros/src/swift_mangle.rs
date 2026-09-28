//! Mangling a Swift declaration into the symbol its entry point is exported as.
//!
//! This is the half of `#[swift::call]` that replaces the hand-written
//! `#[link_name = "$s..."]` block. It covers the shapes these bindings actually
//! name — getters, setters, initializers, methods, enum cases and metadata
//! accessors of nominal types — and reports an error rather than guessing on
//! anything else, since a symbol it got wrong is a link error rather than a call
//! into the wrong function.
//!
//! Declarations are written close to Swift source, with each nominal type's
//! kind in parentheses because the symbol encodes it and the source spelling
//! does not:
//!
//! ```text
//! Foundation.UUID(struct).uuidString: String { get }
//! Foundation.UUID(struct).init?(uuidString: __shared String)
//! static Speech.SpeechTranscriber(class).isAvailable: Bool { get }
//! DockKit.DockAccessoryManager(class).isSystemTrackingEnabled: Bool { get } thunk
//! DockKit.DockAccessory(class).limits: DockKit.DockAccessory(class).Limits(struct) { get }
//! ```
//!
//! Swift's mangling is canonical, and most of what makes it so is back
//! references: an identifier, a nominal type, or a bound generic type that the
//! symbol has already spelled out is referred to by its index rather than
//! spelled again, and the words inside identifiers are shared the same way.
//! Neither is an optimization — the symbol that spells a repeat out is a
//! different symbol — so [`Mangler`] keeps the same tables the compiler does.

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

/// A type as a declaration spells it.
#[derive(Clone, Debug)]
enum Ty {
    /// `()`.
    Void,
    /// A standard library type with a one-letter substitution of its own, such
    /// as `Si` for `Int`.
    Std(char),
    /// The generic parameter of the enclosing type or function.
    Generic,
    Nominal(Nominal),
    Optional(Box<Ty>),
    Array(Box<Ty>),
    Dictionary(Box<Ty>, Box<Ty>),
    Set(Box<Ty>),
    /// `Range<Bound>`, which has a standard substitution of its own.
    Range(Box<Ty>),
    /// `any P` for a single protocol.
    Existential(Nominal),
    /// `T.Type`.
    Metatype(Box<Ty>),
    /// Only built internally, for an enum case's payload.
    Function(Vec<Param>, Box<Ty>),
    /// A type with a two-letter standard substitution, such as the
    /// concurrency library's `ScP` for `TaskPriority`.
    Special(&'static str),
}

/// A nominal type: its module and the path of types down to it.
#[derive(Clone, Debug)]
struct Nominal {
    module: String,
    components: Vec<Component>,
}

/// One `Name(kind)<Args>` step of a nominal type's path.
#[derive(Clone, Debug)]
struct Component {
    name: String,
    kind: char,
    args: Vec<Ty>,
}

impl Nominal {
    /// The declaration's key for its first `depth` components, which is what
    /// the substitution table knows an unbound declaration by.
    fn decl_key(&self, depth: usize) -> String {
        let names: Vec<&str> = self.components[..depth]
            .iter()
            .map(|c| c.name.as_str())
            .collect();
        format!("d:{}.{}", self.module, names.join("."))
    }

    fn key(&self) -> String {
        let mut out = self.module.clone();
        for component in &self.components {
            out.push('.');
            out.push_str(&component.name);
            if !component.args.is_empty() {
                let args: Vec<String> = component.args.iter().map(Ty::key).collect();
                out.push_str(&format!("<{}>", args.join(",")));
            }
        }
        out
    }

    fn is_bound(&self) -> bool {
        self.components.iter().any(|c| !c.args.is_empty())
    }
}

impl Ty {
    /// A canonical spelling, which is what the substitution table knows a
    /// type by.
    fn key(&self) -> String {
        match self {
            Ty::Void => "()".to_string(),
            Ty::Std(letter) => format!("S{letter}"),
            Ty::Generic => "x".to_string(),
            Ty::Nominal(nominal) => nominal.key(),
            Ty::Optional(inner) => format!("{}?", inner.key()),
            Ty::Array(inner) => format!("[{}]", inner.key()),
            Ty::Dictionary(key, value) => format!("[{}:{}]", key.key(), value.key()),
            Ty::Set(inner) => format!("Set<{}>", inner.key()),
            Ty::Range(inner) => format!("Range<{}>", inner.key()),
            Ty::Existential(protocol) => format!("any {}", protocol.key()),
            Ty::Metatype(inner) => format!("{}.Type", inner.key()),
            Ty::Function(params, result) => {
                let params: Vec<String> = params.iter().map(|p| p.ty.key()).collect();
                format!("({})->{}", params.join(","), result.key())
            }
            Ty::Special(text) => text.to_string(),
        }
    }
}

/// Parses a type as a declaration spells it.
fn parse_type(text: &str) -> Result<Ty, String> {
    let text = text.trim();

    if let Some(inner) = text.strip_prefix("any ") {
        return Ok(Ty::Existential(parse_protocol(inner)?));
    }
    if let Some(inner) = text.strip_suffix('?') {
        return Ok(Ty::Optional(Box::new(parse_type(inner)?)));
    }
    if let Some(inner) = text.strip_suffix(".Type") {
        return Ok(Ty::Metatype(Box::new(parse_type(inner)?)));
    }
    // The standard library's sugared generics.
    if let Some(inner) = text.strip_prefix('[').and_then(|t| t.strip_suffix(']')) {
        return Ok(match depth_zero_colon(inner) {
            Some(colon) => Ty::Dictionary(
                Box::new(parse_type(&inner[..colon])?),
                Box::new(parse_type(&inner[colon + 1..])?),
            ),
            None => Ty::Array(Box::new(parse_type(inner)?)),
        });
    }
    if let Some(inner) = text.strip_prefix("Set<").and_then(|t| t.strip_suffix('>')) {
        return Ok(Ty::Set(Box::new(parse_type(inner)?)));
    }
    if let Some(inner) = text
        .strip_prefix("Range<")
        .and_then(|t| t.strip_suffix('>'))
    {
        return Ok(Ty::Range(Box::new(parse_type(inner)?)));
    }

    Ok(match text {
        "Void" | "()" => Ty::Void,
        "Int" => Ty::Std('i'),
        "UInt" => Ty::Std('u'),
        "Bool" => Ty::Std('b'),
        "Double" => Ty::Std('d'),
        "Float" => Ty::Std('f'),
        "String" => Ty::Std('S'),
        // `Error` alone is the existential, as it is in Swift source.
        "Error" => Ty::Existential(parse_protocol("Swift.Error")?),
        "Int8" | "UInt8" | "Int16" | "UInt16" | "Int32" | "UInt32" | "Int64" | "UInt64" => {
            Ty::Nominal(Nominal {
                module: "Swift".to_string(),
                components: vec![Component {
                    name: text.to_string(),
                    kind: 'V',
                    args: Vec::new(),
                }],
            })
        }
        "TaskPriority" => Ty::Special("ScP"),
        "T" => Ty::Generic,
        _ => Ty::Nominal(parse_nominal(text, None)?),
    })
}

/// A protocol named by an existential, whose kind the spelling leaves out.
fn parse_protocol(text: &str) -> Result<Nominal, String> {
    let text = text.trim();
    let text = if text.contains('.') {
        text.to_string()
    } else {
        format!("Swift.{text}")
    };
    parse_nominal(&text, Some('P'))
}

/// Splits `Module.A(class).B(struct)<Args>` into its components.
///
/// `last_kind` is the kind of a final component written without one: the
/// metadata accessor's own type, or a protocol. A type imported from C is a
/// typealias unless it says otherwise, which is how Swift imports a C struct.
fn parse_nominal(text: &str, last_kind: Option<char>) -> Result<Nominal, String> {
    let parts = split_path(text);
    let mut iter = parts.into_iter();
    let module = iter.next().ok_or("empty type path")?;
    let rest: Vec<String> = iter.collect();
    if rest.is_empty() {
        return Err(format!("`{text}` is not a known type"));
    }

    let mut components = Vec::new();
    let last = rest.len() - 1;
    for (index, part) in rest.iter().enumerate() {
        let (head, args) = match part.find('<') {
            Some(open) if part.ends_with('>') => {
                let args = split_commas(&part[open + 1..part.len() - 1])
                    .iter()
                    .map(|arg| parse_type(arg))
                    .collect::<Result<Vec<_>, _>>()?;
                (&part[..open], args)
            }
            _ => (part.as_str(), Vec::new()),
        };
        let (name, kind) = match head.split_once('(') {
            Some((name, kind)) => (name, kind_letter(kind.trim_end_matches(')'))?),
            None if index == last && module == "__C" => (head, last_kind.unwrap_or('a')),
            None if index == last && last_kind.is_some() => (head, last_kind.unwrap()),
            None => return Err(format!("`{head}` needs a kind, as in `{head}(struct)`")),
        };
        components.push(Component {
            name: name.trim().to_string(),
            kind,
            args,
        });
    }
    Ok(Nominal { module, components })
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

/// A parameter of a declaration.
#[derive(Clone, Debug)]
struct Param {
    label: Option<String>,
    ty: Ty,
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
        // A parameter with no name at all is a bare type, as an enum case's
        // payload is written.
        let (label, ty) = match depth_zero_colon(&part) {
            Some(colon) => {
                let label = part[..colon].split_whitespace().next().unwrap_or("_");
                (label.to_string(), part[colon + 1..].trim().to_string())
            }
            None => ("_".to_string(), part.clone()),
        };
        let mut ty = ty.as_str();
        let mut ownership = None;
        for (keyword, letter) in [("__owned", 'n'), ("__shared", 'h'), ("inout", 'z')] {
            if let Some(rest) = ty.strip_prefix(keyword) {
                ownership = Some(letter);
                ty = rest.trim();
            }
        }
        params.push(Param {
            label: (label != "_").then_some(label),
            ty: parse_type(ty)?,
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

/// A run of back references the symbol ends with, which the next one may
/// merge into.
struct Run {
    start: usize,
    end: usize,
    standard: bool,
    /// Each reference with how many times in a row it repeats.
    items: Vec<(usize, char)>,
}

impl Run {
    fn render(&self) -> String {
        let mut out = String::from(if self.standard { "S" } else { "A" });
        for (index, (count, letter)) in self.items.iter().enumerate() {
            if *count > 1 {
                out.push_str(&count.to_string());
            }
            let last = index + 1 == self.items.len();
            out.push(if self.standard || last {
                *letter
            } else {
                letter.to_ascii_lowercase()
            });
        }
        out
    }
}

/// The compiler's mangler state for one symbol.
///
/// Two tables, both indexed in the order things are first spelled out:
/// `substs`, the identifiers and types a later occurrence refers back to as a
/// whole, and `words`, the pieces of identifiers a later identifier may reuse.
#[derive(Default)]
struct Mangler {
    out: String,
    words: Vec<String>,
    substs: Vec<String>,
    run: Option<Run>,
}

fn is_word_start(ch: Option<char>) -> bool {
    matches!(ch, Some(c) if !c.is_ascii_digit() && c != '_')
}

/// A word ends at an underscore, at the end, or where a capital follows
/// something that is not one — so `AVCaptureDevice` is `AVCapture` and
/// `Device`, not three words.
fn is_word_end(ch: Option<char>, previous: char) -> bool {
    match ch {
        None | Some('_') => true,
        Some(c) => !previous.is_ascii_uppercase() && c.is_ascii_uppercase(),
    }
}

/// The characters an operator's name is spelled with in a symbol.
fn translate_operator(name: &str) -> Option<String> {
    name.chars()
        .map(|c| {
            Some(match c {
                '&' => 'a',
                '@' => 'c',
                '/' => 'd',
                '=' => 'e',
                '>' => 'g',
                '<' => 'l',
                '*' => 'm',
                '!' => 'n',
                '|' => 'o',
                '+' => 'p',
                '?' => 'q',
                '%' => 'r',
                '-' => 's',
                '~' => 't',
                '^' => 'x',
                '.' => 'z',
                _ => return None,
            })
        })
        .collect()
}

impl Mangler {
    fn new() -> Self {
        Self {
            out: "$s".to_string(),
            ..Self::default()
        }
    }

    fn push(&mut self, text: &str) {
        self.out.push_str(text);
    }

    fn lookup(&self, key: &str) -> Option<usize> {
        self.substs.iter().position(|k| k == key)
    }

    /// Refers back to an entry of the substitution table.
    fn reference(&mut self, index: usize) {
        if index >= 26 {
            self.run = None;
            self.push("A");
            if index > 26 {
                self.push(&(index - 27).to_string());
            }
            self.push("_");
            return;
        }
        self.merge((b'A' + index as u8) as char, false);
    }

    /// One of the standard library's own substitutions, such as `Si`.
    fn standard(&mut self, letter: char) {
        self.merge(letter, true);
    }

    /// Appends a back reference, folding it into one that ends the symbol:
    /// `AB` then `C` is `AbC`, `AB` then `B` is `A2B`, and `Si` then `Si` is
    /// `S2i`. A standard substitution only merges with a repeat of itself.
    fn merge(&mut self, letter: char, standard: bool) {
        let out_len = self.out.len();
        if let Some(run) = self
            .run
            .as_mut()
            .filter(|run| run.end == out_len && run.standard == standard)
        {
            let (count, last) = run.items.last_mut().expect("a run is never empty");
            if *last == letter {
                *count += 1;
            } else if standard {
                self.run = None;
                return self.merge(letter, standard);
            } else {
                run.items.push((1, letter));
            }
            let start = run.start;
            let rendered = run.render();
            self.out.truncate(start);
            self.out.push_str(&rendered);
            run.end = self.out.len();
            return;
        }
        let run = Run {
            start: self.out.len(),
            end: 0,
            standard,
            items: vec![(1, letter)],
        };
        let rendered = run.render();
        self.out.push_str(&rendered);
        self.run = Some(Run {
            end: self.out.len(),
            ..run
        });
    }

    /// Appends an identifier, or a reference to it if the symbol has one.
    fn identifier(&mut self, ident: &str) {
        let key = format!("i:{ident}");
        if let Some(index) = self.lookup(&key) {
            return self.reference(index);
        }
        self.substs.push(key);
        let encoded = self.encode_words(ident);
        self.push(&encoded);
    }

    /// Spells an identifier out, reusing every word the symbol already has.
    ///
    /// The encoding is a `0` followed by chunks: a letter stands for a known
    /// word and a length-prefixed run for everything between them. The last
    /// letter is capitalized, and a `0` follows it when it ends the identifier.
    fn encode_words(&mut self, ident: &str) -> String {
        let chars: Vec<char> = ident.chars().collect();
        let mut replaced: Vec<(usize, usize)> = Vec::new();
        let mut start: Option<usize> = None;
        for pos in 0..=chars.len() {
            let ch = chars.get(pos).copied();
            if let Some(word_start) = start.filter(|_| is_word_end(ch, chars[pos - 1])) {
                let word: String = chars[word_start..pos].iter().collect();
                if let Some(index) = self.words.iter().position(|w| *w == word) {
                    replaced.push((word_start, index));
                } else if pos - word_start >= 2 && self.words.len() < 26 {
                    self.words.push(word);
                }
                start = None;
            }
            if start.is_none() && is_word_start(ch) {
                start = Some(pos);
            }
        }

        if replaced.is_empty() {
            return format!("{}{}", chars.len(), ident);
        }
        let mut out = String::from("0");
        let mut pos = 0;
        let last = replaced.len() - 1;
        for (index, (word_start, word)) in replaced.iter().enumerate() {
            if pos < *word_start {
                let literal: String = chars[pos..*word_start].iter().collect();
                out.push_str(&format!("{}{literal}", literal.len()));
            }
            pos = word_start + self.words[*word].chars().count();
            let letter = (b'a' + *word as u8) as char;
            if index == last {
                out.push(letter.to_ascii_uppercase());
                if pos == chars.len() {
                    out.push('0');
                }
            } else {
                out.push(letter);
            }
        }
        if pos < chars.len() {
            let literal: String = chars[pos..].iter().collect();
            out.push_str(&format!("{}{literal}", literal.len()));
        }
        out
    }

    fn module(&mut self, module: &str) {
        match module {
            "Swift" => self.push("s"),
            "__C" => self.push("So"),
            other => self.identifier(other),
        }
    }

    /// Appends the unbound declaration of the first `depth` components.
    fn decl(&mut self, nominal: &Nominal, depth: usize) {
        let key = nominal.decl_key(depth);
        if let Some(index) = self.lookup(&key) {
            return self.reference(index);
        }
        if depth == 1 {
            self.module(&nominal.module);
        } else {
            self.decl(nominal, depth - 1);
        }
        let component = &nominal.components[depth - 1];
        self.identifier(&component.name);
        // A protocol named in an existential carries no kind letter.
        if component.kind != 'P' {
            self.out.push(component.kind);
        }
        self.substs.push(key);
    }

    fn nominal(&mut self, nominal: &Nominal) -> Result<(), String> {
        if !nominal.is_bound() {
            self.decl(nominal, nominal.components.len());
            return Ok(());
        }
        let key = format!("t:{}", nominal.key());
        if let Some(index) = self.lookup(&key) {
            self.reference(index);
            return Ok(());
        }
        if nominal.components[..nominal.components.len() - 1]
            .iter()
            .any(|c| !c.args.is_empty())
        {
            return Err(format!(
                "`{}` binds an enclosing type's generic parameters, which is not supported",
                nominal.key()
            ));
        }
        self.decl(nominal, nominal.components.len());
        // One argument list per level of nesting, separated by `_`; only the
        // innermost level is bound here.
        self.push("y");
        for _ in 1..nominal.components.len() {
            self.push("_");
        }
        let args = nominal.components.last().expect("a path").args.clone();
        for arg in &args {
            self.ty(arg)?;
        }
        self.push("G");
        self.substs.push(key);
        Ok(())
    }

    fn ty(&mut self, ty: &Ty) -> Result<(), String> {
        let substituted = matches!(
            ty,
            Ty::Optional(_) | Ty::Array(_) | Ty::Dictionary(..) | Ty::Set(_) | Ty::Range(_)
        );
        let key = format!("t:{}", ty.key());
        if let Some(index) = self.lookup(&key).filter(|_| substituted) {
            self.reference(index);
            return Ok(());
        }
        match ty {
            Ty::Void => self.push("yt"),
            Ty::Std(letter) => self.standard(*letter),
            Ty::Generic => self.push("x"),
            Ty::Nominal(nominal) => self.nominal(nominal)?,
            Ty::Optional(inner) => {
                self.ty(inner)?;
                self.push("Sg");
            }
            Ty::Array(inner) => {
                self.standard('a');
                self.push("y");
                self.ty(inner)?;
                self.push("G");
            }
            Ty::Dictionary(key, value) => {
                self.standard('D');
                self.push("y");
                self.ty(key)?;
                self.ty(value)?;
                self.push("G");
            }
            Ty::Range(inner) => {
                self.standard('n');
                self.push("y");
                self.ty(inner)?;
                self.push("G");
            }
            Ty::Set(inner) => {
                self.standard('h');
                self.push("y");
                self.ty(inner)?;
                self.push("G");
            }
            Ty::Existential(protocol) => {
                self.decl(protocol, protocol.components.len());
                self.push("_p");
            }
            Ty::Metatype(inner) => {
                self.ty(inner)?;
                self.push("m");
            }
            Ty::Function(params, result) => {
                self.result(Some(result))?;
                self.params(params)?;
                self.push("c");
            }
            Ty::Special(text) => self.push(text),
        }
        if substituted {
            self.substs.push(key);
        }
        Ok(())
    }

    /// A function's result, where no result at all is the empty list.
    fn result(&mut self, result: Option<&Ty>) -> Result<(), String> {
        match result {
            None | Some(Ty::Void) => {
                self.push("y");
                Ok(())
            }
            Some(ty) => self.ty(ty),
        }
    }

    /// The parameter list, which is one type bare and several as a tuple.
    ///
    /// A lone unlabelled parameter is the argument type itself; anything else
    /// is a tuple, including a single parameter that carries a label. A tuple
    /// marks only its first element with `_`, whatever it goes on to hold, and
    /// closes with `t`.
    fn params(&mut self, params: &[Param]) -> Result<(), String> {
        if params.is_empty() {
            self.push("y");
            return Ok(());
        }
        let tuple = params.len() > 1 || params[0].label.is_some();
        for (index, param) in params.iter().enumerate() {
            self.ty(&param.ty)?;
            if let Some(letter) = param.ownership {
                self.out.push(letter);
            }
            if tuple && index == 0 {
                self.push("_");
            }
        }
        if tuple {
            self.push("t");
        }
        Ok(())
    }

    /// The argument labels: nothing without parameters, `y` when none has a
    /// label, and otherwise each label or `_` in turn.
    fn labels(&mut self, params: &[Param]) {
        if params.is_empty() {
            return;
        }
        if params.iter().all(|p| p.label.is_none()) {
            return self.push("y");
        }
        for param in params {
            match &param.label {
                Some(label) => self.identifier(label),
                None => self.push("_"),
            }
        }
    }

    /// A member's name, which for an operator is its translated spelling.
    fn decl_name(&mut self, name: &str) -> Result<(), String> {
        let is_operator = !name
            .chars()
            .next()
            .is_some_and(|c| c.is_alphabetic() || c == '_');
        if !is_operator {
            self.identifier(name);
            return Ok(());
        }
        let translated =
            translate_operator(name).ok_or_else(|| format!("`{name}` is not an operator"))?;
        self.identifier(&translated);
        // Every operator these bindings name is infix.
        self.push("oi");
        Ok(())
    }
}

/// Mangles the symbol of a type's metadata accessor.
///
/// `path` names the type as Swift spells it, with a kind on every component
/// but the last, whose kind is `kind`.
pub fn mangle_metadata_accessor(path: &str, kind: char) -> Result<String, String> {
    let nominal = parse_nominal(path.trim(), Some(kind))?;
    let mut m = Mangler::new();
    m.decl(&nominal, nominal.components.len());
    m.push("Ma");
    Ok(m.out)
}

/// Mangles the symbol of an enum case's descriptor, the constant a resilient
/// enum publishes its tag through.
///
/// `path` is the case the way Swift spells it, after its enum:
/// `DockKit.DockAccessory(class).State(enum).docked`, or with its payload,
/// `DockKit.DockAccessory(class).AccessoryEvent(enum).button(Int, Bool)`.
pub fn mangle_enum_case(decl: &str) -> Result<String, String> {
    let decl = decl.trim();
    let open = find_param_paren(decl);
    let (path, payload) = if open < decl.len() {
        let payload = decl[open + 1..]
            .strip_suffix(')')
            .ok_or_else(|| format!("`{decl}` has an unclosed payload"))?;
        (&decl[..open], Some(parse_params(payload)?))
    } else {
        (decl, None)
    };
    let parts = split_path(path);
    let (name, context) = parts
        .split_last()
        .ok_or_else(|| format!("`{decl}` names no case"))?;
    let nominal = parse_nominal(&context.join("."), None)?;
    let enum_ty = Ty::Nominal(nominal.clone());

    let mut m = Mangler::new();
    m.decl(&nominal, nominal.components.len());
    m.identifier(name);
    // The case is a function of the enum's metatype, unlabelled.
    m.push("y");
    match payload {
        Some(params) => m.ty(&Ty::Function(params, Box::new(enum_ty.clone())))?,
        None => m.ty(&enum_ty)?,
    }
    m.ty(&Ty::Metatype(Box::new(enum_ty)))?;
    m.push("FWC");
    Ok(m.out)
}

/// Mangles one of a protocol conformance's symbols: `Mc` for its descriptor,
/// `WP` for the witness table a conformance that needs no instantiation
/// publishes.
///
/// `decl` is written `Type: Protocol`, and the conformance is taken to be
/// declared in the type's own module, which is where the bindings' ones are.
pub fn mangle_conformance(decl: &str, suffix: &str) -> Result<String, String> {
    let (ty, protocol) = decl
        .split_once(':')
        .ok_or_else(|| format!("`{decl}` is not written `Type: Protocol`"))?;
    let ty = parse_type(ty)?;
    let module = match &ty {
        Ty::Nominal(nominal) => nominal.module.clone(),
        _ => "Swift".to_string(),
    };

    let mut m = Mangler::new();
    m.ty(&ty)?;
    // The standard library's own protocols have standard substitutions.
    match protocol.trim() {
        "Hashable" | "Swift.Hashable" => m.standard('H'),
        "Equatable" | "Swift.Equatable" => m.standard('Q'),
        other => {
            let protocol = parse_protocol(other)?;
            m.decl(&protocol, protocol.components.len());
        }
    }
    m.module(&module);
    m.push(suffix);
    Ok(m.out)
}

/// Whether the callee takes ownership of each parameter, in declaration order.
///
/// Swift's defaults differ by member: an initializer's parameters arrive at
/// `+1` unless marked `__shared`, while a method borrows unless marked
/// `__owned`. A caller that gets this backwards either leaks the value or
/// releases one it never owned, and neither shows up as a compile error, so it
/// is read off the declaration rather than assumed.
pub fn param_conventions(decl: &str) -> Result<Vec<bool>, String> {
    let (_, member, ..) = parse_decl(decl)?;
    Ok(match member {
        Member::Property { .. } => Vec::new(),
        Member::Init { params, .. } => params.iter().map(|p| p.ownership != Some('h')).collect(),
        Member::Method { params, .. } => params.iter().map(|p| p.ownership == Some('n')).collect(),
    })
}

/// Whether the declaration names a suspending function.
///
/// What tells `#[swift::call]` to await the call on a Swift task rather than
/// make it directly, since a suspending function cannot be called at all
/// without the caller allocating its context first.
pub fn is_async(decl: &str) -> Result<bool, String> {
    Ok(match parse_decl(decl)?.1 {
        Member::Property { .. } => false,
        Member::Init { is_async, .. } | Member::Method { is_async, .. } => is_async,
    })
}

/// Strips the `static` and `thunk` markers and splits the rest.
fn parse_decl(decl: &str) -> Result<(String, Member, bool, bool), String> {
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
    let (head, member) = split_member(decl)?;
    Ok((head, member, is_static, thunk))
}

/// Mangles a Swift declaration into its exported symbol.
///
/// A suspending function has a second symbol beside this one: its async
/// function pointer, the record holding the entry point and the size of the
/// context a caller has to allocate. That symbol is this one with `Tu`
/// appended — after the `Tj` of a dispatch thunk, since what a caller awaits
/// through a thunk is the thunk.
pub fn mangle(decl: &str) -> Result<String, String> {
    let (head, member, is_static, thunk) = parse_decl(decl)?;
    let context = parse_nominal(&head, None)?;

    let mut m = Mangler::new();
    m.decl(&context, context.components.len());

    match member {
        Member::Property { name, ty, accessor } => {
            m.decl_name(&name)?;
            m.ty(&parse_type(&ty)?)?;
            m.push(match accessor.as_str() {
                "get" => "vg",
                "set" => "vs",
                other => return Err(format!("unknown accessor `{other}`")),
            });
        }
        Member::Init {
            params,
            failable,
            is_async,
            throws,
        } => {
            // An initializer's result is the enclosing type, which the symbol
            // has just spelled, so it comes out as a back reference.
            m.labels(&params);
            let own = Ty::Nominal(context.clone());
            let result = if failable {
                Ty::Optional(Box::new(own))
            } else {
                own
            };
            m.ty(&result)?;
            m.params(&params)?;
            m.push(if is_async { "Ya" } else { "" });
            m.push(if throws { "K" } else { "" });
            m.push("cfC");
        }
        Member::Method {
            name,
            params,
            result,
            is_async,
            throws,
        } => {
            m.decl_name(&name)?;
            m.labels(&params);
            let result = result.as_deref().map(parse_type).transpose()?;
            m.result(result.as_ref())?;
            m.params(&params)?;
            m.push(if is_async { "Ya" } else { "" });
            m.push(if throws { "K" } else { "" });
            m.push("F");
        }
    }

    if is_static {
        m.push("Z");
    }
    if thunk {
        m.push("Tj");
    }
    Ok(m.out)
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

    let open = find_param_paren(before_result);
    if open == before_result.len() {
        return Err(format!("`{decl}` is neither a property nor a function"));
    }

    let path = before_result[..open].trim();
    let params_text = before_result[open + 1..]
        .trim()
        .strip_suffix(')')
        .ok_or_else(|| format!("`{decl}` has an unclosed parameter list"))?
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

/// The offset of the first colon at depth zero, which separates a name from
/// its type, or a dictionary's key from its value.
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
/// markers attached to type names and anything inside a generic argument
/// list. Returns the text's length when there is none.
fn find_param_paren(text: &str) -> usize {
    let mut angle = 0i32;
    let mut index = 0usize;
    while index < text.len() {
        match text.as_bytes()[index] {
            b'<' => angle += 1,
            b'>' => angle -= 1,
            b'(' if angle == 0 => {
                let close = text[index..].find(')').map(|offset| index + offset);
                let is_kind = close.is_some_and(|close| {
                    matches!(
                        &text[index + 1..close],
                        "struct" | "class" | "enum" | "protocol"
                    )
                });
                if !is_kind {
                    return index;
                }
                index = close.unwrap();
            }
            _ => {}
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
        let decl = "DockKit.DockAccessory(class).selectSubjects(_: [Foundation.UUID(struct)]) async throws";
        assert_eq!(
            "$s7DockKit0A9AccessoryC14selectSubjectsyySay10Foundation4UUIDVGYaKFTu",
            format!("{}Tu", mangle(decl).unwrap())
        );

        // Through a dispatch thunk the pointer is the thunk's, not the
        // function's, so `Tu` goes after `Tj` rather than before it.
        let thunked = "DockKit.DockAccessoryManager(class).setSystemTrackingEnabled(_: Bool) async throws thunk";
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
            (
                "Foundation.Notification",
                'V',
                "$s10Foundation12NotificationVMa",
            ),
            (
                "Foundation.AttributedString(struct).CharacterView",
                'V',
                "$s10Foundation16AttributedStringV13CharacterViewVMa",
            ),
            (
                "Foundation.AttributedString",
                'V',
                "$s10Foundation16AttributedStringVMa",
            ),
            ("Foundation.Date", 'V', "$s10Foundation4DateVMa"),
            ("Foundation.UUID", 'V', "$s10Foundation4UUIDVMa"),
            ("Foundation.Locale", 'V', "$s10Foundation6LocaleVMa"),
            (
                "MusicUnderstanding.MusicUnderstandingSession(class).SessionResult",
                'V',
                "$s18MusicUnderstanding0aB7SessionC0C6ResultVMa",
            ),
            (
                "MusicUnderstanding.MusicUnderstandingSession",
                'C',
                "$s18MusicUnderstanding0aB7SessionCMa",
            ),
            (
                "MusicUnderstanding.RhythmResult",
                'V',
                "$s18MusicUnderstanding12RhythmResultVMa",
            ),
            (
                "MusicUnderstanding.LoudnessResult",
                'V',
                "$s18MusicUnderstanding14LoudnessResultVMa",
            ),
            (
                "MusicUnderstanding.InstrumentActivityResult",
                'V',
                "$s18MusicUnderstanding24InstrumentActivityResultVMa",
            ),
            (
                "Speech.SpeechTranscriber(class).Result",
                'V',
                "$s6Speech0A11TranscriberC6ResultVMa",
            ),
            (
                "Speech.SpeechTranscriber",
                'C',
                "$s6Speech0A11TranscriberCMa",
            ),
            (
                "Speech.SpeechAnalyzer(class).Options",
                'V',
                "$s6Speech0A8AnalyzerC7OptionsVMa",
            ),
            ("Speech.SpeechAnalyzer", 'C', "$s6Speech0A8AnalyzerCMa"),
            (
                "Speech.SpeechDetector(class).DetectionOptions",
                'V',
                "$s6Speech0A8DetectorC16DetectionOptionsVMa",
            ),
            (
                "Speech.SpeechDetector(class).SensitivityLevel",
                'O',
                "$s6Speech0A8DetectorC16SensitivityLevelOMa",
            ),
            ("Speech.SpeechDetector", 'C', "$s6Speech0A8DetectorCMa"),
            (
                "Speech.DictationTranscriber(class).Result",
                'V',
                "$s6Speech20DictationTranscriberC6ResultVMa",
            ),
            (
                "Speech.DictationTranscriber",
                'C',
                "$s6Speech20DictationTranscriberCMa",
            ),
            (
                "Speech.CaptureInputSequenceProvider",
                'C',
                "$s6Speech28CaptureInputSequenceProviderCMa",
            ),
            (
                "DockKit.DockAccessoryManager",
                'C',
                "$s7DockKit0A16AccessoryManagerCMa",
            ),
            (
                "DockKit.DockAccessory(class).AccessoryEvent",
                'O',
                "$s7DockKit0A9AccessoryC0C5EventOMa",
            ),
            (
                "DockKit.DockAccessory(class).Identifier",
                'V',
                "$s7DockKit0A9AccessoryC10IdentifierVMa",
            ),
            (
                "DockKit.DockAccessory(class).MotionState",
                'V',
                "$s7DockKit0A9AccessoryC11MotionStateVMa",
            ),
            (
                "DockKit.DockAccessory(class).Observation",
                'V',
                "$s7DockKit0A9AccessoryC11ObservationVMa",
            ),
            (
                "DockKit.DockAccessory(class).StateChange",
                'V',
                "$s7DockKit0A9AccessoryC11StateChangeVMa",
            ),
            (
                "DockKit.DockAccessory(class).BatteryState",
                'V',
                "$s7DockKit0A9AccessoryC12BatteryStateVMa",
            ),
            (
                "DockKit.DockAccessory(class).TrackedObject",
                'V',
                "$s7DockKit0A9AccessoryC13TrackedObjectVMa",
            ),
            (
                "DockKit.DockAccessory(class).TrackedPerson",
                'V',
                "$s7DockKit0A9AccessoryC13TrackedPersonVMa",
            ),
            (
                "DockKit.DockAccessory(class).TrackingState",
                'V',
                "$s7DockKit0A9AccessoryC13TrackingStateVMa",
            ),
            (
                "DockKit.DockAccessory(class).CameraInformation",
                'V',
                "$s7DockKit0A9AccessoryC17CameraInformationVMa",
            ),
            (
                "DockKit.DockAccessory(class).TrackedSubjectType",
                'O',
                "$s7DockKit0A9AccessoryC18TrackedSubjectTypeOMa",
            ),
            (
                "DockKit.DockAccessory(class).Limits(struct).Limit",
                'V',
                "$s7DockKit0A9AccessoryC6LimitsV5LimitVMa",
            ),
            (
                "DockKit.DockAccessory(class).Limits",
                'V',
                "$s7DockKit0A9AccessoryC6LimitsVMa",
            ),
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
        let params = |text: &str| {
            let mut m = Mangler::default();
            m.params(&parse_params(text).unwrap()).unwrap();
            m.out
        };
        assert_eq!(
            "Si_SdSbSSt",
            params("a: Int, b: Double, c: Bool, d: String")
        );
        assert_eq!("Si_t", params("a: Int"));
        // Without a label there is no tuple at all.
        assert_eq!("Si", params("_ a: Int"));
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

    /// Declarations whose symbols refer back to what they already spelled —
    /// the enclosing type, its module, a label, an optional — measured against
    /// the symbols the frameworks export.
    #[test]
    fn mangles_back_references() {
        let cases = [
            (
                "DockKit.DockAccessory(class).limits: DockKit.DockAccessory(class).Limits(struct) { get }",
                "$s7DockKit0A9AccessoryC6limitsAC6LimitsVvg",
            ),
            (
                "DockKit.DockAccessory(class).Limits(struct).init(yaw: DockKit.DockAccessory(class).Limits(struct).Limit(struct)?, pitch: DockKit.DockAccessory(class).Limits(struct).Limit(struct)?, roll: DockKit.DockAccessory(class).Limits(struct).Limit(struct)?)",
                "$s7DockKit0A9AccessoryC6LimitsV3yaw5pitch4rollA2E5LimitVSg_A2KtcfC",
            ),
            (
                "Speech.SpeechTranscriber(class).init(locale: Foundation.Locale(struct), preset: Speech.SpeechTranscriber(class).Preset(struct))",
                "$s6Speech0A11TranscriberC6locale6presetAC10Foundation6LocaleV_AC6PresetVtcfC",
            ),
            (
                "static DockKit.DockAccessory(class).Identifier(struct).==(_: DockKit.DockAccessory(class).Identifier(struct), _: DockKit.DockAccessory(class).Identifier(struct)) -> Bool",
                "$s7DockKit0A9AccessoryC10IdentifierV2eeoiySbAE_AEtFZ",
            ),
            (
                "MusicUnderstanding.LoudnessResult(struct).integrated: MusicUnderstanding.MusicUnderstandingSession(class).TimedValue(struct)<Float> { get }",
                "$s18MusicUnderstanding14LoudnessResultV10integratedAA0aB7SessionC10TimedValueVy_SfGvg",
            ),
            (
                "MusicUnderstanding.MusicUnderstandingSession(class).SessionResult(struct).rhythm: MusicUnderstanding.RhythmResult(struct)? { get }",
                "$s18MusicUnderstanding0aB7SessionC0C6ResultV6rhythmAA06RhythmD0VSgvg",
            ),
            (
                "MusicUnderstanding.InstrumentActivityResult(struct).ranges: [MusicUnderstanding.InstrumentActivityResult(struct).Instrument(struct): [__C.CMTimeRange]] { get }",
                "$s18MusicUnderstanding24InstrumentActivityResultV6rangesSDyAC0C0VSaySo11CMTimeRangeaGGvg",
            ),
            (
                "static MusicUnderstanding.InstrumentActivityResult(struct).Instrument(struct).bass: MusicUnderstanding.InstrumentActivityResult(struct).Instrument(struct) { get }",
                "$s18MusicUnderstanding24InstrumentActivityResultV0C0V4bassAEvgZ",
            ),
            (
                "static Speech.DictationTranscriber(class).Preset(struct).multisegmentDictationCC: Speech.DictationTranscriber(class).Preset(struct) { get }",
                "$s6Speech20DictationTranscriberC6PresetV012multisegmentB2CCAEvgZ",
            ),
            (
                "DockKit.DockAccessory(class).Observation(struct).init(identifier: Int, type: DockKit.DockAccessory(class).Observation(struct).ObservationType(enum), rect: __C.CGRect(struct), faceYawAngle: Foundation.Measurement(struct)<__C.NSUnitAngle(class)>?)",
                "$s7DockKit0A9AccessoryC11ObservationV10identifier4type4rect12faceYawAngleAESi_AE0D4TypeOSo6CGRectV10Foundation11MeasurementVySo06NSUnitJ0CGSgtcfC",
            ),
            (
                "DockKit.DockAccessory(class).CameraInformation(struct).init(captureDevice: __C.AVCaptureDeviceType, cameraPosition: __C.AVCaptureDevicePosition(struct), orientation: DockKit.DockAccessory(class).CameraOrientation(enum), cameraIntrinsics: __C.simd_float3x3?, referenceDimensions: __C.CGSize(struct)?)",
                "$s7DockKit0A9AccessoryC17CameraInformationV13captureDevice14cameraPosition11orientation0H10Intrinsics19referenceDimensionsAESo09AVCaptureG4Typea_So0ngI0VAC0D11OrientationOSo13simd_float3x3aSgSo6CGSizeVSgtcfC",
            ),
            (
                "DockKit.DockAccessory(class).setOrientation(_: __C.SPVector3D, duration: Swift.Duration(struct), relative: Bool) throws -> __C.NSProgress(class)",
                "$s7DockKit0A9AccessoryC14setOrientation_8duration8relativeSo10NSProgressCSo10SPVector3Da_s8DurationVSbtKF",
            ),
            (
                "DockKit.DockAccessory(class).setFramingMode(_: DockKit.DockAccessory(class).FramingMode(enum)) async throws",
                "$s7DockKit0A9AccessoryC14setFramingModeyyAC0eF0OYaKF",
            ),
            (
                "DockKit.DockAccessory(class).BatteryState(struct).chargeState: DockKit.DockAccessory(class).BatteryChargeState(enum) { get }",
                "$s7DockKit0A9AccessoryC12BatteryStateV06chargeE0AC0d6ChargeE0Ovg",
            ),
            (
                "DockKit.DockAccessory(class).MotionState(struct).error: Error? { get }",
                "$s7DockKit0A9AccessoryC11MotionStateV5errors5Error_pSgvg",
            ),
            (
                "DockKit.DockAccessory(class).StateChange(struct).accessory: DockKit.DockAccessory(class)? { get }",
                "$s7DockKit0A9AccessoryC11StateChangeV9accessoryACSgvg",
            ),
            (
                "DockKit.DockAccessory(class).TrackingState(struct).trackedSubjects: [DockKit.DockAccessory(class).TrackedSubjectType(enum)] { get }",
                "$s7DockKit0A9AccessoryC13TrackingStateV15trackedSubjectsSayAC18TrackedSubjectTypeOGvg",
            ),
            (
                "DockKit.DockAccessory(class).StateChanges(struct).makeAsyncIterator() -> DockKit.DockAccessory(class).StateChanges(struct).Iterator(struct)",
                "$s7DockKit0A9AccessoryC12StateChangesV17makeAsyncIteratorAE0H0VyF",
            ),
            (
                "DockKit.DockAccessory(class).StateChanges(struct).Iterator(struct).next() async -> DockKit.DockAccessory(class).StateChange(struct)?",
                "$s7DockKit0A9AccessoryC12StateChangesV8IteratorV4nextAC0D6ChangeVSgyYaF",
            ),
            (
                "MusicUnderstanding.MusicUnderstandingSession(class).analyze(for: Set<MusicUnderstanding.AnalysisType(struct)>) async throws -> MusicUnderstanding.MusicUnderstandingSession(class).SessionResult(struct) thunk",
                "$s18MusicUnderstanding0aB7SessionC7analyze3forAC0C6ResultVShyAA12AnalysisTypeVG_tYaKFTj",
            ),
            (
                "Speech.SpeechDetector(class).init(detectionOptions: Speech.SpeechDetector(class).DetectionOptions(struct), reportResults: Bool)",
                "$s6Speech0A8DetectorC16detectionOptions13reportResultsA2C09DetectionD0V_SbtcfC",
            ),
            (
                "Speech.SpeechAnalyzer(class).init(modules: [any Speech.SpeechModule], options: Speech.SpeechAnalyzer(class).Options(struct)?)",
                "$s6Speech0A8AnalyzerC7modules7optionsACSayAA0A6Module_pG_AC7OptionsVSgtcfC",
            ),
            (
                "DockKit.DockAccessory(class).Limits(struct).Limit(struct).init(positionRange: Range<Double>, maximumSpeed: Double) throws",
                "$s7DockKit0A9AccessoryC6LimitsV5LimitV13positionRange12maximumSpeedAGSnySdG_SdtKcfC",
            ),
            (
                "DockKit.DockAccessory(class).Limits(struct).Limit(struct).positionRange: Range<Double> { get }",
                "$s7DockKit0A9AccessoryC6LimitsV5LimitV13positionRangeSnySdGvg",
            ),
            (
                "static Swift.Duration(struct).seconds(_: Double) -> Swift.Duration(struct)",
                "$ss8DurationV7secondsyABSdFZ",
            ),
            (
                "DockKit.DockAccessory(class).setLimits(_: DockKit.DockAccessory(class).Limits(struct)) throws",
                "$s7DockKit0A9AccessoryC9setLimitsyyAC0E0VKF",
            ),
            (
                "static Speech.CaptureInputSequenceProvider(class).providerWithSession(from: __C.AVCaptureDevice(class), compatibleWith: [any Speech.SpeechModule], priority: TaskPriority?) async throws -> Speech.CaptureInputSequenceProvider(class)",
                "$s6Speech28CaptureInputSequenceProviderC19providerWithSession4from010compatibleG08priorityACSo15AVCaptureDeviceC_SayAA0A6Module_pGScPSgtYaKFZ",
            ),
            (
                "Speech.CaptureInputSequenceProvider(class).captureSession: __C.AVCaptureSession(class) { get }",
                "$s6Speech28CaptureInputSequenceProviderC14captureSessionSo09AVCaptureG0Cvg",
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

    /// Enum case descriptors, the constants a resilient enum's tags are read
    /// from.
    #[test]
    fn mangles_enum_cases() {
        let cases = [
            (
                "DockKit.DockAccessory(class).State(enum).docked",
                "$s7DockKit0A9AccessoryC5StateO6dockedyA2EmFWC",
            ),
            (
                "DockKit.DockAccessory(class).Observation(struct).ObservationType(enum).humanFace",
                "$s7DockKit0A9AccessoryC11ObservationV0D4TypeO9humanFaceyA2GmFWC",
            ),
            (
                "DockKit.DockAccessory(class).AccessoryEvent(enum).button(Int, Bool)",
                "$s7DockKit0A9AccessoryC0C5EventO6buttonyAESi_SbtcAEmFWC",
            ),
            (
                "DockKit.DockAccessory(class).AccessoryEvent(enum).cameraZoom(factor: Double)",
                "$s7DockKit0A9AccessoryC0C5EventO10cameraZoomyAESd_tcAEmFWC",
            ),
            (
                "DockKit.DockAccessory(class).TrackedSubjectType(enum).person(DockKit.DockAccessory(class).TrackedPerson(struct))",
                "$s7DockKit0A9AccessoryC18TrackedSubjectTypeO6personyAeC0D6PersonVcAEmFWC",
            ),
        ];
        for (decl, expected) in cases {
            assert_eq!(Ok(expected.to_string()), mangle_enum_case(decl), "{decl}");
        }
    }

    /// Conformance descriptors and witness tables, which the bindings hand to
    /// the runtime rather than call.
    #[test]
    fn mangles_conformances() {
        let cases = [
            (
                "Speech.SpeechDetector(class): Speech.SpeechModule",
                "WP",
                "$s6Speech0A8DetectorCAA0A6ModuleAAWP",
            ),
            (
                "MusicUnderstanding.AnalysisType(struct): Hashable",
                "Mc",
                "$s18MusicUnderstanding12AnalysisTypeVSHAAMc",
            ),
            (
                "MusicUnderstanding.InstrumentActivityResult(struct).Instrument(struct): Hashable",
                "Mc",
                "$s18MusicUnderstanding24InstrumentActivityResultV0C0VSHAAMc",
            ),
            ("Int: Hashable", "Mc", "$sSiSHsMc"),
            ("String: Hashable", "Mc", "$sSSSHsMc"),
        ];
        for (decl, suffix, expected) in cases {
            assert_eq!(
                Ok(expected.to_string()),
                mangle_conformance(decl, suffix),
                "{decl}"
            );
        }
    }

    /// Words split only where a capital follows a lowercase letter, so a run
    /// of capitals stays in the word it starts.
    #[test]
    fn a_run_of_capitals_is_one_word() {
        let mut m = Mangler::default();
        m.identifier("AVCaptureDeviceType");
        assert_eq!(vec!["AVCapture", "Device", "Type"], m.words);
    }
}
