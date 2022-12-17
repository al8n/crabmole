//! Crabmole derives
#![deny(missing_docs)]

use proc_macro2::{Delimiter, Spacing, TokenTree};
use quote::quote;
use syn::{
    parse::{Parse, Parser},
    Type, Visibility,
};

#[derive(Debug, Default)]
struct Constants {
    normal: Vec<proc_macro2::TokenStream>,
    structs: Vec<proc_macro2::TokenStream>,
}

/// Declare constants like Go.
///
/// # Examples
/// ## All in one
/// ```rust
/// use crabmole_derive::constn;
///
/// pub struct Foo<T, R>(T, R);
///
/// pub struct Bar<R, T>(R, T);
///
/// constn! {
///     u8 {
///         pub FOO_0 = 0;
///         /// The first element
///         pub(crate) FOO_1 = 1;
///         pub FOO_2 = 2;
///     },
///     Foo<u64, i64> {
///         pub FOO_00 = Foo(0, 0);
///         pub FOO_11 = Foo(1, 1);
///         FOO_22 = Foo(2, 2);
///     },
///     impl Foo<u32, u64> {
///         u32 {
///             pub B0 = 0;
///             pub B1 = 1;
///             pub B2 = 2;
///         },
///         Bar<u32, u64> {
///             pub BAR_0 = Bar(0, 0);
///         },
///
///     },
///     impl<T: core::fmt::Debug, R: core::fmt::Debug> Foo<T, R> {
///         Bar<u64, i64> {
///             pub BAR_00 = Bar(0, 0);
///             pub(crate) BAR_11 = Bar(1, 1);
///             BAR_22 = Bar(2, 2);
///         },
///         u32 {
///             pub BAR_1 = 1;
///             pub BAR_2 = 2;
///             pub BAR_3 = 3;
///         }
///     },
/// }
/// ```
///
/// ## Declare constants for struct
/// ```rust
/// use crabmole_derive::constn;
///
/// #[derive(PartialEq, Eq, Debug)]
/// pub struct Foo(u32);
///
/// constn! {
///     u8 {
///         pub U1 = 1;
///         pub(crate) U2 = 2;
///     },
///     Foo {
///
///         pub FOO_A = Foo(1);
///         pub(crate) FOO_B = Foo(2);
///         FOO_C = Foo(3);
///
///     }
/// }
///
/// assert_eq!(FOO_A, Foo(1));
/// assert_eq!(FOO_B, Foo(2));
/// assert_eq!(FOO_C, Foo(3));
/// assert_eq!(U1, 1);
/// assert_eq!(U2, 2);
/// ```
///
/// ## Declare generic constants
/// ```rust
/// use crabmole_derive::constn;
///
/// #[derive(Debug, PartialEq, Eq)]
/// pub struct Foo<T>(T);
///
/// constn! {
///     Foo<u32> {
///         A = Foo(1);
///         pub(crate) B = Foo(2);
///     }
/// }
///
/// assert_eq!(A, Foo(1));
/// assert_eq!(B, Foo(2));
/// ```
///
/// ## Declare constants for generics with trait bounds
/// ```rust
/// use crabmole_derive::constn;
///
/// struct Bar<T>(T);
///
/// constn! {
///     impl<T: core::fmt::Debug> Bar<T> {
///         u8 {
///             pub A = 1;
///             pub(crate) B = 2;
///             C = 3;
///         },
///         u16 {
///             pub D = 1;
///             pub(crate) E = 2;
///             F = 3;
///         }
///     }
/// }
///
/// #[derive(Debug)]
/// struct Baz;
///
/// assert_eq!(Bar::<Baz>::A, 1);
/// assert_eq!(Bar::<Baz>::B, 2);
/// assert_eq!(Bar::<Baz>::C, 3);
/// assert_eq!(Bar::<Baz>::D, 1);
/// assert_eq!(Bar::<Baz>::E, 2);
/// assert_eq!(Bar::<Baz>::F, 3);
/// ```
///
/// ## Declare constants for generics
/// ```rust
/// use crabmole_derive::constn;
///
/// struct Bar<T>(T);
///
/// constn! {
///     impl Bar<u32> {
///         u8 {
///             pub D = 1;
///             pub(crate) E = 2;
///             F = 3;
///         }
///     }
/// }
///
/// assert_eq!(Bar::<u32>::D, 1);
/// assert_eq!(Bar::<u32>::E, 2);
/// assert_eq!(Bar::<u32>::F, 3);
/// ```
#[proc_macro]
pub fn constn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let mut iter = input.into_iter().peekable();
    let mut consts = Constants::default();
    while let Some(tt) = iter.next() {
        match tt {
            TokenTree::Ident(target) => {
                if target == "impl" {
                    let mut impl_statement = vec![TokenTree::Ident(target)];
                    'inner: for tt in iter.by_ref() {
                        match tt {
                            TokenTree::Group(g)
                                if g.delimiter() == Delimiter::Brace
                                    || g.delimiter() == Delimiter::Parenthesis =>
                            {
                                let sub_iter = g.stream().into_iter();
                                let mut const_ty_segments = Vec::new();
                                let mut impls = Vec::new();
                                let mut next_const_ty = false;
                                for tt in sub_iter {
                                    match tt {
                                        TokenTree::Group(g)
                                            if g.delimiter() == Delimiter::Brace
                                                || g.delimiter() == Delimiter::Parenthesis =>
                                        {
                                            let mut subsub_iter = g.stream().into_iter();
                                            let const_ty = match Type::parse
                                                .parse2(quote! { #(#const_ty_segments)* })
                                            {
                                                Ok(v) => v,
                                                Err(e) => {
                                                    return e.to_compile_error().into();
                                                }
                                            };
                                            let const_exprs = match hanlde_struct_constructs_declare(
                                                const_ty,
                                                &mut subsub_iter,
                                            ) {
                                                Ok(v) => v,
                                                Err(e) => {
                                                    return e.to_compile_error().into();
                                                }
                                            };
                                            impls.push(quote! {
                                                #(#const_exprs)*
                                            });
                                            const_ty_segments.clear();
                                            next_const_ty = true;
                                        }
                                        TokenTree::Punct(p) => {
                                            if p.as_char() == ',' && next_const_ty {
                                                next_const_ty = false;
                                                continue;
                                            } else {
                                                const_ty_segments.push(TokenTree::Punct(p));
                                            }
                                        }
                                        x => {
                                            const_ty_segments.push(x);
                                        }
                                    }
                                }
                                consts.structs.push(quote! {
                                    #(#impl_statement)* {
                                        #(#impls)*
                                    }
                                });
                                break 'inner;
                            }
                            x => impl_statement.push(x),
                        }
                    }
                    impl_statement.clear();
                } else {
                    let mut const_ty_segments = vec![TokenTree::Ident(target)];
                    let mut const_exprs = Vec::new();
                    'inner: for tt in iter.by_ref() {
                        match tt {
                            TokenTree::Group(g)
                                if g.delimiter() == Delimiter::Brace
                                    || g.delimiter() == Delimiter::Parenthesis =>
                            {
                                let mut sub_iter = g.stream().into_iter();
                                let const_ty =
                                    match Type::parse.parse2(quote! { #(#const_ty_segments)* }) {
                                        Ok(v) => v,
                                        Err(e) => {
                                            return e.to_compile_error().into();
                                        }
                                    };
                                const_exprs = match hanlde_normal_constants_declare(
                                    const_ty,
                                    &mut sub_iter,
                                ) {
                                    Ok(v) => v,
                                    Err(e) => {
                                        return e.to_compile_error().into();
                                    }
                                };
                                break 'inner;
                            }
                            x => const_ty_segments.push(x),
                        }
                    }
                    consts.normal.push(quote! {
                        #(#const_exprs)*
                    });
                }
            }
            TokenTree::Punct(p) => {
                if p.as_char() != ',' {
                    return syn::Error::new(p.span(), "constn: expected comma")
                        .to_compile_error()
                        .into();
                }
            }
            x => {
                return syn::Error::new(x.span(), "constn: expected identifier")
                    .to_compile_error()
                    .into()
            }
        }
    }

    let Constants { normal, structs } = consts;

    quote! {
        #(#normal)*

        #(#structs)*
    }
    .into()
}

