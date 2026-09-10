//! Checks every selector the bindings send against the Objective-C runtime.
//!
//! Run with `cargo test -p cidre selector_audit`.
//!
//! The sources are scanned for `#[objc::msg_send(sel)]` and `#[objc::init(sel)]`
//! attributes. The Objective-C class of the enclosing `impl` block's type comes
//! from the `#[doc(alias = "...")]` on its `define_obj_type!`, and the runtime
//! is asked whether that class (or its metaclass, for methods without a
//! receiver) responds to the selector. Methods of `#[objc::protocol(...)]`
//! traits are checked against the protocol's method list.
//!
//! Classes and protocols that are not loaded in the test process (other
//! platforms' frameworks, cidre's own protocols) are skipped, so run it on each
//! platform. Set `CIDRE_AUDIT_VERBOSE=1` to list what was skipped.

use std::{
    collections::HashMap,
    ffi::{CString, c_char, c_void},
    fs,
    path::{Path, PathBuf},
};

use crate::objc::{Class, Id, Protocol, Sel, sel_reg_name};

#[repr(C)]
struct MethodDesc {
    name: *const c_void,
    types: *const c_char,
}

#[link(name = "objc")]
unsafe extern "C" {
    fn objc_copyClassList(count: *mut u32) -> *mut &'static Class<Id>;
    fn class_getSuperclass(cls: &Class<Id>) -> Option<&'static Class<Id>>;
    fn protocol_getMethodDescription(
        proto: &Protocol,
        sel: &Sel,
        required: bool,
        instance: bool,
    ) -> MethodDesc;
    fn free(ptr: *mut c_void);
}

use crate::objc::{class_respondsToSelector, objc_getClass, objc_getProtocol, object_getClass};

/// The class as an object, for `object_getClass`.
fn as_obj(cls: &Class<Id>) -> &Id {
    unsafe { std::mem::transmute(cls) }
}

/// What a selector is sent to.
#[derive(Clone, Debug)]
enum Target {
    /// A Rust type, resolved to a class through its doc alias.
    Type(String),
    /// A protocol trait.
    Protocol(String),
    /// A generic or unknown receiver, e.g. inside a `macro_rules!`.
    Unknown(String),
    /// Compiled out on this platform.
    Skipped,
}

#[derive(Debug)]
struct Send {
    file: PathBuf,
    line: usize,
    target: Target,
    sel: String,
    /// Whether the method has a receiver.
    instance: bool,
}

/// Whether a `#[cfg(..)]` predicate holds for the platform running the test.
///
/// Only `target_os` is decided; features and anything else are assumed on.
fn cfg_active(pred: &str) -> bool {
    let pred = pred.trim();
    for (name, all) in [("all(", true), ("any(", false)] {
        if let Some(inner) = pred.strip_prefix(name).and_then(|p| p.strip_suffix(')')) {
            let parts = split_top_level(inner);
            return if all {
                parts.iter().all(|p| cfg_active(p))
            } else {
                parts.iter().any(|p| cfg_active(p))
            };
        }
    }
    if let Some(inner) = pred.strip_prefix("not(").and_then(|p| p.strip_suffix(')')) {
        return !cfg_active(inner);
    }
    if let Some(os) = pred.strip_prefix("target_os") {
        let os = os.trim().trim_start_matches('=').trim().trim_matches('"');
        return os == std::env::consts::OS;
    }
    true
}

/// Splits `a, b(c, d), e` at the commas outside parentheses.
fn split_top_level(s: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0;
    let mut cur = String::new();
    for c in s.chars() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => {
                parts.push(std::mem::take(&mut cur));
                continue;
            }
            _ => {}
        }
        cur.push(c);
    }
    if !cur.trim().is_empty() {
        parts.push(cur);
    }
    parts
}

/// The predicate of a `#[cfg(..)]` attribute, which may span several lines.
fn cfg_attr(lines: &[&str], i: usize) -> Option<String> {
    let t = lines[i].trim();
    let start = t.strip_prefix("#[cfg(")?;
    let mut text = start.to_string();
    let mut j = i;
    while !text.contains(")]") {
        j += 1;
        text.push_str(lines.get(j)?.trim());
    }
    let end = text.rfind(")]")?;
    Some(text[..end].to_string())
}

