use anyhow::{Context, Result, bail};
use cargo_metadata::{Metadata, MetadataCommand, TargetKind};
use proc_macro2::{TokenStream, TokenTree};
use quote::{format_ident, quote};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use syn::visit::{self, Visit};
use syn::visit_mut::{self, VisitMut};
use syn::{Attribute, File, Item, ItemMod, ItemUse, Lit, Meta, PathSegment, UseTree};

/// Bundles a Rust solution and any referenced workspace library crates.
pub fn bundle(input: &Path) -> Result<String> {
    let input = input
        .canonicalize()
        .with_context(|| format!("failed to find `{}`", input.display()))?;
    let mut solution = parse_and_expand(&input)?;
    retain_referenced_solution_modules(&mut solution);
    let metadata = MetadataCommand::new()
        .current_dir(input.parent().context("input has no parent directory")?)
        .no_deps()
        .exec()
        .context("failed to run `cargo metadata`; is the solution inside a Cargo workspace?")?;

    let referenced = ReferencedCrates::from_file(&solution);
    reject_external_dependencies(&metadata, &input, &referenced.names)?;
    let libraries = workspace_libraries(&metadata);
    let mut embedded = Vec::new();

    for crate_name in referenced
        .names
        .iter()
        .filter(|name| libraries.contains_key(*name))
    {
        let library_path = &libraries[crate_name];
        if library_path == &input {
            continue;
        }
        let mut library = parse_and_expand(library_path)
            .with_context(|| format!("failed to bundle library crate `{crate_name}`"))?;
        let references = CrateReferences::from_file(&solution, crate_name);
        retain_referenced_library_modules(&mut library, &references);
        CratePathRewriter::new(crate_name).visit_file_mut(&mut library);
        embedded.push(wrap_library(crate_name, library)?);
    }

    embedded.append(&mut solution.items);
    solution.items = embedded;
    Ok(prettyplease::unparse(&solution))
}

fn reject_external_dependencies(
    metadata: &Metadata,
    input: &Path,
    referenced: &BTreeSet<String>,
) -> Result<()> {
    let workspace_packages = metadata.workspace_packages();
    let package = workspace_packages
        .iter()
        .filter(|package| {
            package
                .manifest_path
                .parent()
                .is_some_and(|root| input.starts_with(root.as_std_path()))
        })
        .max_by_key(|package| package.manifest_path.components().count())
        .with_context(|| {
            format!(
                "could not find the Cargo package containing `{}`",
                input.display()
            )
        })?;

    for dependency in &package.dependencies {
        let crate_name = dependency
            .rename
            .clone()
            .unwrap_or_else(|| dependency.name.to_string())
            .replace('-', "_");
        if referenced.contains(&crate_name) && dependency.path.is_none() {
            bail!(
                "cannot bundle external dependency `{crate_name}` from crates.io or git; \
                 submission code may only use the standard library and path-based workspace crates"
            );
        }
    }
    Ok(())
}

fn workspace_libraries(metadata: &Metadata) -> BTreeMap<String, PathBuf> {
    metadata
        .workspace_packages()
        .iter()
        .flat_map(|package| package.targets.iter())
        .filter(|target| target.kind.iter().any(|kind| *kind == TargetKind::Lib))
        .map(|target| {
            (
                target.name.replace('-', "_"),
                target.src_path.clone().into_std_path_buf(),
            )
        })
        .collect()
}

fn parse_and_expand(path: &Path) -> Result<File> {
    let source =
        fs::read_to_string(path).with_context(|| format!("failed to read `{}`", path.display()))?;
    let mut file = syn::parse_file(&source)
        .with_context(|| format!("failed to parse `{}`", path.display()))?;
    let module_dir = child_module_dir(path)?;
    expand_items(&mut file.items, &module_dir, path)?;
    Ok(file)
}

fn expand_items(items: &mut [Item], module_dir: &Path, source_path: &Path) -> Result<()> {
    for item in items {
        let Item::Mod(module) = item else {
            continue;
        };

        if let Some((_, nested_items)) = &mut module.content {
            let nested_dir = module_dir.join(module.ident.to_string());
            expand_items(nested_items, &nested_dir, source_path)?;
            continue;
        }

        let module_path = resolve_module(module, module_dir, source_path)?;
        let nested = parse_and_expand(&module_path)?;
        let attributes = module
            .attrs
            .iter()
            .filter(|attribute| !attribute.path().is_ident("path"));
        let visibility = &module.vis;
        let identifier = &module.ident;
        let inner_attributes = &nested.attrs;
        let nested_items = &nested.items;
        *item = syn::parse2(quote! {
            #(#attributes)*
            #visibility mod #identifier {
                #(#inner_attributes)*
                #(#nested_items)*
            }
        })?;
    }
    Ok(())
}

