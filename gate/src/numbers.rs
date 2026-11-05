// MU-DERIVATIONS D1.4, MU-STANDARDS 2.12 — THE NUMBERS GATE: every numeric
// item that carries a literal is classified, and every underived one is named
// in the open-physics register (`physics::axioms::OPEN_PHYSICS`).
//
//! The gate reads the tree's items with `syn` (never raw lines), under the
//! rules N1–N5 of MU-DERIVATIONS D1.4:
//!
//! - **N1, scope.** Every `const`, `static`, impl `const` and trait `const`
//!   with a default under `src/`, `algebra/src/`, `forms/src/` and `gate/`, except items
//!   under `#[cfg(test)]` (on the item or an enclosing module) and files
//!   under a `tests/` directory or named `tests.rs`.
//! - **N2, numeric type.** The item's type (references stripped, array,
//!   slice and tuple elements visited) has a numeric leaf. A float leaf or a
//!   float literal is refused outright.
//! - **N3, literal-bearing.** The initializer holds an integer literal
//!   outside `{0, 1}`. An initializer of paths, `0`, `1`, operators and
//!   calls is derived, and its inputs are read in turn.
//! - **N4, classification.** A literal-bearing numeric item carries exactly
//!   one class tag as the first token of a doc line: `EXACT:` (naming a
//!   `#[test]` fn that exists), `FORMAT:`, `EXTERNAL:`, `MEASURED:` (only
//!   under the instruments: `src/bin/`, `src/observe/`, `gate/`) or
//!   `REGISTERED:` (naming its register entry).
//! - **N5, bijection and value.** Every register entry's `site` is an item
//!   tagged `REGISTERED` with its name, every such item has an entry, and the
//!   entry's `value` is the initializer as `syn` prints it.
//!
//! N6 (an integer literal outside `{0, 1}` in a non-const expression of the
//! law paths, outside codecs and byte indices) is read as an inventory and
//! reported; it refuses nothing until the inventory is empty.

use std::path::{Path, PathBuf};

use syn::spanned::Spanned;
use syn::visit::Visit;

/// The class tags of N4, in the order a hit lists them.
const TAGS: [&str; 5] = ["EXACT:", "FORMAT:", "EXTERNAL:", "MEASURED:", "REGISTERED:"];

/// The numeric leaves of N2.
const NUMERIC: [&str; 17] = [
    "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize", "f32", "f64", "Duration",
    "BigRational", "BigInt",
];

/// The roots N1 reads.
const SCOPE: [&str; 4] = ["src", "algebra/src", "forms/src", "gate"];

/// The roots whose `#[test]` fns an `EXACT:` tag may name.
const TEST_ROOTS: [&str; 7] = ["src", "algebra", "forms", "sdk", "gate", "tests", "examples"];

/// The instruments, where a `MEASURED:` item may stand.
const INSTRUMENTS: [&str; 3] = ["src/bin/", "src/observe/", "gate/"];

/// The paths N6 reads.
const LAW_PATHS: [&str; 6] =
    ["src/physics/", "src/server/cognition/", "src/court/", "src/chronometry/", "src/economy/", "src/network/"];

/// The codec verbs, whose literals are field widths (N6).
const CODECS: [&str; 6] = ["put", "take", "encode", "decode", "write", "read"];

/// The register's file, relative to the repository root.
pub const REGISTER: &str = "src/physics/axioms/founding.rs";

/// The register's carrier.
const REGISTER_ITEM: &str = "OPEN_PHYSICS";

