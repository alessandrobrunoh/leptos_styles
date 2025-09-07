use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

/// Applies scoped CSS styles from a file to a Leptos component.
///
/// Reads CSS from the specified file path, generates a unique component ID,
/// and wraps the component's view in a div with scoped styles.
///
/// # Arguments
///
/// * `path` - Path to the CSS file relative to `CARGO_MANIFEST_DIR`
///
/// # Example
///
/// ```ignore
/// use leptos::*;
/// use leptos_styles::styles;
///
/// #[component]
/// #[styles("src/my_component.css")]
/// pub fn MyComponent() -> impl IntoView {
///     view! { <p>"Styled content"</p> }
/// }
/// ```
#[proc_macro_attribute]
pub fn styles(attr: TokenStream, item: TokenStream) -> TokenStream {
    let path_lit = parse_macro_input!(attr as LitStr);
    let path_str = path_lit.value();
    let mut func = parse_macro_input!(item as ItemFn);

    let filename = std::path::Path::new(&path_str)
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("component");

    let mut hash: u32 = 5381;
    for byte in filename.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u32);
    }
    let unique_id = format!("{}{}", filename, hash % 10000);

    let original_body = func.block;

    let new_body = quote! {
        {
            let original_view = #original_body;
            leptos::view! {
                <style>{concat!("#", #unique_id, " { ", include_str!(#path_str), " }")}</style>
                <div id=#unique_id>
                    {original_view}
                </div>
            }
        }
    };

    let new_body_as_block: syn::Block = match syn::parse2(new_body) {
        Ok(block) => block,
        Err(e) => return e.to_compile_error().into(),
    };
    func.block = Box::new(new_body_as_block);

    quote! {
        #func
    }
    .into()
}