fn resolve_module(module: &ItemMod, module_dir: &Path, source_path: &Path) -> Result<PathBuf> {
    if let Some(relative) = path_attribute(&module.attrs)? {
        return Ok(source_path
            .parent()
            .context("module source has no parent")?
            .join(relative));
    }

    let name = module.ident.to_string();
    let flat = module_dir.join(format!("{name}.rs"));
    let nested = module_dir.join(&name).join("mod.rs");
    match (flat.is_file(), nested.is_file()) {
        (true, false) => Ok(flat),
        (false, true) => Ok(nested),
        (true, true) => bail!(
            "module `{name}` is ambiguous: both `{}` and `{}` exist",
            flat.display(),
            nested.display()
        ),
        (false, false) => bail!(
            "could not resolve module `{name}` declared in `{}` (tried `{}` and `{}`)",
            source_path.display(),
            flat.display(),
            nested.display()
        ),
    }
}

fn path_attribute(attributes: &[Attribute]) -> Result<Option<PathBuf>> {
    for attribute in attributes {
        if !attribute.path().is_ident("path") {
            continue;
        }
        if let Meta::NameValue(value) = &attribute.meta
            && let syn::Expr::Lit(expression) = &value.value
            && let Lit::Str(path) = &expression.lit
        {
            return Ok(Some(PathBuf::from(path.value())));
        }
        bail!("expected #[path = \"...\"]");
    }
    Ok(None)
}

fn child_module_dir(path: &Path) -> Result<PathBuf> {
    let parent = path.parent().context("module source has no parent")?;
    if path.file_name().and_then(|name| name.to_str()) == Some("mod.rs") {
        Ok(parent.to_path_buf())
    } else if matches!(
        path.file_name().and_then(|name| name.to_str()),
        Some("lib.rs" | "main.rs")
    ) {
        Ok(parent.to_path_buf())
    } else {
        let stem = path.file_stem().context("module source has no file stem")?;
        Ok(parent.join(stem))
    }
}

#[derive(Default)]
struct CrateReferences {
    members: BTreeSet<String>,
    whole_crate: bool,
}

impl CrateReferences {
    fn from_file(file: &File, crate_name: &str) -> Self {
        struct Collector<'a> {
            crate_name: &'a str,
            references: CrateReferences,
        }

        impl<'ast> Visit<'ast> for Collector<'_> {
            fn visit_item_use(&mut self, item: &'ast ItemUse) {
                let mut leaves = Vec::new();
                flatten_use_tree(&item.tree, &mut Vec::new(), &mut leaves);
                for leaf in leaves {
                    if leaf
                        .path
                        .first()
                        .is_some_and(|root| root == self.crate_name)
                    {
                        if leaf.glob || leaf.alias.is_some() && leaf.path.len() == 1 {
                            self.references.whole_crate = true;
                        } else if let Some(member) = leaf.path.get(1) {
                            self.references.members.insert(member.clone());
                        } else {
                            self.references.whole_crate = true;
                        }
                    }
                }
                visit::visit_item_use(self, item);
            }

            fn visit_path(&mut self, path: &'ast syn::Path) {
                let segments: Vec<_> = path
                    .segments
                    .iter()
                    .map(|segment| segment.ident.to_string())
                    .collect();
                self.record_path(&segments);
                visit::visit_path(self, path);
            }

            fn visit_macro(&mut self, mac: &'ast syn::Macro) {
                for path in token_paths(mac.tokens.clone()) {
                    self.record_path(&path);
                }
                visit::visit_macro(self, mac);
            }
        }

        impl Collector<'_> {
            fn record_path(&mut self, segments: &[String]) {
                if segments.first().is_some_and(|root| root == self.crate_name) {
                    if let Some(member) = segments.get(1) {
                        self.references.members.insert(member.clone());
                    } else {
                        self.references.whole_crate = true;
                    }
                }
            }
        }

        let mut collector = Collector {
            crate_name,
            references: Self::default(),
        };
        collector.visit_file(file);
        collector.references
    }
}