/// One numeric item of N1, as the gate reads it.
#[derive(Debug, Clone)]
pub struct NumericItem {
    /// `<file>::[<owner>::]<NAME>`, the coordinates a register entry cites.
    pub site: String,
    /// The file relative to the repository root.
    pub file: String,
    pub line: usize,
    pub name: String,
    /// The integer literals outside `{0, 1}` in the initializer (N3).
    pub literals: Vec<String>,
    /// A float leaf in the type or a float literal in the initializer (N2).
    pub float: bool,
    /// The numeric leaf the type carries (N2).
    pub numeric: bool,
    /// The doc lines that open with a class tag: (tag, text).
    pub tags: Vec<(&'static str, String)>,
    /// The initializer as `syn` prints it (N5).
    pub value: Option<String>,
}

/// One entry of the register, as the gate reads it (N5).
#[derive(Debug, Clone)]
pub struct Entry {
    pub name: String,
    pub value: String,
    pub site: String,
    pub line: usize,
}

/// One inline literal of N6.
#[derive(Debug, Clone)]
pub struct Inline {
    pub file: String,
    pub line: usize,
    pub literal: String,
}

/// The gate's reading of the tree: the hits it refuses and the N6 inventory.
#[derive(Debug, Default)]
pub struct NumbersReading {
    pub hits: Vec<String>,
    pub inventory: Vec<Inline>,
    pub items: usize,
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = match std::fs::read_dir(dir) {
        Ok(rd) => rd,
        Err(_) => return Ok(()),
    };
    for e in entries {
        let e = e.map_err(|e| format!("{}: {e}", dir.display()))?;
        let p = e.path();
        if p.is_dir() {
            if p.file_name().is_some_and(|n| n == "target") {
                continue;
            }
            walk(&p, out)?;
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
    Ok(())
}

fn relative(repo: &Path, file: &Path) -> String {
    // A file outside the repository is named whole.
    let rel = match file.strip_prefix(repo) {
        Ok(rel) => rel,
        Err(_) => file,
    };
    rel.components().map(|c| c.as_os_str().to_string_lossy().into_owned()).collect::<Vec<_>>().join("/")
}

/// A file N1 excludes: under a `tests/` directory, or named `tests.rs`.
fn is_test_file(rel: &str) -> bool {
    rel.split('/').any(|c| c == "tests") || rel.ends_with("/tests.rs") || rel == "tests.rs"
}

/// `#[cfg(test)]`, `#[cfg(all(test, …))]`: an item compiled for tests only.
fn is_cfg_test(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|a| {
        if !a.path().is_ident("cfg") {
            return false;
        }
        let syn::Meta::List(list) = &a.meta else { return false };
        let words: Vec<String> = list.tokens.to_string().split(|c: char| !c.is_alphanumeric() && c != '_').map(str::to_string).collect();
        words.iter().any(|w| w == "test") && !words.iter().any(|w| w == "not")
    })
}

fn has_test_attr(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|a| a.path().is_ident("test") || a.path().segments.last().is_some_and(|s| s.ident == "test"))
}

/// The doc lines that open with a class tag.
fn tags_of(attrs: &[syn::Attribute]) -> Vec<(&'static str, String)> {
    let mut out = Vec::new();
    for a in attrs {
        if !a.path().is_ident("doc") {
            continue;
        }
        let syn::Meta::NameValue(nv) = &a.meta else { continue };
        let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) = &nv.value else { continue };
        let line = s.value();
        let line = line.trim_start();
        for tag in TAGS {
            if let Some(rest) = line.strip_prefix(tag) {
                out.push((tag, rest.trim().to_string()));
            }
        }
    }
    out
}

/// N2: the type's leaves, references stripped, arrays, slices and tuples
/// visited. Returns (numeric, float).
fn leaves(ty: &syn::Type) -> (bool, bool) {
    match ty {
        syn::Type::Reference(r) => leaves(&r.elem),
        syn::Type::Array(a) => leaves(&a.elem),
        syn::Type::Slice(s) => leaves(&s.elem),
        syn::Type::Paren(p) => leaves(&p.elem),
        syn::Type::Group(g) => leaves(&g.elem),
        syn::Type::Tuple(t) => t.elems.iter().map(leaves).fold((false, false), |a, b| (a.0 || b.0, a.1 || b.1)),
        syn::Type::Path(p) => match p.path.segments.last() {
            Some(seg) => {
                let id = seg.ident.to_string();
                let float = id == "f32" || id == "f64";
                (NUMERIC.contains(&id.as_str()), float)
            }
            None => (false, false),
        },
        _ => (false, false),
    }
}

/// The integer literals outside `{0, 1}` and the float literals of an
/// expression (N3), read by a visit of every `ExprLit` it holds.
struct Literals {
    ints: Vec<String>,
    float: bool,
}

impl<'ast> Visit<'ast> for Literals {
    fn visit_lit(&mut self, lit: &'ast syn::Lit) {
        match lit {
            syn::Lit::Int(i) if !is_unit(i) => self.ints.push(i.to_string()),
            syn::Lit::Float(_) => self.float = true,
            _ => {}
        }
    }
}

