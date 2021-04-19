extern crate proc_macro;

use proc_macro2::Literal;
use syn::{Data, DeriveInput, Lit, parse_macro_input, Attribute, Ident, Error};

#[proc_macro]
pub fn license(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let l = match parse_macro_input!(input as Lit) {
        Lit::Str(s) => s.value(),
        l => {
            return Error::new(l.span(), "Expected string literal")
                .to_compile_error()
                .into();
        },
    };
    let license = l + "\u{0}";
    let license_len = license.len();

    let license = Literal::byte_string(license.as_bytes());

    proc_macro::TokenStream::from(quote::quote! {
        #[no_mangle]
        #[link_section = "license"]
        static LICENSE: [u8; #license_len] = *#license;

        #[panic_handler]
        fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
            loop {}
        }

        #[lang = "eh_personality"]
        extern "C" fn eh_personality() {}
    })
}

#[proc_macro_derive(BpfApp, attributes(license, ringbuf, prog))]
pub fn derive_bpf_app(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { data, .. } = parse_macro_input!(input as DeriveInput);

    struct KernelTokens<T> {
        decl: T,
        new_field: T,
    }

    let kt = KernelTokens {
        decl: quote::quote! {},
        new_field: quote::quote! {},
    };

    fn process_attrib(
        attrs: &[Attribute],
        name_ident: &Ident,
    ) -> Option<KernelTokens<impl quote::ToTokens>> {
        struct AttributeRingbuf {
            size: usize,
        }

        impl syn::parse::Parse for AttributeRingbuf {
            fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
                let _: syn::Ident = input.parse()?;
                let _: syn::Token![=] = input.parse()?;
                let size = match input.parse()? {
                    Lit::Int(l) => l.base10_digits().parse().map_err(|e| {
                        syn::Error::new(l.span(), format_args!("Bad integer literal {}", e))
                    })?,
                    l => return Err(syn::Error::new(l.span(), "Expected integer literal")),
                };
                Ok(AttributeRingbuf { size })
            }
        }

        let attribute = attrs.first()?;
        let segment = attribute.path.segments.first()?;
        let ident_str = segment.ident.to_string();
        match ident_str.as_str() {
            "ringbuf" => {
                let AttributeRingbuf { size } = attribute.parse_args().ok()?;

                Some(KernelTokens {
                    decl: quote::quote! {
                        #[no_mangle]
                        #[link_section = ".maps"]
                        #[allow(non_upper_case_globals)]
                        static mut #name_ident: ebpf_kern::RingBuffer<#size> =
                            ebpf_kern::RingBuffer::new();
                    },
                    new_field: quote::quote! {
                        #name_ident: ebpf_kern::RingBufferRef::new(&mut #name_ident),
                    },
                })
            },
            "prog" => {
                if let Lit::Str(l) = attribute.parse_args().ok()? {
                    Some(KernelTokens {
                        decl: quote::quote! {
                            #[no_mangle]
                            #[link_section = #l]
                            fn #name_ident(
                                ctx: *const ebpf_kern::cty::c_void,
                            ) -> ebpf_kern::cty::c_int {
                                let mut app = unsafe { app_instance() };
                                let ctx = unsafe { ebpf_kern::Context::cast(ctx) };
                                match app.#name_ident(ctx) {
                                    Ok(()) => 0,
                                    Err(c) => c,
                                }
                            }
                        },
                        new_field: quote::quote! {
                            #name_ident: ebpf_kern::ProgRef::new(#name_ident),
                        },
                    })
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    let kt = match data {
        Data::Struct(data) => data.fields.into_iter().fold(kt, |kt, field| {
            let val = field.ident.unwrap();

            match process_attrib(&field.attrs, &val) {
                None => kt,
                Some(KernelTokens { decl, new_field }) => {
                    let KernelTokens {
                        decl: decl_a,
                        new_field: new_field_a,
                    } = kt;
                    KernelTokens {
                        decl: quote::quote! {
                            #decl_a #decl
                        },
                        new_field: quote::quote! {
                            #new_field_a #new_field
                        },
                    }
                },
            }
        }),
        _ => unimplemented!(),
    };

    let KernelTokens { decl, new_field } = kt;

    proc_macro::TokenStream::from(quote::quote! {
        unsafe fn app_instance() -> App {
            #decl

            App {
                #new_field
            }
        }
    })
}