#[derive(Clone)]
struct UseLeaf {
    path: Vec<String>,
    alias: Option<String>,
    glob: bool,
}

fn flatten_use_tree(tree: &UseTree, prefix: &mut Vec<String>, leaves: &mut Vec<UseLeaf>) {
    match tree {
        UseTree::Path(path) => {
            prefix.push(path.ident.to_string());
            flatten_use_tree(&path.tree, prefix, leaves);
            prefix.pop();
        }
        UseTree::Name(name) => {
            let mut path = prefix.clone();
            path.push(name.ident.to_string());
            leaves.push(UseLeaf {
                path,
                alias: None,
                glob: false,
            });
        }
        UseTree::Rename(rename) => {
            let mut path = prefix.clone();
            path.push(rename.ident.to_string());
            leaves.push(UseLeaf {
                path,
                alias: Some(rename.rename.to_string()),
                glob: false,
            });
        }
        UseTree::Group(group) => {
            for item in &group.items {
                flatten_use_tree(item, prefix, leaves);
            }
        }
        UseTree::Glob(_) => leaves.push(UseLeaf {
            path: prefix.clone(),
            alias: None,
            glob: true,
        }),
    }
}

fn token_paths(tokens: TokenStream) -> Vec<Vec<String>> {
    fn collect(tokens: TokenStream, paths: &mut Vec<Vec<String>>) {
        let trees: Vec<_> = tokens.into_iter().collect();
        for tree in &trees {
            if let TokenTree::Group(group) = tree {
                collect(group.stream(), paths);
            }
        }
        for start in 0..trees.len() {
            let TokenTree::Ident(identifier) = &trees[start] else {
                continue;
            };
            let mut path = vec![identifier.to_string()];
            let mut index = start + 1;
            while index + 2 < trees.len()
                && matches!(&trees[index], TokenTree::Punct(punctuation) if punctuation.as_char() == ':')
                && matches!(&trees[index + 1], TokenTree::Punct(punctuation) if punctuation.as_char() == ':')
            {
                let TokenTree::Ident(segment) = &trees[index + 2] else {
                    break;
                };
                path.push(segment.to_string());
                index += 3;
            }
            if path.len() > 1 {
                paths.push(path);
            }
        }
    }

    let mut paths = Vec::new();
    collect(tokens, &mut paths);
    paths
}

fn retain_referenced_solution_modules(solution: &mut File) {
    let modules: BTreeSet<_> = solution
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Mod(module) => Some(module.ident.to_string()),
            _ => None,
        })
        .collect();
    if modules.is_empty() {
        return;
    }

    let mut roots = RootDependencies::default();
    for item in &solution.items {
        if !matches!(item, Item::Mod(_)) {
            roots.visit_item(item);
        }
    }
    let mut selected: BTreeSet<_> = roots
        .unqualified
        .intersection(&modules)
        .cloned()
        .chain(roots.root_members.intersection(&modules).cloned())
        .collect();
    let mut pending: VecDeque<_> = selected.iter().cloned().collect();
    while let Some(module_name) = pending.pop_front() {
        let Some(Item::Mod(module)) = solution
            .items
            .iter()
            .find(|item| matches!(item, Item::Mod(candidate) if candidate.ident == module_name))
        else {
            continue;
        };
        let mut dependencies = RootDependencies::default();
        dependencies.visit_item_mod(module);
        for dependency in dependencies.root_members.intersection(&modules) {
            if selected.insert(dependency.clone()) {
                pending.push_back(dependency.clone());
            }
        }
    }
    solution.items.retain(
        |item| !matches!(item, Item::Mod(module) if !selected.contains(&module.ident.to_string())),
    );
}

