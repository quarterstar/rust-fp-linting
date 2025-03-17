use proc_macro::TokenStream;
use quote::quote;
use syn::{visit::Visit, File, ItemFn};

#[proc_macro_attribute]
pub fn apply_deny_all(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast: File = syn::parse(item).unwrap();
    let mut visitor = FunctionVisitor::default();
    visitor.visit_file(&ast);

    for item in &mut ast.items {
        if let syn::Item::Fn(item_fn) = item {
            item_fn.attrs.push(syn::parse_quote! {
                #[rust_fp_linting::deny_all]
            });
        }
    }

    TokenStream::from(quote!(#ast))
}

#[derive(Default)]
struct FunctionVisitor;

impl<'ast> Visit<'ast> for FunctionVisitor {
    fn visit_item_fn(&mut self, _: &'ast ItemFn) {
        // Do nothing, just visit all functions
    }
}

#[proc_macro_attribute]
pub fn deny_all(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: File = syn::parse(item).unwrap();
    let mut visitor = Visitor::default();
    visitor.visit_file(&ast);

    if visitor.has_for_loop {
        panic!("For loops are not allowed in this codebase!");
    }

    if visitor.has_while_loop {
        panic!("While loops are not allowed in this codebase!");
    }

    if visitor.has_mut {
        panic!("Mutable values are not allowed in this codebase! State must be pure.");
    }

    if visitor.has_continue {
        println!("`continue` is not allowed in this codebase! Loop operators are not allowed.")
    }

    if visitor.has_break {
        println!("`break` is not allowed in this codebase! Loop operators are not allowed.")
    }

    TokenStream::from(quote!(#ast))
}

struct Visitor {
    has_for_loop: bool,
    has_while_loop: bool,
    has_mut: bool,
    has_continue: bool,
    has_break: bool,
}

impl Default for Visitor {
    fn default() -> Self {
        Self {
            has_for_loop: false,
            has_while_loop: false,
            has_mut: false,
            has_continue: false,
            has_break: false,
        }
    }
}

impl<'ast> Visit<'ast> for Visitor {
    fn visit_expr_for_loop(&mut self, _: &syn::ExprForLoop) {
        self.has_for_loop = true;
    }

    fn visit_expr_while(&mut self, _: &'_ syn::ExprWhile) {
        self.has_while_loop = true;
    }

    fn visit_expr_loop(&mut self, _: &'_ syn::ExprLoop) {
        self.has_while_loop = true;
    }

    fn visit_expr_continue(&mut self, _: &'_ syn::ExprContinue) {
        self.has_continue = true;
    }

    fn visit_expr_break(&mut self, _: &'_ syn::ExprBreak) {
        self.has_break = true;
    }

    fn visit_pat(&mut self, pat: &'ast syn::Pat) {
        if let syn::Pat::Ident(pat_ident) = pat {
            if pat_ident.mutability.is_some() {
                self.has_mut = true;
            }
        }
        syn::visit::visit_pat(self, pat);
    }

    fn visit_fn_arg(&mut self, arg: &'ast syn::FnArg) {
        if let syn::FnArg::Typed(pat_type) = arg {
            if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                if pat_ident.mutability.is_some() {
                    self.has_mut = true;
                }
            }
        }
        syn::visit::visit_fn_arg(self, arg);
    }

    fn visit_type_reference(&mut self, ty: &'ast syn::TypeReference) {
        if ty.mutability.is_some() {
            self.has_mut = true;
        }
        syn::visit::visit_type_reference(self, ty);
    }
}
