//! Checks every Swift symbol the bindings link against the running system.
//!
//! Run with `cargo test -p cidre symbol_audit`.
//!
//! A binding names a Swift entry point either as a declaration that
//! `#[swift::call]` mangles or as a mangled symbol written out by hand. Either
//! one can be wrong and still compile: a bad symbol only fails when some binary
//! links that particular binding, so most never get tried. This scans the
//! sources for every symbol the bindings name and asks the dynamic linker for
//! each.
//!
//! Because Swift mangles a parameter's convention into the symbol, a
//! declaration that says `__shared` where the real function takes its argument
//! at `+1` (or the reverse) names a symbol that does not exist, so this also
//! checks the ownership claims `#[swift::call]` generates its calls from.
//!
//! The sources are scanned for:
//!
//! - `#[swift::call("declaration")]` and `swift::symbol!("declaration")`,
//!   mangled the way the macros mangle them, plus the async function pointer
//!   beside a suspending one;
//! - `metadata_accessor!(kind, "Module.Type")`, the `#[swift::class]`,
//!   `#[swift::struct]` and `#[swift::enum]` forms of `define_swift!`, and
//!   `define_swift_getter_enum!`'s `= swift "Module.Type"`, as metadata
//!   accessors;
//! - every other `"$s..."` string literal, which is how a hand-mangled symbol
//!   is written wherever it appears.
//!
//! A symbol is skipped when this system cannot have it: its framework does not
//! load here (another platform's), or every binding that names it needs a
//! newer OS than the one running the test — by a `#[cfg(feature =
//! "macos_27_0")]` on it or on a module above it, or an
//! `#[api::available(macos = 27.0)]`. So run it on each platform, and on the
//! newest OS to cover everything. Set `CIDRE_AUDIT_VERBOSE=1` to list what was
//! skipped, and `CIDRE_AUDIT_OS=26.0` to audit as if running an older OS.

use std::{
    collections::BTreeMap,
    ffi::{CString, c_char, c_int, c_void},
    fs,
    path::{Path, PathBuf},
};

#[allow(dead_code)]
#[path = "../../../cidre-macros/src/swift_mangle.rs"]
mod swift_mangle;

const RTLD_LAZY: c_int = 0x1;
const RTLD_GLOBAL: c_int = 0x8;
const RTLD_DEFAULT: *const c_void = -2isize as *const c_void;

unsafe extern "C" {
    fn dlopen(path: *const c_char, mode: c_int) -> *mut c_void;
    fn dlsym(handle: *const c_void, symbol: *const c_char) -> *mut c_void;
}

/// Where a symbol was named.
#[derive(Debug)]
struct Site {
    file: PathBuf,
    line: usize,
    /// The declaration it was mangled from, if it was not written mangled.
    decl: Option<String>,
    /// The oldest version of this OS the binding naming it can be built for.
    requires: Version,
}

