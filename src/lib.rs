use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

/// Applies CSS styles from a specified file to a Leptos component.
///
/// This attribute macro transforms a Leptos component function by:
/// 1. Reading the content of a CSS file specified by `path`.
/// 2. Generating a unique ID for the component based on the CSS filename (stem).
/// 3. Wrapping the original component's view output within a `<div>` element
///    whose `id` attribute is set to the generated unique ID.
/// 4. Injecting a `<style>` tag into the component's view, containing the CSS
///    from the file, automatically scoped to the generated unique ID using a
///    CSS ID selector.
///
/// The CSS in the file will be applied to the wrapped `<div>` and its children
/// using the ID selector. This mechanism helps in scoping styles to a specific
/// component instance.
///
/// # Arguments
///
/// * `path`: A string literal representing the path to the CSS file.
///           This path is resolved relative to the `CARGO_MANIFEST_DIR`
///           environment variable, which typically points to the root of your crate.
///
/// # Example
///
/// Assuming you have a CSS file at `src/my_component.css`:
/// ```css
/// p {
///     color: blue;
///     font-size: 1.2em;
/// }
/// ```
///
/// You can apply these styles to a Leptos component function like this:
///
/// ```ignore
/// use leptos::*;
/// // Assuming your proc-macro crate is named `my_macros`
/// use my_macros::styles;
///
/// #[component]
/// #[styles("src/my_component.css")]
/// pub fn MyComponent() -> impl IntoView {
///     view! {
///         <p>"Hello from MyComponent!"</p>
///     }
/// }
/// ```
///
/// This would conceptually expand to a view structure similar to:
/// ```html
/// <style>
///     #my_component1234 { /* The content of src/my_component.css is included here */
///         p {
///             color: blue;
///             font-size: 1.2em;
///         }
///     }
/// </style>
/// <div id="my_component1234">
///     <p>Hello from MyComponent!</p>
/// </div>
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