fn retain_referenced_library_modules(library: &mut File, references: &CrateReferences) {
    let modules: BTreeSet<_> = library
        .items
        .iter()
        .filter_map(|item| match item {
            Item::Mod(module) => Some(module.ident.to_string()),
            _ => None,
        })
        .collect();
    if modules.is_empty() || references.whole_crate {
        return;
    }

    let exports = root_exports(&library.items, &modules);
    let mut required_members = references.members.clone();
    let mut selected = BTreeSet::new();
    let mut pending = VecDeque::new();
    resolve_members(
        &required_members,
        &modules,
        &exports,
        &mut selected,
        &mut pending,
    );

    while let Some(module_name) = pending.pop_front() {
        let Some(Item::Mod(module)) = library
            .items
            .iter()
            .find(|item| matches!(item, Item::Mod(candidate) if candidate.ident == module_name))
        else {
            continue;
        };
        let mut dependencies = RootDependencies::default();
        dependencies.visit_item_mod(module);
        required_members.extend(dependencies.root_members);
        resolve_members(
            &required_members,
            &modules,
            &exports,
            &mut selected,
            &mut pending,
        );
    }

    library.items.retain_mut(|item| match item {
        Item::Mod(module) => selected.contains(&module.ident.to_string()),
        Item::Use(item_use) => {
            prune_use_tree(&mut item_use.tree, &mut Vec::new(), &modules, &selected)
        }
        _ => true,
    });
}

fn root_exports(items: &[Item], modules: &BTreeSet<String>) -> BTreeMap<String, BTreeSet<String>> {
    let mut exports: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for item in items {
        let Item::Use(item_use) = item else {
            continue;
        };
        let mut leaves = Vec::new();
        flatten_use_tree(&item_use.tree, &mut Vec::new(), &mut leaves);
        for leaf in leaves {
            let Some(module) = source_module(&leaf.path, modules) else {
                continue;
            };
            if leaf.glob {
                continue;
            }
            let exported = leaf
                .alias
                .or_else(|| leaf.path.last().cloned())
                .unwrap_or_else(|| module.clone());
            exports.entry(exported).or_default().insert(module);
        }
    }
    exports
}

fn resolve_members(
    members: &BTreeSet<String>,
    modules: &BTreeSet<String>,
    exports: &BTreeMap<String, BTreeSet<String>>,
    selected: &mut BTreeSet<String>,
    pending: &mut VecDeque<String>,
) {
    for member in members {
        if modules.contains(member) {
            if selected.insert(member.clone()) {
                pending.push_back(member.clone());
            }
            continue;
        }
        if let Some(candidates) = exports.get(member) {
            for module in candidates {
                if selected.insert(module.clone()) {
                    pending.push_back(module.clone());
                }
            }
        }
    }
}

#[derive(Default)]
struct RootDependencies {
    root_members: BTreeSet<String>,
    unqualified: BTreeSet<String>,
}

impl RootDependencies {
    fn record_segments(&mut self, segments: &[String]) {
        match segments.first().map(String::as_str) {
            Some("crate" | "super") => {
                if let Some(member) = segments.get(1) {
                    self.root_members.insert(member.clone());
                }
            }
            Some("self") => {
                if let Some(member) = segments.get(1) {
                    self.unqualified.insert(member.clone());
                }
            }
            Some(member) => {
                self.unqualified.insert(member.to_owned());
            }
            None => {}
        }
    }

    fn record_use(&mut self, tree: &UseTree) {
        let mut leaves = Vec::new();
        flatten_use_tree(tree, &mut Vec::new(), &mut leaves);
        for leaf in leaves {
            let mut path = leaf.path.iter();
            match path.next().map(String::as_str) {
                Some("crate" | "super") => {
                    if let Some(member) = path.next() {
                        self.root_members.insert(member.clone());
                    }
                }
                Some("self") => {
                    if let Some(member) = path.next() {
                        self.unqualified.insert(member.clone());
                    }
                }
                Some(member) => {
                    self.unqualified.insert(member.to_owned());
                }
                None => {}
            }
        }
    }
}

impl<'ast> Visit<'ast> for RootDependencies {
    fn visit_item_use(&mut self, item: &'ast ItemUse) {
        self.record_use(&item.tree);
        visit::visit_item_use(self, item);
    }

    fn visit_path(&mut self, path: &'ast syn::Path) {
        let segments: Vec<_> = path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect();
        self.record_segments(&segments);
        visit::visit_path(self, path);
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        for path in token_paths(mac.tokens.clone()) {
            self.record_segments(&path);
        }
        visit::visit_macro(self, mac);
    }
}

fn source_module(path: &[String], modules: &BTreeSet<String>) -> Option<String> {
    path.iter()
        .find(|segment| !matches!(segment.as_str(), "crate" | "self" | "super"))
        .filter(|segment| modules.contains(*segment))
        .cloned()
}