impl std::fmt::Display for Site {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.file.display(), self.line)?;
        if let Some(decl) = &self.decl {
            write!(f, " `{decl}`")?;
        }
        Ok(())
    }
}

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// The source with comment lines blanked, so documentation that spells out an
/// example symbol is not taken for a binding. Lines keep their numbers.
fn code_only(text: &str) -> String {
    text.lines()
        .map(|line| {
            if line.trim_start().starts_with("//") {
                ""
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// The 1-based line `offset` falls on.
fn line_of(text: &str, offset: usize) -> usize {
    text[..offset].matches('\n').count() + 1
}

/// Resolves a string literal's source text — the part between the quotes — to
/// the string it denotes, as the macros do.
fn unescape(body: &str) -> String {
    let mut out = String::with_capacity(body.len());
    let mut chars = body.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }
        match chars.next() {
            // A line continuation eats the newline and the indentation after it.
            Some('\n') => {
                while chars.peek().is_some_and(|c| c.is_whitespace()) {
                    chars.next();
                }
            }
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some(other) => out.push(other),
            None => break,
        }
    }
    out
}

/// A literal starting at `index`: its body's range, and where scanning
/// resumes. Char literals are recognized only so that `'"'` does not open a
/// string.
fn literal_at(bytes: &[u8], index: usize) -> Option<(Option<(usize, usize)>, usize)> {
    match bytes[index] {
        b'\'' => {
            // `'"'`, `'\''`, `'\\'` and the like; anything else is a lifetime.
            if bytes.get(index + 1) == Some(&b'\\') && bytes.get(index + 3) == Some(&b'\'') {
                Some((None, index + 4))
            } else if bytes.get(index + 2) == Some(&b'\'') {
                Some((None, index + 3))
            } else {
                None
            }
        }
        b'r' if matches!(bytes.get(index + 1), Some(b'"' | b'#'))
            && (index == 0
                || !(bytes[index - 1].is_ascii_alphanumeric() || bytes[index - 1] == b'_')) =>
        {
            let hashes = bytes[index + 1..]
                .iter()
                .take_while(|&&b| b == b'#')
                .count();
            let start = index + 1 + hashes;
            if bytes.get(start) != Some(&b'"') {
                return None;
            }
            let closing: Vec<u8> = std::iter::once(b'"')
                .chain(std::iter::repeat_n(b'#', hashes))
                .collect();
            let end = bytes[start + 1..]
                .windows(closing.len())
                .position(|w| w == closing.as_slice())
                .map_or(bytes.len(), |p| start + 1 + p);
            Some((Some((start + 1, end)), end + closing.len()))
        }
        b'"' => {
            let mut end = index + 1;
            while end < bytes.len() && bytes[end] != b'"' {
                if bytes[end] == b'\\' {
                    end += 1;
                }
                end += 1;
            }
            let end = end.min(bytes.len());
            Some((Some((index + 1, end)), end + 1))
        }
        _ => None,
    }
}

/// The text inside the parentheses or braces that open at `open`, or `None`
/// if they never close. Literals are skipped, so a parenthesis inside a
/// declaration does not end it.
fn group(text: &str, open: usize) -> Option<&str> {
    let bytes = text.as_bytes();
    let (opening, closing) = match bytes[open] {
        b'(' => (b'(', b')'),
        b'{' => (b'{', b'}'),
        other => panic!("`{}` opens no group", other as char),
    };
    let mut depth = 0usize;
    let mut index = open;
    while index < bytes.len() {
        if let Some((_, next)) = literal_at(bytes, index) {
            index = next;
            continue;
        }
        match bytes[index] {
            b if b == opening => depth += 1,
            b if b == closing => {
                depth -= 1;
                if depth == 0 {
                    return Some(&text[open + 1..index]);
                }
            }
            _ => {}
        }
        index += 1;
    }
    None
}

/// Every string literal in `text`, unescaped, with its offset.
fn literals(text: &str) -> Vec<(usize, String)> {
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        match literal_at(bytes, index) {
            Some((Some((start, end)), next)) => {
                let raw = bytes[index] == b'r';
                let body = &text[start..end];
                out.push((
                    index,
                    if raw {
                        body.to_string()
                    } else {
                        unescape(body)
                    },
                ));
                index = next;
            }
            Some((None, next)) => index = next,
            None => index += 1,
        }
    }
    out
}

/// The declaration a macro's arguments spell, joined from all their string
/// literals, or `None` for a `macro_rules!` body passing its own fragments
/// through — whose invocations are what gets checked instead.
fn joined(args: &str) -> Option<String> {
    if args.contains('$') {
        return None;
    }
    let pieces = literals(args);
    (!pieces.is_empty()).then(|| pieces.into_iter().map(|(_, piece)| piece).collect())
}

/// The `name = "literal"` pairs in a macro invocation's body, one per line.
fn assignments(body: &str) -> Vec<(String, String)> {
    body.lines()
        .filter_map(|line| {
            let (name, value) = line.trim().split_once(" = ")?;
            let name = name.trim();
            if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                return None;
            }
            let value = literals(value).into_iter().next()?.1;
            Some((name.to_string(), value))
        })
        .collect()
}

/// The literal following `key` in `body`.
fn value_after(body: &str, key: &str) -> Option<String> {
    let at = body.find(key)? + key.len();
    literals(&body[at..])
        .into_iter()
        .next()
        .map(|(_, value)| value)
}