/// The classes behind pomace symbols: `NS_VIEW = [NSView class];` or
/// `NS_VIEW = NSClassFromString(@"NSView");`.
fn pomace_symbols(dir: &Path) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut files = Vec::new();
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|e| e == "h" || e == "m") {
                out.push(path);
            }
        }
    }
    walk(dir, &mut files);
    for file in files {
        for line in fs::read_to_string(&file).unwrap().lines() {
            let t = line.trim();
            let Some((sym, rest)) = t.split_once(" = ") else {
                continue;
            };
            let sym = sym.trim();
            if !sym
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
            {
                continue;
            }
            let class = if let Some(r) = rest.strip_prefix('[') {
                r.split_whitespace().next()
            } else if let Some(r) = rest.strip_prefix("NSClassFromString(@\"") {
                r.split('"').next()
            } else {
                None
            };
            if let Some(class) = class {
                map.insert(sym.to_string(), class.to_string());
            }
        }
    }
    map
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

fn attr_arg(line: &str, attr: &str) -> Option<String> {
    let start = line.find(attr)? + attr.len();
    let rest = &line[start..];
    let end = rest.find(')')?;
    Some(rest[..end].trim().to_string())
}

fn doc_alias(line: &str) -> Option<String> {
    let start = line.find("#[doc(alias = \"")? + "#[doc(alias = \"".len();
    let rest = &line[start..];
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

/// The Rust type names defined in `lines` with their Objective-C class,
/// taken from the doc alias or the pomace symbol of their `define_obj_type!`.
fn aliases(
    lines: &[&str],
    symbols: &HashMap<String, String>,
) -> (HashMap<String, String>, std::collections::HashSet<String>) {
    let mut map = HashMap::new();
    let mut names = std::collections::HashSet::new();
    let mut pending: Option<String> = None;
    let mut in_define = false;
    let mut defined: Option<String> = None;
    let mut current_impl: Option<String> = None;
    for line in lines {
        let t = line.trim();
        if t.starts_with("impl") && !t.contains(" for ") {
            if let Target::Type(name) = impl_type(t) {
                current_impl = Some(name);
            }
        }
        if let Some(rest) = t.strip_prefix("define_cls!(") {
            if let (Some(name), Some(class)) = (&current_impl, symbol_class(rest, symbols)) {
                map.entry(name.clone()).or_insert(class);
            }
            continue;
        }
        if let Some(rest) = t
            .strip_prefix("define_obj_type!(")
            .or_else(|| t.strip_prefix("define_obj_type! {"))
        {
            in_define = true;
            pending = None;
            defined = None;
            if rest.trim().is_empty() {
                continue;
            }
            // Single line form: `define_obj_type!(pub Name(Base), SYMBOL);`
            let name = rest
                .trim_start_matches("pub ")
                .split(|c: char| !(c.is_alphanumeric() || c == '_'))
                .next()
                .unwrap_or("")
                .to_string();
            names.insert(name.clone());
            if let Some(class) = symbol_class(rest, symbols) {
                map.insert(name, class);
            }
            in_define = false;
            continue;
        }
        if in_define && defined.is_some() {
            // Trailing lines of the block: look for the pomace symbol.
            if let Some(class) = symbol_class(t, symbols) {
                map.entry(defined.clone().unwrap()).or_insert(class);
            }
            if t.starts_with(");") || t.ends_with(");") || t == "}" {
                in_define = false;
                defined = None;
            }
            continue;
        }
        if let Some(alias) = doc_alias(t) {
            if pending.is_none() {
                pending = Some(alias);
            }
            continue;
        }
        if t.starts_with("#[") || t.starts_with("///") || t.is_empty() {
            continue;
        }
        if in_define {
            // `pub Name(Base)`, `pub Name(Base),`, `Name + Trait,`, `Name(Base),`
            let name = t
                .trim_start_matches("pub ")
                .split(|c: char| !(c.is_alphanumeric() || c == '_'))
                .next()
                .unwrap_or("")
                .to_string();
            if name.is_empty() {
                in_define = false;
                continue;
            }
            names.insert(name.clone());
            if let Some(alias) = pending.take() {
                map.insert(name.clone(), alias);
            } else if let Some(class) = symbol_class(t, symbols) {
                map.insert(name.clone(), class);
            }
            if t.ends_with(");") {
                in_define = false;
            } else {
                defined = Some(name);
            }
            continue;
        }
        // `pub struct Name<..>(ns::Id, ..)` right after an alias.
        if let Some(rest) = t.strip_prefix("pub struct ") {
            let name: String = rest
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            names.insert(name.clone());
            if let Some(alias) = pending.take() {
                map.insert(name, alias);
            }
            continue;
        }
        pending = None;
    }
    map.insert("Id".to_string(), "NSObject".to_string());
    (map, names)
}

/// The class of the first pomace symbol mentioned in `text`.
fn symbol_class(text: &str, symbols: &HashMap<String, String>) -> Option<String> {
    text.split(|c: char| !(c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_'))
        .filter(|w| w.len() > 2 && w.contains('_'))
        .find_map(|w| symbols.get(w).cloned())
}

/// The receiver type of an `impl` header, without generics or paths.
fn impl_type(header: &str) -> Target {
    let mut rest = header.trim_start_matches("impl").trim_start();
    if rest.starts_with('<') {
        let mut depth = 0;
        let mut end = 0;
        for (i, c) in rest.char_indices() {
            match c {
                '<' => depth += 1,
                '>' => {
                    depth -= 1;
                    if depth == 0 {
                        end = i + 1;
                        break;
                    }
                }
                _ => {}
            }
        }
        rest = rest[end..].trim_start();
    }
    if rest.contains(" for ") {
        return Target::Unknown(header.to_string());
    }
    let path: String = rest
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_' || *c == ':')
        .collect();
    let mut name = path.rsplit("::").next().unwrap_or("").to_string();
    // `arc::A<Name>`: the allocation of `Name`
    if name == "A" || name == "Allocated" {
        let inner = rest.split('<').nth(1).unwrap_or("");
        let inner: String = inner
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_' || *c == ':')
            .collect();
        name = inner.rsplit("::").next().unwrap_or("").to_string();
    }
    if name.is_empty() {
        Target::Unknown(header.to_string())
    } else {
        Target::Type(name)
    }
}

fn collect(
    file: &Path,
    symbols: &HashMap<String, String>,
    global: &HashMap<String, Option<String>>,
) -> Vec<Send> {
    let text = fs::read_to_string(file).unwrap();
    let lines: Vec<&str> = text.lines().collect();
    let (mut aliases, names) = aliases(&lines, symbols);
    for (name, class) in global {
        if let (Some(class), false) = (class, names.contains(name)) {
            aliases.entry(name.clone()).or_insert(class.clone());
        }
    }
    let mut sends = Vec::new();

    // A stack of (brace depth at entry, target) for impl and trait blocks.
    let mut contexts: Vec<(i32, Target)> = Vec::new();
    let mut depth = 0i32;
    let mut protocol: Option<String> = None;
    let mut trait_alias: Option<String> = None;
    let mut in_macro_rules = false;
    // `#[cfg(..)]` attributes seen since the last item.
    let mut cfg_ok = true;
    // `available(..)` attributes seen since the last item.
    let mut avail_ok = true;
    let mut in_attr = false;

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        let t = line.trim();

        if let Some(pred) = cfg_attr(&lines, i) {
            cfg_ok &= cfg_active(&pred);
        }
        if let Some(attr) = available_attr(&lines, i) {
            avail_ok &= available_here(&attr);
        }

        if t.starts_with("macro_rules!") {
            in_macro_rules = true;
        }
        if let Some(name) = attr_arg(t, "#[objc::protocol(") {
            protocol = Some(name);
        }
        if let Some(alias) = doc_alias(t) {
            trait_alias = Some(alias);
        } else if !(t.starts_with("#[") || t.starts_with("//") || t.is_empty())
            && !t.starts_with("pub trait")
            && !t.starts_with("trait ")
        {
            trait_alias = None;
        }
        if t.starts_with("impl") || t.starts_with("pub trait") || t.starts_with("trait ") {
            // The header may span lines until its `{`.
            let mut header = String::new();
            let mut j = i;
            while j < lines.len() {
                header.push_str(lines[j].trim());
                header.push(' ');
                if lines[j].contains('{') {
                    break;
                }
                j += 1;
            }
            let target = if !cfg_ok {
                Target::Skipped
            } else if t.starts_with("impl") {
                if in_macro_rules {
                    Target::Unknown(format!("macro_rules {header}"))
                } else {
                    match impl_type(&header) {
                        Target::Type(name) => match aliases.get(&name) {
                            Some(alias) => Target::Type(alias.clone()),
                            None => Target::Unknown(format!("no alias for {name}: {header}")),
                        },
                        other => other,
                    }
                }
            } else {
                match protocol.take().or_else(|| trait_alias.take()) {
                    Some(name) => Target::Protocol(name),
                    None => Target::Unknown(header.to_string()),
                }
            };
            contexts.push((depth, target));
            cfg_ok = true;
            avail_ok = true;
        }

        let attr = if t.starts_with("//") {
            None
        } else {
            attr_arg(t, "#[objc::msg_send(").or_else(|| attr_arg(t, "#[objc::init("))
        };
        if let Some(sel) = attr {
            // The signature follows, after other attributes and docs.
            let mut j = i + 1;
            let mut sig = String::new();
            while j < lines.len() {
                let s = lines[j].trim();
                if let Some(pred) = cfg_attr(&lines, j) {
                    cfg_ok &= cfg_active(&pred);
                }
                if let Some(attr) = available_attr(&lines, j) {
                    avail_ok &= available_here(&attr);
                }
                if s.starts_with("#[") || s.starts_with("//") || s.is_empty() {
                    j += 1;
                    continue;
                }
                sig.push_str(s);
                sig.push(' ');
                if s.contains(';') || s.contains('{') || s.ends_with(')') {
                    break;
                }
                j += 1;
            }
            if let Some(open) = sig.find('(') {
                let params = sig[open + 1..].trim_start();
                let instance = is_self_param(params);
                let target = if cfg_ok && avail_ok {
                    contexts
                        .last()
                        .map(|(_, t)| t.clone())
                        .unwrap_or(Target::Unknown("top level".to_string()))
                } else {
                    Target::Skipped
                };
                cfg_ok = true;
                avail_ok = true;
                sends.push(Send {
                    file: file.to_path_buf(),
                    line: i + 1,
                    target,
                    sel,
                    instance,
                });
            }
        }

        // A multi-line attribute keeps the pending `cfg` state; any other
        // code line ends the item the attributes belonged to.
        if t.starts_with("#[") && t.matches('[').count() > t.matches(']').count() {
            in_attr = true;
        } else if in_attr {
            if t.matches(']').count() > t.matches('[').count() {
                in_attr = false;
            }
        } else if !(t.starts_with("#[") || t.starts_with("//") || t.is_empty()) {
            cfg_ok = true;
            avail_ok = true;
        }

        for c in line.chars() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if let Some((entry, _)) = contexts.last() {
                        if depth <= *entry {
                            contexts.pop();
                        }
                    }
                    if in_macro_rules && depth == 0 {
                        in_macro_rules = false;
                    }
                }
                _ => {}
            }
        }
        i += 1;
    }
    sends
}

