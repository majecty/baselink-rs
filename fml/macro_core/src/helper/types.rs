// Copyright 2020 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

/// Note that this verbose manual pattern matching is due to
/// lack of Rust's generic specializaiton :(
pub fn is_handle(the_type: &syn::Type) -> Option<syn::Path> {
    match the_type {
        syn::Type::Path(syn::TypePath {
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments,
            },
        }) => {
            // we can't use pattern matching with an ident.
            if segments.len() == 1 && segments.first().unwrap().ident == quote::format_ident!("Box") {
                match segments.first().unwrap() {
                    syn::PathSegment {
                        ident: _,
                        arguments:
                            syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                                colon2_token: None,
                                lt_token: _,
                                args,
                                gt_token: _,
                            }),
                    } => {
                        if args.len() == 1 {
                            match args.first().unwrap() {
                                syn::GenericArgument::Type(syn::Type::TraitObject(syn::TypeTraitObject {
                                    dyn_token: Some(_),
                                    bounds,
                                })) => {
                                    if bounds.len() == 1 {
                                        match bounds.first().unwrap() {
                                            syn::TypeParamBound::Trait(syn::TraitBound {
                                                paren_token: None,
                                                modifier: syn::TraitBoundModifier::None,
                                                lifetimes: None,
                                                path,
                                            }) => Some(path.clone()),
                                            _ => None,
                                        }
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            }
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

#[test]
fn recognize_handle_type() {
    let t1 = is_handle(&syn::parse_str::<syn::Type>("Box<dyn Service1>").unwrap());
    let t2 = is_handle(&syn::parse_str::<syn::Type>("Box<dyn a::b::Service1>").unwrap());
    let t3 = is_handle(&syn::parse_str::<syn::Type>("Box<Service1>").unwrap());
    let t4 = is_handle(&syn::parse_str::<syn::Type>("Arc<dyn Service1>").unwrap());
    let t5 = is_handle(&syn::parse_str::<syn::Type>("Box<Box<dyn Service1>>").unwrap());

    assert_eq!(t1.unwrap(), syn::parse_str::<syn::Path>("Service1").unwrap());
    assert_eq!(t2.unwrap(), syn::parse_str::<syn::Path>("a::b::Service1").unwrap());
    assert!(t3.is_none());
    assert!(t4.is_none());
    assert!(t5.is_none());
}

/// In addition, it coverts str->String and [] -> Vec
pub fn is_ref(the_type: &syn::Type) -> Result<Option<syn::Type>, String> {
    if *the_type == syn::parse2::<syn::Type>(quote! {
        &str
    })
    .unwrap() {
        return Ok(Some(syn::parse2::<syn::Type>(quote! {
            String
        })
        .unwrap()))
    }

    match the_type {
        syn::Type::Reference(x) => {
            if x.lifetime.is_some() {
                return Err("Lifetime exists".to_owned())
            }
            if x.mutability.is_some() {
                return Err("Mutable".to_owned())
            }
            match *x.elem {
                syn::Type::Slice(_) => Ok(Some(syn::parse2::<syn::Type>(quote! {
                    Vec<_>
                })
                .unwrap())),
                _ => Ok(Some((*x.elem).clone()))
            }
        }
        _ => Ok(None),
    }
}

#[test]
fn recognize_ref() {
    let t = syn::parse_str::<syn::Type>("Vec<u32>").unwrap();
    assert!(is_ref(&t).unwrap().is_none());
    let t = syn::parse_str::<syn::Type>("&Vec<u32>").unwrap();
    let tu = syn::parse_str::<syn::Type>("Vec<u32>").unwrap();
    assert_eq!(is_ref(&t).unwrap().unwrap(), tu);
    let t = syn::parse_str::<syn::Type>("&i32").unwrap();
    let tu = syn::parse_str::<syn::Type>("i32").unwrap();
    assert_eq!(is_ref(&t).unwrap().unwrap(), tu);
    let t = syn::parse_str::<syn::Type>("&str").unwrap();
    let tu = syn::parse_str::<syn::Type>("String").unwrap();
    assert_eq!(is_ref(&t).unwrap().unwrap(), tu);
    let t = syn::parse_str::<syn::Type>("&[u8]").unwrap();
    let tu = syn::parse_str::<syn::Type>("Vec<_>").unwrap();
    assert_eq!(is_ref(&t).unwrap().unwrap(), tu);
    let t = syn::parse_str::<syn::Type>("&mut i32").unwrap();
    assert!(is_ref(&t).is_err())
}