/// What a symbol was derived from.
enum Source<'a> {
    Decl(&'a str),
    AsyncDecl(&'a str),
    EnumCase(&'a str),
    Accessor(&'a str, &'a str),
    /// Written out already.
    Mangled(String),
}

/// An OS version, as `(major, minor)`.
type Version = (u32, u32);

/// The version of the OS running the test, or the one `CIDRE_AUDIT_OS` names,
/// which is how the skipping is checked for an older OS without one.
fn running_os() -> Version {
    if let Some(text) = std::env::var("CIDRE_AUDIT_OS")
        .ok()
        .filter(|text| !text.is_empty())
    {
        let mut parts = text.split('.').map(|part| part.parse::<u32>().unwrap_or(0));
        return (parts.next().unwrap_or(0), parts.next().unwrap_or(0));
    }
    unsafe extern "C" {
        fn sysctlbyname(
            name: *const c_char,
            old: *mut c_void,
            old_len: *mut usize,
            new: *const c_void,
            new_len: usize,
        ) -> c_int;
    }
    let mut buf = [0u8; 32];
    let mut len = buf.len();
    let name = CString::new("kern.osproductversion").unwrap();
    let status = unsafe {
        sysctlbyname(
            name.as_ptr(),
            buf.as_mut_ptr().cast(),
            &mut len,
            core::ptr::null(),
            0,
        )
    };
    assert_eq!(0, status, "kern.osproductversion must be readable");
    let text = std::str::from_utf8(&buf[..len])
        .unwrap()
        .trim_end_matches('\0');
    let mut parts = text.split('.').map(|part| part.parse::<u32>().unwrap_or(0));
    (parts.next().unwrap_or(0), parts.next().unwrap_or(0))
}

/// The name this OS goes by in feature flags and availability attributes.
fn os_key() -> &'static str {
    match std::env::consts::OS {
        "macos" => "macos",
        "ios" => "ios",
        "tvos" => "tvos",
        "watchos" => "watchos",
        "visionos" => "visionos",
        other => other,
    }
}

/// The newest version of this OS that attributes in `text` require: a
/// `feature = "macos_27_0"` in a `cfg`, or `macos = 27.0` in an `available`.
/// Several conditions on one item all have to hold, so the newest wins.
fn required_version(text: &str) -> Version {
    let key = os_key();
    let mut version = (0, 0);
    let feature = format!("feature = \"{key}_");
    for (at, _) in text.match_indices(&feature) {
        let rest = &text[at + feature.len()..];
        let digits: String = rest
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '_')
            .collect();
        let mut parts = digits.split('_').map(|part| part.parse().unwrap_or(0));
        version = version.max((parts.next().unwrap_or(0), parts.next().unwrap_or(0)));
    }
    for attr in ["available("] {
        for (at, _) in text.match_indices(attr) {
            let Some(args) = group(text, at + attr.len() - 1) else {
                continue;
            };
            for kv in args.split(',') {
                let Some((name, value)) = kv.split_once('=') else {
                    continue;
                };
                if name.trim() != key {
                    continue;
                }
                let mut parts = value
                    .trim()
                    .split('.')
                    .map(|part| part.parse().unwrap_or(0));
                version = version.max((parts.next().unwrap_or(0), parts.next().unwrap_or(0)));
            }
        }
    }
    version
}

/// What each line of a file requires of the OS: `base`, the requirement of
/// the module the file is, raised by the attributes of every item the line
/// is part of.
///
/// An item is taken to be the attributes and doc comments before it, then
/// everything up to where its brackets close — so an `impl` block's
/// attributes cover its members, and a member's cover the attributes written
/// beside the one that names a symbol.
fn line_requirements(text: &str, base: Version) -> Vec<Version> {
    let lines: Vec<&str> = text.lines().collect();
    let mut requires = vec![base; lines.len()];
    let mut index = 0;
    while index < lines.len() {
        // The attribute block, including attributes that span lines.
        let block_start = index;
        let mut depth = 0i32;
        while index < lines.len() {
            let line = lines[index].trim();
            if depth > 0 || line.starts_with("#[") {
                depth += bracket_depth(line, b'[', b']');
            } else if !(line.is_empty() || line.starts_with("//")) {
                break;
            }
            index += 1;
        }
        if index >= lines.len() {
            break;
        }
        // The item, up to where its brackets close or its statement ends.
        let item_start = index;
        let mut depth = 0i32;
        let mut opened = false;
        while index < lines.len() {
            let line = lines[index];
            let change = bracket_depth(line, b'{', b'}') + bracket_depth(line, b'(', b')');
            opened |= depth + change > 0 || line.contains('{') || line.contains('(');
            depth += change;
            index += 1;
            if depth <= 0 && (opened || line.trim_end().ends_with(';')) {
                break;
            }
        }
        let block: String = lines[block_start..item_start.max(block_start)].join("\n");
        let header = lines[item_start];
        let needed = required_version(&block).max(required_version(header));
        if needed > (0, 0) {
            for line in &mut requires[block_start..index] {
                *line = (*line).max(needed);
            }
        }
        // An item with a body holds items of its own, whose attributes add
        // to this one's, so the scan goes back inside it.
        if index > item_start + 1 {
            index = item_start + 1;
        }
    }
    requires
}

