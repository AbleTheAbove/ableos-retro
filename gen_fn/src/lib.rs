extern crate proc_macro;

use proc_macro::{Literal, TokenStream, TokenTree};
use quote::{TokenStreamExt, __private::Span, format_ident, quote};
use syn::Ident;

#[proc_macro]
pub fn gen_fn(args: TokenStream) -> TokenStream {
	let mut a: Option<Literal> = None;
	let mut b: Option<Literal> = None;
	let mut name: Option<proc_macro::Ident> = None;
	let mut remaining: Vec<TokenTree> = vec![];

	for arg in args.into_iter() {
		match arg.clone() {
			TokenTree::Literal(literal) => {
				if a.is_none() {
					a = Some(literal);
				} else if b.is_none() {
					b = Some(literal);
				} else {
					remaining.push(arg.clone());
				}
			}
			TokenTree::Ident(ident) => {
				if name.is_none() {
					name = Some(ident);
				} else {
					remaining.push(arg.clone());
				}
			}
			_ => {
				remaining.push(arg.clone());
			}
		}
	}

	let mut token_buf = quote![];

	match (a, b, name) {
		(Some(a), Some(b), Some(name)) => {
			let a = usize::from_str_radix(&a.to_string(), 10).unwrap();
			let b = usize::from_str_radix(&b.to_string(), 10).unwrap();
			let name = name.to_string();
			for i in a..b {
				let name = format_ident![
					"_{}",
					Ident::new(&format!["{}{}", name, i], Span::call_site())
				];

				token_buf.append_all(quote! {
					extern "x86-interrupt" fn #name(stack_frame: InterruptStackFrame) {
						info!["Here"];
						unsafe {
							pic::PICS.lock().notify_end_of_interrupt(#i as u8);
						}
					}
				})
			}
		}
		_ => {}
	}

	token_buf.into()
}

#[proc_macro]
pub fn gen_name(args: TokenStream) -> TokenStream {
	let mut a: Option<Literal> = None;
	let mut b: Option<Literal> = None;
	let mut name: Option<proc_macro::Ident> = None;
	let mut remaining: Vec<TokenTree> = vec![];

	for arg in args.into_iter() {
		match arg.clone() {
			TokenTree::Literal(literal) => {
				if a.is_none() {
					a = Some(literal);
				} else if b.is_none() {
					b = Some(literal);
				} else {
					remaining.push(arg.clone());
				}
			}
			TokenTree::Ident(ident) => {
				if name.is_none() {
					name = Some(ident);
				} else {
					remaining.push(arg.clone());
				}
			}
			_ => {
				remaining.push(arg.clone());
			}
		}
	}

	let mut token_buf = quote![];

	match (a, b, name) {
		(Some(a), Some(b), Some(name)) => {
			let a = usize::from_str_radix(&a.to_string(), 10).unwrap();
			let b = usize::from_str_radix(&b.to_string(), 10).unwrap();
			let name = name.to_string();
			for i in a..b {
				let name = format_ident![
					"_{}",
					Ident::new(&format!["{}{}", name, i], Span::call_site())
				];

				token_buf.append_all(quote! {
										idt[#i].set_handler_fn(#name);
				})
			}
		}
		_ => {}
	}

	token_buf.into()
}