/// `0` and `1`, in any radix, with any suffix and any `_`.
fn is_unit(i: &syn::LitInt) -> bool {
    matches!(i.base10_parse::<u128>(), Ok(0) | Ok(1))
}

/// The N1 collector over one file: items, test fns and N6 literals.
struct Collector {
    file: String,
    in_scope: bool,
    law_path: bool,
    cfg_test: usize,
    owner: Vec<String>,
    fns: Vec<String>,
    in_index: usize,
    items: Vec<NumericItem>,
    tests: Vec<String>,
    inventory: Vec<Inline>,
    register: Vec<Entry>,
}

impl Collector {
    fn item(&mut self, attrs: &[syn::Attribute], ident: &syn::Ident, ty: &syn::Type, expr: Option<&syn::Expr>) {
        if !self.in_scope || self.cfg_test > 0 || is_cfg_test(attrs) {
            return;
        }
        let (numeric, float_ty) = leaves(ty);
        let mut lits = Literals { ints: Vec::new(), float: false };
        if let Some(e) = expr {
            lits.visit_expr(e);
        }
        let name = ident.to_string();
        #[expect(clippy::disallowed_methods, reason = "rule 15 — the innermost owner is the one pushed last: the order is the nesting, content")]
        let site = match self.owner.last() {
            Some(o) => format!("{}::{o}::{name}", self.file),
            None => format!("{}::{name}", self.file),
        };
        if self.file == REGISTER && name == REGISTER_ITEM {
            if let Some(e) = expr {
                self.register = register_of(e);
            }
        }
        self.items.push(NumericItem {
            site,
            file: self.file.to_string(),
            line: ident.span().start().line,
            name,
            literals: lits.ints,
            float: float_ty || lits.float,
            numeric,
            tags: tags_of(attrs),
            value: expr.map(|e| quote::ToTokens::to_token_stream(e).to_string()),
        });
    }

    fn in_codec(&self) -> bool {
        self.fns.iter().any(|f| CODECS.iter().any(|c| f.starts_with(c)))
    }
}

impl<'ast> Visit<'ast> for Collector {
    fn visit_item_mod(&mut self, m: &'ast syn::ItemMod) {
        let test = is_cfg_test(&m.attrs);
        self.cfg_test += usize::from(test);
        syn::visit::visit_item_mod(self, m);
        self.cfg_test -= usize::from(test);
    }

    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let test = is_cfg_test(&i.attrs);
        let owner = match &*i.self_ty {
            syn::Type::Path(p) => p.path.segments.last().map(|s| s.ident.to_string()),
            _ => None,
        };
        self.cfg_test += usize::from(test);
        // An impl of no named path type is owned by `impl`.
        self.owner.push(match owner {
            Some(o) => o,
            None => "impl".to_string(),
        });
        syn::visit::visit_item_impl(self, i);
        self.owner.pop();
        self.cfg_test -= usize::from(test);
    }

    fn visit_item_trait(&mut self, t: &'ast syn::ItemTrait) {
        let test = is_cfg_test(&t.attrs);
        self.cfg_test += usize::from(test);
        self.owner.push(t.ident.to_string());
        syn::visit::visit_item_trait(self, t);
        self.owner.pop();
        self.cfg_test -= usize::from(test);
    }

    fn visit_item_fn(&mut self, f: &'ast syn::ItemFn) {
        if has_test_attr(&f.attrs) {
            self.tests.push(f.sig.ident.to_string());
        }
        let test = is_cfg_test(&f.attrs) || has_test_attr(&f.attrs);
        self.cfg_test += usize::from(test);
        self.fns.push(f.sig.ident.to_string());
        syn::visit::visit_item_fn(self, f);
        self.fns.pop();
        self.cfg_test -= usize::from(test);
    }

    fn visit_impl_item_fn(&mut self, f: &'ast syn::ImplItemFn) {
        let test = is_cfg_test(&f.attrs);
        self.cfg_test += usize::from(test);
        self.fns.push(f.sig.ident.to_string());
        syn::visit::visit_impl_item_fn(self, f);
        self.fns.pop();
        self.cfg_test -= usize::from(test);
    }

    fn visit_trait_item_fn(&mut self, f: &'ast syn::TraitItemFn) {
        self.fns.push(f.sig.ident.to_string());
        syn::visit::visit_trait_item_fn(self, f);
        self.fns.pop();
    }

    fn visit_item_const(&mut self, c: &'ast syn::ItemConst) {
        self.item(&c.attrs, &c.ident, &c.ty, Some(&c.expr));
    }

    fn visit_item_static(&mut self, s: &'ast syn::ItemStatic) {
        self.item(&s.attrs, &s.ident, &s.ty, Some(&s.expr));
    }

    fn visit_impl_item_const(&mut self, c: &'ast syn::ImplItemConst) {
        self.item(&c.attrs, &c.ident, &c.ty, Some(&c.expr));
    }

    fn visit_trait_item_const(&mut self, c: &'ast syn::TraitItemConst) {
        if let Some((_, e)) = &c.default {
            self.item(&c.attrs, &c.ident, &c.ty, Some(e));
        }
    }

    fn visit_type(&mut self, _: &'ast syn::Type) {}

    fn visit_expr_index(&mut self, e: &'ast syn::ExprIndex) {
        self.visit_expr(&e.expr);
        self.in_index += 1;
        self.visit_expr(&e.index);
        self.in_index -= 1;
    }

    fn visit_lit_int(&mut self, i: &'ast syn::LitInt) {
        if self.law_path && self.cfg_test == 0 && self.in_index == 0 && !self.fns.is_empty() && !self.in_codec() && !is_unit(i) {
            self.inventory.push(Inline { file: self.file.to_string(), line: i.span().start().line, literal: i.to_string() });
        }
    }
}