/// How far a line moves the nesting of one kind of bracket, not counting any
/// inside a literal.
fn bracket_depth(line: &str, open: u8, close: u8) -> i32 {
    let bytes = line.as_bytes();
    let mut depth = 0;
    let mut index = 0;
    while index < bytes.len() {
        if let Some((_, next)) = literal_at(bytes, index) {
            index = next;
            continue;
        }
        if bytes[index] == open {
            depth += 1;
        } else if bytes[index] == close {
            depth -= 1;
        }
        index += 1;
    }
    depth
}

/// What each file requires of the OS, from the `cfg`s on the `mod`
/// declarations leading to it: a module compiled only for a newer OS holds
/// only symbols that OS has.
fn module_requirements(root: &Path) -> BTreeMap<PathBuf, Version> {
    let mut out = BTreeMap::new();
    let mut pending = vec![(root.join("lib.rs"), (0, 0))];
    while let Some((file, base)) = pending.pop() {
        let Ok(source) = fs::read_to_string(&file) else {
            continue;
        };
        out.insert(file.clone(), base);
        let text = code_only(&source);
        let requires = line_requirements(&text, base);
        // `lib.rs` and `foo/mod.rs` own their directory; `foo.rs` owns `foo/`.
        let dir = if file
            .file_name()
            .is_some_and(|name| name == "lib.rs" || name == "mod.rs")
        {
            file.parent().unwrap().to_path_buf()
        } else {
            file.with_extension("")
        };
        for (number, line) in text.lines().enumerate() {
            let module = line
                .trim()
                .trim_start_matches("pub ")
                .trim_start_matches("pub(crate) ")
                .trim_start_matches("pub(super) ")
                .strip_prefix("mod ")
                .and_then(|rest| rest.strip_suffix(';'));
            if let Some(module) = module {
                let needed = requires[number];
                pending.push((dir.join(format!("{module}.rs")), needed));
                pending.push((dir.join(module).join("mod.rs"), needed));
            }
        }
    }
    out
}