/// Whether a parameter list starts with a receiver: `self`, `&self`,
/// `&'a mut self`, `mut self`.
fn is_self_param(params: &str) -> bool {
    let mut p = params.trim_start();
    if let Some(rest) = p.strip_prefix('&') {
        p = rest.trim_start();
        if let Some(rest) = p.strip_prefix('\'') {
            p = rest
                .trim_start_matches(|c: char| c.is_alphanumeric() || c == '_')
                .trim_start();
        }
    }
    if let Some(rest) = p.strip_prefix("mut ") {
        p = rest.trim_start();
    }
    p.starts_with("self") && !p[4..].starts_with(|c: char| c.is_alphanumeric() || c == '_')
}

/// Whether an `available(..)` attribute lists the platform running the test.
fn available_here(attr: &str) -> bool {
    let key = match std::env::consts::OS {
        "macos" => "macos",
        "ios" => "ios",
        "tvos" => "tvos",
        "watchos" => "watchos",
        "visionos" => "visionos",
        _ => return true,
    };
    attr.split(',')
        .any(|kv| kv.trim().starts_with(key) && kv.contains('='))
}

/// The argument of a multi-line `#[objc::available(..)]` or
/// `#[api::available(..)]` attribute starting at line `i`.
fn available_attr(lines: &[&str], i: usize) -> Option<String> {
    let t = lines[i].trim();
    let start = t
        .strip_prefix("#[objc::available(")
        .or_else(|| t.strip_prefix("#[api::available("))?;
    let mut text = start.to_string();
    let mut j = i;
    while !text.contains(")]") {
        j += 1;
        text.push_str(lines.get(j)?.trim());
    }
    let end = text.rfind(")]")?;
    Some(text[..end].to_string())
}