/// The register's entries, read as an array of struct literals.
fn register_of(e: &syn::Expr) -> Vec<Entry> {
    let syn::Expr::Array(a) = e else { return Vec::new() };
    let mut out = Vec::new();
    for el in &a.elems {
        let syn::Expr::Struct(s) = el else { continue };
        let field = |n: &str| -> Option<String> {
            s.fields.iter().find_map(|f| match (&f.member, &f.expr) {
                (syn::Member::Named(id), syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(v), .. })) if id == n => Some(v.value()),
                _ => None,
            })
        };
        if let (Some(name), Some(value), Some(site)) = (field("name"), field("value"), field("site")) {
            out.push(Entry { name, value, site, line: s.span().start().line });
        }
    }
    out
}

/// One file's reading.
fn read_file(repo: &Path, path: &Path, scope: bool) -> Result<Collector, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let ast = syn::parse_file(&text).map_err(|e| format!("{}: numbers gate cannot parse: {e}", path.display()))?;
    let rel = relative(repo, path);
    let mut c = Collector {
        in_scope: scope && !is_test_file(&rel),
        law_path: LAW_PATHS.iter().any(|p| rel.starts_with(p)) && !is_test_file(&rel),
        file: rel,
        cfg_test: 0,
        owner: Vec::new(),
        fns: Vec::new(),
        in_index: 0,
        items: Vec::new(),
        tests: Vec::new(),
        inventory: Vec::new(),
        register: Vec::new(),
    };
    c.visit_file(&ast);
    Ok(c)
}

/// The tree as N1 reads it.
#[derive(Debug, Default)]
pub struct Tree {
    /// The numeric items of the scope.
    pub items: Vec<NumericItem>,
    /// The `#[test]` fns of the test roots, by name.
    pub tests: Vec<String>,
    /// The register's entries.
    pub register: Vec<Entry>,
    /// The N6 inline literals of the law paths.
    pub inventory: Vec<Inline>,
}

/// The tree's numeric items, test fns, register and N6 inventory.
pub fn read_tree(repo: &Path) -> Result<Tree, String> {
    let mut items = Vec::new();
    let mut tests = Vec::new();
    let mut register = Vec::new();
    let mut inventory = Vec::new();
    let mut seen: Vec<PathBuf> = Vec::new();
    for root in TEST_ROOTS {
        let mut files = Vec::new();
        walk(&repo.join(root), &mut files)?;
        for f in files {
            if seen.contains(&f) {
                continue;
            }
            let rel = relative(repo, &f);
            let scope = SCOPE.iter().any(|s| rel.starts_with(&format!("{s}/")));
            let c = read_file(repo, &f, scope)?;
            items.extend(c.items);
            tests.extend(c.tests);
            inventory.extend(c.inventory);
            if !c.register.is_empty() {
                register = c.register;
            }
            seen.push(f);
        }
    }
    Ok(Tree { items, tests, register, inventory })
}

