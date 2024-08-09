// #[proxy_error(crate::Error)]

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{ItemEnum, Path, Type};

#[proc_macro_attribute]
pub fn proxy_error(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = proc_macro2::TokenStream::from(args);
    let item = proc_macro2::TokenStream::from(item);

    // println!("{:?}", args);
    // println!("{:?}", item);

    // .filter_map(|x| syn::parse::<syn::Ident>(x.into()).ok());

    // let target = syn::parse2::<Path>(args.next().unwrap().into()).expect("can't parse target");
    let target = syn::parse2::<Type>(
        args.clone()
            .into_iter()
            .take_while(|x| x.to_string() != ",")
            .filter(|x| x.to_string() != ",")
            .collect::<TokenStream>(), // .tap(|x| println!("target = {:?}", x)),
    )
    .expect("can't parse target");
    // let target = args
    //     .clone()
    //     .into_iter()
    //     .take_while(|x| x.to_string() != ",")
    //     .filter(|x| x.to_string() != ",")
    //     .collect::<TokenStream>()
    //     .tap(|x| println!("target = {:?}", x));
    // TODO: optional
    let paths = syn::parse2::<Path>(
        args.into_iter()
            .skip_while(|x| x.to_string() != ",")
            .filter(|x| x.to_string() != ",")
            .collect::<TokenStream>(),
    )
    .expect("can't parse paths");

    // for target_segment in target.segments.iter() {
    //     println!("target = {:?}", target_segment.ident);
    // }
    // println!("paths = {:?}", paths);

    let item = syn::parse2::<ItemEnum>(item).expect("can't parse item");
    let item_ident = &item.ident;

    // println!("item = {:?}", item_ident);

    let impls_from = item.variants.iter().map(|member| {
        let member_ident = &member.ident;
        let attrs = member
            .attrs
            .iter()
            .filter(|attr| {
                attr.meta
                    .path()
                    .segments
                    .iter()
                    .all(|seg| seg.ident.to_string() != "error")
            })
            .collect::<Vec<_>>();
        // support unnamed only and one filed
        let member_field = member.fields.iter().next().unwrap();
        let member_field_ty = &member_field.ty;
        // let member_field_attrs = &*member_field.attrs;

        quote! {
            #( #attrs )*
            impl From<#member_field_ty> for #target {
                fn from(x: #member_field_ty) -> #target {
                    #item_ident::from(x).into()
                }
            }

            #( #attrs )*
            impl TryFrom<#target> for #member_field_ty {
                type Error = ();

                fn try_from(x: #target) -> Result<Self, Self::Error> {


                    match x {
                        #target::#paths(#item_ident::#member_ident(err)) => Ok(err),
                        _err => return Err(())
                    }
                }
            }

            #( #attrs )*
            impl<'a> TryFrom<&'a #target> for #member_field_ty {
                type Error = bool;

                fn try_from(x: &#target) -> Result<Self, Self::Error> {


                    match x {
                        #target::#paths(#item_ident::#member_ident(_err)) => Err(true),
                        _ => return Err(false)
                    }
                }
            }
        }
    });

    // quote! {
    //     #item

    //     $( impls_from )+
    // }
    // .into_token_stream()
    // .into()

    quote! {
        #item

        #( #impls_from )*
    }
    .into_token_stream()
    // .tap(|x| println!("\n\n{}", x))
    .into()
}