fn hanlde_struct_constructs_declare(
    ty: Type,
    iter: &mut dyn Iterator<Item = TokenTree>,
) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let mut constants = Vec::new();
    let mut iter = iter.peekable();
    while iter.peek().is_some() {
        let mut expr = take_while_not_semicolon(&mut iter).into_iter().peekable();
        while expr.peek().is_some() {
            let mut lhs = take_while_equal(&mut expr).into_iter().peekable();
            let (size, _) = lhs.size_hint();
            if size == 1 {
                let rhs = take_after_equal(&mut expr);
                let lhs = match lhs.next().unwrap() {
                    TokenTree::Ident(i) => i,
                    x => return Err(syn::Error::new(x.span(), "constn: expected identifier")),
                };
                constants.push(quote! (
                    const #lhs: #ty = #rhs;
                ));
                continue;
            }

            if size == 2 {
                let rhs = take_after_equal(&mut expr);
                let vis = match lhs.next().unwrap() {
                    TokenTree::Ident(i) => syn::parse_str::<Visibility>(&i.to_string())?,
                    x => return Err(syn::Error::new(x.span(), "constn: expected identifier")),
                };
                let lhs = match lhs.next().unwrap() {
                    TokenTree::Ident(i) => i,
                    x => return Err(syn::Error::new(x.span(), "constn: expected identifier")),
                };
                constants.push(quote! (
                    #vis const #lhs: #ty = #rhs;
                ));
                continue;
            }

            let rhs = take_after_equal(&mut expr);
            let vis = lhs.clone().take(size - 1);
            let lhs = match lhs.last().unwrap() {
                TokenTree::Ident(i) => i,
                x => return Err(syn::Error::new(x.span(), "constn: expected identifier")),
            };
            constants.push(quote! (
                #(#vis)* const #lhs: #ty = #rhs;
            ));
        }
    }

    Ok(constants)
}