fn prune_use_tree(
    tree: &mut UseTree,
    prefix: &mut Vec<String>,
    modules: &BTreeSet<String>,
    selected: &BTreeSet<String>,
) -> bool {
    match tree {
        UseTree::Path(path) => {
            prefix.push(path.ident.to_string());
            let keep = prune_use_tree(&mut path.tree, prefix, modules, selected);
            prefix.pop();
            keep
        }
        UseTree::Name(name) => {
            let mut full_path = prefix.clone();
            full_path.push(name.ident.to_string());
            source_module(&full_path, modules).is_none_or(|module| selected.contains(&module))
        }
        UseTree::Rename(rename) => {
            let mut full_path = prefix.clone();
            full_path.push(rename.ident.to_string());
            source_module(&full_path, modules).is_none_or(|module| selected.contains(&module))
        }
        UseTree::Glob(_) => {
            source_module(prefix, modules).is_none_or(|module| selected.contains(&module))
        }
        UseTree::Group(group) => {
            let original = std::mem::take(&mut group.items);
            group.items = original
                .into_iter()
                .filter_map(|mut item| {
                    prune_use_tree(&mut item, prefix, modules, selected).then_some(item)
                })
                .collect();
            !group.items.is_empty()
        }
    }
}

fn wrap_library(crate_name: &str, library: File) -> Result<Item> {
    let identifier = format_ident!("{crate_name}");
    let attributes = library.attrs;
    let items = library.items;
    Ok(syn::parse2(quote! {
        #[allow(warnings)]
        mod #identifier {
            #(#attributes)*
            #(#items)*
        }
    })?)
}

#[derive(Default)]
struct ReferencedCrates {
    names: BTreeSet<String>,
}

impl ReferencedCrates {
    fn from_file(file: &File) -> Self {
        let mut visitor = Self::default();
        visitor.visit_file(file);
        visitor
    }

    fn record_use(&mut self, tree: &UseTree) {
        match tree {
            UseTree::Path(path) => {
                self.names.insert(path.ident.to_string());
            }
            UseTree::Name(name) => {
                self.names.insert(name.ident.to_string());
            }
            UseTree::Rename(rename) => {
                self.names.insert(rename.ident.to_string());
            }
            UseTree::Group(group) => {
                for item in &group.items {
                    self.record_use(item);
                }
            }
            UseTree::Glob(_) => {}
        }
    }
}

impl<'ast> Visit<'ast> for ReferencedCrates {
    fn visit_item_use(&mut self, item: &'ast ItemUse) {
        self.record_use(&item.tree);
        visit::visit_item_use(self, item);
    }

    fn visit_path(&mut self, path: &'ast syn::Path) {
        if let Some(first) = path.segments.first() {
            self.names.insert(first.ident.to_string());
        }
        visit::visit_path(self, path);
    }
}

struct CratePathRewriter {
    crate_identifier: syn::Ident,
}

impl CratePathRewriter {
    fn new(crate_name: &str) -> Self {
        Self {
            crate_identifier: format_ident!("{crate_name}"),
        }
    }

    fn rewrite_use_tree(&self, tree: &mut UseTree) {
        match tree {
            UseTree::Path(path) if path.ident == "crate" => {
                let remainder = (*path.tree).clone();
                path.tree = Box::new(UseTree::Path(syn::UsePath {
                    ident: self.crate_identifier.clone(),
                    colon2_token: Default::default(),
                    tree: Box::new(remainder),
                }));
            }
            UseTree::Path(path) => self.rewrite_use_tree(&mut path.tree),
            UseTree::Group(group) => {
                for item in &mut group.items {
                    self.rewrite_use_tree(item);
                }
            }
            _ => {}
        }
    }
}

impl VisitMut for CratePathRewriter {
    fn visit_item_use_mut(&mut self, item: &mut ItemUse) {
        self.rewrite_use_tree(&mut item.tree);
        visit_mut::visit_item_use_mut(self, item);
    }

    fn visit_path_mut(&mut self, path: &mut syn::Path) {
        if path
            .segments
            .first()
            .is_some_and(|segment| segment.ident == "crate")
        {
            path.segments
                .insert(1, PathSegment::from(self.crate_identifier.clone()));
        }
        visit_mut::visit_path_mut(self, path);
    }
}