/// The first word of a class tag's text, the name it cites; a tag with no
/// word cites the empty name, which no test fn and no item carries.
fn first_word(text: &str) -> &str {
    match text.split_whitespace().next() {
        Some(word) => word,
        None => "",
    }
}

fn hit(item: &NumericItem, rule: &str, why: &str) -> String {
    format!("  {}:{}: `{}` — {rule}: {why}", item.file, item.line, item.name)
}

/// The numbers gate over the tree at `repo`: the hits of N2–N5, and the N6
/// inventory as a reading.
pub fn lint_numbers(repo: &Path) -> Result<NumbersReading, String> {
    let Tree { items, tests, register, inventory } = read_tree(repo)?;
    let mut hits = Vec::new();
    let mut counted = 0;
    for item in &items {
        if !item.numeric && !item.float {
            continue;
        }
        if item.float {
            hits.push(hit(item, "N2", "float — NO CONSTANT and law 4: exact or refused"));
            continue;
        }
        let registered = item.tags.iter().any(|(t, _)| *t == "REGISTERED:");
        if item.literals.is_empty() && !registered {
            continue;
        }
        counted += 1;
        match item.tags.as_slice() {
            [] => hits.push(format!(
                "  {}:{}: `{}` — N4: numeric item with literal(s) [{}] carries no class (EXACT | FORMAT | EXTERNAL | MEASURED | REGISTERED): NO CONSTANT, MU-RECURSION §0.0",
                item.file,
                item.line,
                item.name,
                item.literals.join(", ")
            )),
            [(tag, text)] => {
                if text.is_empty() {
                    hits.push(hit(item, "N4", &format!("the class tag `{tag}` carries no text")));
                    continue;
                }
                match *tag {
                    "EXACT:" => {
                        let named = first_word(text).trim_matches(|c| c == '`' || c == ',' || c == '.' || c == ';');
                        let leaf = match named.rsplit("::").next() {
                            Some(leaf) => leaf,
                            None => named,
                        };
                        if !tests.iter().any(|t| t == leaf) {
                            hits.push(hit(item, "N4", &format!("`EXACT:` names `{named}`, which is no #[test] fn of the tree")));
                        }
                    }
                    "MEASURED:" if !INSTRUMENTS.iter().any(|p| item.file.starts_with(p)) => {
                        hits.push(hit(
                            item,
                            "N4",
                            "`MEASURED:` outside the instruments (src/bin/, src/observe/, gate/): a measured number at a law site is read at run time there",
                        ));
                    }
                    "REGISTERED:" => {
                        let named = first_word(text).trim_matches('`');
                        if named != item.name {
                            hits.push(hit(item, "N5", &format!("`REGISTERED:` names `{named}`, not the item")));
                        }
                        match register.iter().find(|e| e.site == item.site) {
                            None => hits.push(hit(item, "N5", &format!("`REGISTERED` with no entry in {REGISTER} at site `{}`", item.site))),
                            Some(e) => {
                                if e.name != item.name {
                                    hits.push(hit(item, "N5", &format!("register entry at {REGISTER}:{} names `{}`", e.line, e.name)));
                                }
                                if item.value.as_deref() != Some(e.value.as_str()) {
                                    hits.push(hit(
                                        item,
                                        "N5",
                                        &format!(
                                            "value `{}` differs from the register's `{}` ({REGISTER}:{})",
                                            match item.value.as_deref() {
                                                Some(value) => value,
                                                None => "",
                                            },
                                            e.value,
                                            e.line
                                        ),
                                    ));
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            many => hits.push(hit(
                item,
                "N4",
                &format!("{} class tags; a numeric item carries exactly one", many.len()),
            )),
        }
    }
    for e in &register {
        let resolved = items.iter().any(|i| i.site == e.site && i.tags.iter().any(|(t, _)| *t == "REGISTERED:"));
        if !resolved {
            hits.push(format!(
                "  {REGISTER}:{}: `{}` — N5: register entry whose site `{}` is no item tagged REGISTERED (a stale entry)",
                e.line, e.name, e.site
            ));
        }
    }
    Ok(NumbersReading { hits, inventory, items: counted })
}