/// Files and directories compiled out on this platform by a `#[cfg(..)]` on
/// their `mod` declaration.
fn skipped_modules(files: &[PathBuf]) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for file in files {
        let text = fs::read_to_string(file).unwrap();
        let lines: Vec<&str> = text.lines().collect();
        let mut cfg_ok = true;
        let mut in_attr = false;
        for (i, line) in lines.iter().enumerate() {
            let t = line.trim();
            if let Some(pred) = cfg_attr(&lines, i) {
                cfg_ok &= cfg_active(&pred);
            }
            if t.starts_with("#[") && t.matches('[').count() > t.matches(']').count() {
                in_attr = true;
                continue;
            }
            if in_attr {
                if t.matches(']').count() > t.matches('[').count() {
                    in_attr = false;
                }
                continue;
            }
            if t.starts_with("#[") || t.starts_with("//") || t.is_empty() {
                continue;
            }
            let module = t
                .strip_prefix("pub mod ")
                .or_else(|| t.strip_prefix("pub(crate) mod "))
                .or_else(|| t.strip_prefix("mod "))
                .and_then(|m| m.strip_suffix(';'));
            if let (Some(module), false) = (module, cfg_ok) {
                // `lib.rs` and `foo.rs` own `foo/`; `foo/mod.rs` owns `foo/`.
                let dir = if file
                    .file_name()
                    .is_some_and(|n| n == "lib.rs" || n == "mod.rs")
                {
                    file.parent().unwrap().to_path_buf()
                } else {
                    file.with_extension("")
                };
                out.push(dir.join(format!("{module}.rs")));
                out.push(dir.join(module));
            }
            cfg_ok = true;
        }
    }
    out
}