/// Every symbol the bindings in `file` name, and where.
fn collect(
    file: &Path,
    base: Version,
    out: &mut BTreeMap<String, Vec<Site>>,
    errors: &mut Vec<String>,
) {
    let text = code_only(&fs::read_to_string(file).unwrap());
    let requires = line_requirements(&text, base);
    let shown = file
        .strip_prefix(env!("CARGO_MANIFEST_DIR"))
        .unwrap_or(file)
        .to_path_buf();
    let mut derive = |source: Source, at: usize| {
        let (result, decl, also_async) = match source {
            Source::Decl(decl) => (
                swift_mangle::mangle(decl),
                Some(decl.to_string()),
                swift_mangle::is_async(decl).unwrap_or(false),
            ),
            Source::AsyncDecl(decl) => (swift_mangle::mangle(decl), Some(decl.to_string()), true),
            Source::EnumCase(decl) => (
                swift_mangle::mangle_enum_case(decl),
                Some(format!("case {decl}")),
                false,
            ),
            Source::Accessor(kind, path) => (
                swift_mangle::kind_letter(kind)
                    .and_then(|kind| swift_mangle::mangle_metadata_accessor(path, kind)),
                Some(format!("{kind} {path}")),
                false,
            ),
            Source::Mangled(symbol) => (Ok(symbol), None, false),
        };
        let line = line_of(&text, at);
        match result {
            Ok(symbol) => {
                let mut add = |symbol: String| {
                    out.entry(symbol).or_default().push(Site {
                        file: shown.clone(),
                        line,
                        decl: decl.clone(),
                        requires: requires[line - 1],
                    })
                };
                if also_async {
                    add(format!("{symbol}Tu"));
                }
                add(symbol);
            }
            Err(e) => errors.push(format!(
                "{}:{line}: cannot mangle `{}`: {e}",
                shown.display(),
                decl.unwrap_or_default()
            )),
        }
    };
    let invocations = |marker: &str| -> Vec<(usize, &str)> {
        text.match_indices(marker)
            .filter_map(|(at, _)| Some((at, group(&text, at + marker.len() - 1)?)))
            .collect()
    };

    // Declarations the call and symbol macros mangle.
    for (at, args) in invocations("#[swift::call(") {
        if let Some(sym) = args.trim_start().strip_prefix("sym") {
            // Written mangled, so the literal is picked up with the rest below;
            // only the async function pointer beside it is added here.
            let written_async = args.split(',').any(|part| part.trim() == "async");
            if written_async && let Some((_, sym)) = literals(sym).into_iter().next() {
                derive(Source::Mangled(format!("{sym}Tu")), at);
            }
            continue;
        }
        if let Some(decl) = joined(args) {
            derive(Source::Decl(&decl), at);
        }
    }
    for (at, args) in invocations("swift::symbol!(") {
        if let Some(decl) = joined(args) {
            derive(Source::Decl(&decl), at);
        }
    }
    for (at, args) in invocations("swift::async_symbols!(") {
        if let Some(decl) = joined(args) {
            derive(Source::AsyncDecl(&decl), at);
        }
    }
    for (at, args) in invocations("swift::enum_case!(") {
        if let Some(decl) = joined(args) {
            derive(Source::EnumCase(&decl), at);
        }
    }

    // Metadata accessors named by type.
    for (at, args) in invocations("metadata_accessor!(") {
        let Some((kind, rest)) = args.split_once(',') else {
            continue;
        };
        if let Some(path) = joined(rest) {
            derive(Source::Accessor(kind.trim(), &path), at);
        }
    }
    for kind in ["class", "struct", "enum"] {
        for (at, args) in invocations(&format!("#[swift::{kind}(")) {
            if let Some((_, path)) = literals(args)
                .into_iter()
                .next()
                .filter(|_| !args.contains('$'))
            {
                derive(Source::Accessor(kind, &path), at);
            }
        }
    }

    // The enum and sequence macros, which derive their members' symbols from
    // the type's name.
    for (at, body) in invocations("define_swift_tag_enum!(") {
        let Some(path) = value_after(body, "= swift ").filter(|_| !body.contains('$')) else {
            continue;
        };
        let cases = &body[body.find("cases").unwrap_or(0)..];
        for (_, case) in assignments(cases) {
            derive(Source::EnumCase(&format!("{path}(enum).{case}")), at);
        }
        derive(
            Source::Decl(&format!("{path}(enum).hashValue: Int {{ get }}")),
            at,
        );
        if body.lines().any(|line| line.trim() == "debug,") {
            derive(
                Source::Decl(&format!("{path}(enum).debugDescription: String {{ get }}")),
                at,
            );
        }
    }
    for (at, body) in invocations("define_swift_getter_enum!(") {
        let Some(path) = value_after(body, "= swift ").filter(|_| !body.contains('$')) else {
            continue;
        };
        derive(Source::Accessor("struct", &path), at);
        let cases = &body[body.find("= swift ").unwrap()..];
        for (_, getter) in assignments(cases) {
            derive(
                Source::Decl(&format!(
                    "static {path}(struct).{getter}: {path}(struct) {{ get }}"
                )),
                at,
            );
        }
    }
    for (at, body) in invocations("define_async_sequence! {") {
        if body.contains('$') {
            continue;
        }
        // `element = Rust = "Swift"`, so the Swift type is the line's literal.
        let element = body
            .lines()
            .find(|line| line.trim().starts_with("element = "))
            .and_then(|line| literals(line).into_iter().next())
            .map(|(_, element)| element);
        let (Some(path), Some(element)) = (value_after(body, "sequence = "), element) else {
            continue;
        };
        derive(Source::Accessor("struct", &path), at);
        derive(
            Source::Accessor("struct", &format!("{path}(struct).Iterator")),
            at,
        );
        derive(
            Source::Decl(&format!(
                "{path}(struct).makeAsyncIterator() -> {path}(struct).Iterator(struct)"
            )),
            at,
        );
        derive(
            Source::AsyncDecl(&format!(
                "{path}(struct).Iterator(struct).next() async -> {element}?"
            )),
            at,
        );
    }

    // Everything written mangled.
    for (at, lit) in literals(&text) {
        let is_symbol = lit.len() > 2
            && lit.starts_with("$s")
            && lit
                .chars()
                .all(|c| c == '$' || c == '_' || c.is_ascii_alphanumeric());
        if is_symbol {
            derive(Source::Mangled(lit), at);
        }
    }
}

