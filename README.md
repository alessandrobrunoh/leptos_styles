# leptos_styles

A macro library for scoped CSS styling in Leptos components.
This library provides scoped CSS styling using an attribute macro syntax similar to `#[component]`. I created it this way because I liked the consistency with Leptos's existing attribute macro pattern, rather than using function-like macros such as `styles!("")`.

## How it works

The `#[styles]` attribute macro automatically scopes CSS to individual Leptos components by:

1. **Reading CSS files** at compile time from the specified path
2. **Generating unique IDs** based on the CSS filename
3. **Wrapping components** in a div with the unique ID
4. **Injecting scoped styles** using CSS ID selectors

This ensures your component styles don't leak or conflict with other components.

## Usage

```rust
use leptos::*;
use leptos_styles::styles;

#[component]
#[styles("src/my_component.css")]
pub fn MyComponent() -> impl IntoView {
    view! {
        <p>"Hello World!"</p>
    }
}
```

With `src/my_component.css`:
```css
p {
    color: blue;
    font-size: 1.2em;
}
```

This automatically generates:
- A unique ID like `my_component1234`
- Scoped CSS: `#my_component1234 { /* your CSS */ }`
- Wrapped output: `<div id="my_component1234"><p>Hello World!</p></div>`

## Features

- [x] Compile-time SCSS injection
- [x] Automatic style scoping
- [x] Zero runtime overhead
- [x] Works with any SCSS file
- [x] Consistent with `#[component]` syntax
- [ ] Need to test on CSS file