fn hanlde_normal_constants_declare(
    ident: Type,
    iter: &mut dyn Iterator<Item = TokenTree>,
) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let mut constants = Vec::new();
    let mut iter = iter.peekable();
    while iter.peek().is_some() {
        let mut expr = take_while_not_semicolon(&mut iter).into_iter().peekable();
        while expr.peek().is_some() {
            let mut lhs = take_while_equal(&mut expr).into_iter().peekable();
            let (size, _) = lhs.size_hint();
            if size == 1 {
                let rhs = take_after_equal(&mut expr);
                let lhs = match lhs.next().unwrap() {
                    TokenTree::Ident(i) => i,
                    x => return Err(syn::Error::new(x.span(), "constn: expected identifier")),
                };
                constants.push(quote! (
                    const #lhs: #ident = #rhs;
                ));
                continue;
            }

            if size == 2 {
                let rhs = take_after_equal(&mut expr);
                let vis = match lhs.next().unwrap() {
                    TokenTree::Ident(i) => syn::parse_str::<Visibility>(&i.to_string())?,
                    x => return Err(syn::Error::new(x.span(), "constn: expected identifier")),
                };
                let lhs = match lhs.next().unwrap() {
                    TokenTree::Ident(i) => i,
                    x => return Err(syn::Error::new(x.span(), "constn: expected identifier")),
                };
                constants.push(quote! (
                    #vis const #lhs: #ident = #rhs;
                ));
                continue;
            }

            let rhs = take_after_equal(&mut expr);
            let vis = lhs.clone().take(size - 1);
            let lhs = match lhs.last().unwrap() {
                TokenTree::Ident(i) => i,
                x => return Err(syn::Error::new(x.span(), "constn: expected identifier")),
            };
            constants.push(quote! (
                #(#vis)* const #lhs: #ident = #rhs;
            ));
        }
    }

    Ok(constants)
}

fn take_while_not_semicolon(i: &mut dyn Iterator<Item = TokenTree>) -> proc_macro2::TokenStream {
    i.take_while(
        |i| !matches!(i, TokenTree::Punct(p) if p.as_char() == ';' && p.spacing() == Spacing::Alone),
      )
      .collect()
}

fn take_while_equal(i: &mut dyn Iterator<Item = TokenTree>) -> proc_macro2::TokenStream {
    i.take_while(
        |i| !matches!(i, TokenTree::Punct(p) if p.as_char() == '=' && p.spacing() == Spacing::Alone),
      )
      .collect()
}

fn take_after_equal(i: &mut dyn Iterator<Item = TokenTree>) -> proc_macro2::TokenStream {
    i.filter(
        |i| !matches!(i, TokenTree::Punct(p) if p.as_char() != '=' && p.spacing() == Spacing::Alone),
      )
      .collect()
}