/// The module a symbol's entity lives in, as far as its first node says.
///
/// Enough to find the image to load: a module name spelled out, or one of the
/// standard library's own substitutions. `None` for a symbol that opens with
/// something else, such as an imported C type's extension.
fn module_of(symbol: &str) -> Option<String> {
    let rest = symbol.strip_prefix("$s")?;
    let digits: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
    if !digits.is_empty() && !digits.starts_with('0') {
        let len: usize = digits.parse().ok()?;
        return rest
            .get(digits.len()..digits.len() + len)
            .map(str::to_string);
    }
    if rest.starts_with("Sc") {
        return Some("_Concurrency".to_string());
    }
    if rest.starts_with("So") {
        return None;
    }
    if rest.starts_with('s') || rest.starts_with('S') {
        return Some("Swift".to_string());
    }
    None
}

/// Where the module's code is loaded from.
fn image_of(module: &str) -> String {
    match module {
        "Swift" => "/usr/lib/swift/libswiftCore.dylib".to_string(),
        "_Concurrency" => "/usr/lib/swift/libswift_Concurrency.dylib".to_string(),
        framework => format!("/System/Library/Frameworks/{framework}.framework/{framework}"),
    }
}

/// Loads each module once, remembering which ones exist on this platform.
#[derive(Default)]
struct Images(BTreeMap<String, bool>);

impl Images {
    fn load(&mut self, module: &str) -> bool {
        *self.0.entry(module.to_string()).or_insert_with(|| {
            let path = CString::new(image_of(module)).unwrap();
            !unsafe { dlopen(path.as_ptr(), RTLD_LAZY | RTLD_GLOBAL) }.is_null()
        })
    }
}

fn resolves(symbol: &str) -> bool {
    // A Mach-O symbol carries a leading underscore that `dlsym` adds itself.
    let name = CString::new(symbol).unwrap();
    !unsafe { dlsym(RTLD_DEFAULT, name.as_ptr()) }.is_null()
}

#[test]
fn symbol_audit() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    rust_files(&root, &mut files);
    files.sort();

    let mut symbols = BTreeMap::new();
    let mut errors = Vec::new();
    let modules = module_requirements(&root);
    // This file spells out example symbols of its own.
    let this = root.join("swift").join("symbol_audit.rs");
    for file in files.iter().filter(|file| **file != this) {
        let base = modules.get(file).copied().unwrap_or((0, 0));
        collect(file, base, &mut symbols, &mut errors);
    }
    let os = running_os();

    let verbose = std::env::var_os("CIDRE_AUDIT_VERBOSE").is_some();
    let mut images = Images::default();
    let mut checked = 0usize;
    let mut skipped = 0usize;
    for (symbol, sites) in &symbols {
        if let Some(module) = module_of(symbol)
            && !images.load(&module)
        {
            skipped += 1;
            if verbose {
                eprintln!("skipped {symbol}: {module} does not load here");
            }
            continue;
        }
        // Named only by bindings built for a newer OS than this one, which
        // need not have it.
        let needed = sites
            .iter()
            .map(|site| site.requires)
            .min()
            .unwrap_or((0, 0));
        if needed > os {
            skipped += 1;
            if verbose {
                eprintln!(
                    "skipped {symbol}: needs {} {}.{}, running {}.{}",
                    os_key(),
                    needed.0,
                    needed.1,
                    os.0,
                    os.1
                );
            }
            continue;
        }
        checked += 1;
        if !resolves(symbol) {
            let sites: Vec<String> = sites.iter().map(|site| format!("  {site}")).collect();
            errors.push(format!(
                "{symbol} does not resolve, named at\n{}",
                sites.join("\n")
            ));
        }
    }

    eprintln!(
        "swift symbol audit on {} {}.{}: {checked} checked, {skipped} skipped, {} problems",
        os_key(),
        os.0,
        os.1,
        errors.len()
    );
    assert!(
        checked > 100,
        "the scan found only {checked} symbols to check, so it has stopped seeing the bindings"
    );
    assert!(errors.is_empty(), "\n{}", errors.join("\n"));
}

#[test]
fn reads_the_module_off_a_symbol() {
    assert_eq!(
        Some("DockKit".into()),
        module_of("$s7DockKit0A9AccessoryCMa")
    );
    assert_eq!(Some("Swift".into()), module_of("$sSS5countSivg"));
    assert_eq!(Some("Swift".into()), module_of("$ss5Int64VN"));
    assert_eq!(Some("_Concurrency".into()), module_of("$sScPMa"));
    assert_eq!(None, module_of("$sSo6CGRectVMa"));
}