fn cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}

/// Every registered class, keyed by its superclass.
struct Hierarchy(HashMap<usize, Vec<&'static Class<Id>>>);

fn key(cls: &Class<Id>) -> usize {
    cls as *const Class<Id> as usize
}

impl Hierarchy {
    fn load() -> Self {
        let mut map: HashMap<usize, Vec<&'static Class<Id>>> = HashMap::new();
        unsafe {
            let mut count = 0u32;
            let list = objc_copyClassList(&mut count);
            for i in 0..count as usize {
                let cls = *list.add(i);
                if let Some(sup) = class_getSuperclass(cls) {
                    map.entry(key(sup)).or_default().push(cls);
                }
            }
            free(list as *mut c_void);
        }
        Self(map)
    }

    /// Whether `cls`, or any class derived from it, responds to `sel`.
    ///
    /// Class clusters and Metal descriptors implement their selectors in
    /// private subclasses, so the public class alone is not enough.
    fn responds(&self, cls: &'static Class<Id>, sel: &Sel, instance: bool) -> bool {
        let mut stack = vec![cls];
        while let Some(cls) = stack.pop() {
            let receiver = if instance {
                Some(cls)
            } else {
                unsafe { object_getClass(Some(as_obj(cls))) }
            };
            if let Some(receiver) = receiver {
                if unsafe { class_respondsToSelector(receiver, sel) } {
                    return true;
                }
            }
            if let Some(subs) = self.0.get(&key(cls)) {
                stack.extend(subs);
            }
        }
        false
    }
}

#[test]
fn selectors_exist() {
    let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    rust_files(&src, &mut files);
    files.sort();
    let skipped = skipped_modules(&files);
    files.retain(|f| !skipped.iter().any(|s| f == s || f.starts_with(s)));

    let verbose = std::env::var_os("CIDRE_AUDIT_VERBOSE").is_some();
    let hierarchy = Hierarchy::load();
    let symbols = pomace_symbols(&Path::new(env!("CARGO_MANIFEST_DIR")).join("pomace"));

    // Types are often extended from other files, so aliases are crate-wide
    // as long as the name is unambiguous.
    let mut global: HashMap<String, Option<String>> = HashMap::new();
    for file in &files {
        let text = fs::read_to_string(file).unwrap();
        let lines: Vec<&str> = text.lines().collect();
        for (name, class) in aliases(&lines, &symbols).0 {
            global
                .entry(name)
                .and_modify(|c| {
                    if c.as_deref() != Some(&class) {
                        *c = None;
                    }
                })
                .or_insert(Some(class));
        }
    }
    let mut checked = 0usize;
    let mut missing = Vec::new();
    let mut unloaded = std::collections::BTreeSet::new();
    let mut unknown = 0usize;
    let mut unresolved = std::collections::BTreeMap::<String, usize>::new();

    for file in &files {
        for send in collect(file, &symbols, &global) {
            let sel = unsafe { sel_reg_name(cstr(&send.sel).as_ptr()) };
            match &send.target {
                Target::Type(class) => {
                    let Some(cls) = (unsafe { objc_getClass(cstr(class).as_ptr().cast()) }) else {
                        unloaded.insert(class.clone());
                        continue;
                    };
                    checked += 1;
                    if !hierarchy.responds(cls, sel, send.instance) {
                        missing.push(format!(
                            "{}:{}: {}{} {}",
                            send.file.strip_prefix(&src).unwrap().display(),
                            send.line,
                            if send.instance { "-[" } else { "+[" },
                            class,
                            send.sel
                        ));
                    }
                }
                Target::Protocol(name) => {
                    let Some(proto) = (unsafe { objc_getProtocol(cstr(name).as_ptr()) }) else {
                        unloaded.insert(format!("<{name}>"));
                        continue;
                    };
                    checked += 1;
                    let declared = [(true, true), (false, true), (true, false), (false, false)]
                        .iter()
                        .any(|(required, instance)| {
                            !unsafe {
                                protocol_getMethodDescription(proto, sel, *required, *instance)
                            }
                            .name
                            .is_null()
                        });
                    if !declared {
                        missing.push(format!(
                            "{}:{}: <{}> {}",
                            send.file.strip_prefix(&src).unwrap().display(),
                            send.line,
                            name,
                            send.sel
                        ));
                    }
                }
                Target::Unknown(why) => {
                    unknown += 1;
                    *unresolved
                        .entry(format!(
                            "{}: {why}",
                            send.file.strip_prefix(&src).unwrap().display()
                        ))
                        .or_default() += 1;
                }
                Target::Skipped => {}
            }
        }
    }

    println!(
        "selector audit: {checked} checked, {} missing, {} classes/protocols not loaded, {unknown} receivers unresolved",
        missing.len(),
        unloaded.len()
    );
    if verbose {
        for name in &unloaded {
            println!("not loaded: {name}");
        }
        for (why, n) in &unresolved {
            println!("unresolved ({n}): {why}");
        }
    }
    assert!(
        missing.is_empty(),
        "selectors the runtime does not know:\n{}",
        missing.join("\n")
    );
}
