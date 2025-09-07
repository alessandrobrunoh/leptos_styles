# leptos_styles - Scoped CSS Styling for Leptos Components

leptos_styles is a Rust procedural macro library that provides scoped CSS styling for Leptos components through an attribute macro syntax. The library reads CSS/SCSS files at compile time and generates unique component-scoped styles.

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

### Bootstrap and Build
- Ensure Rust and Cargo are installed and up to date:
  - `rustc --version` (should be 1.89.0 or newer)
  - `cargo --version` (should be 1.89.0 or newer)
- Build the library:
  - `cargo build` -- takes ~9 seconds for clean build. Set timeout to 30+ seconds for safety.
  - `cargo build --release` -- takes ~9 seconds for optimized build. Set timeout to 30+ seconds.
  - `cargo check` -- takes ~4 seconds for quick compilation check. Set timeout to 15+ seconds.

### Testing
- Run tests:
  - `cargo test` -- takes <2 seconds. Currently has 0 unit tests and 1 ignored doc test.
- CRITICAL: Since this is a proc-macro library, functional validation requires creating test projects that use the macro.

### Validation and Quality Assurance
- Always run code quality tools before committing:
  - `cargo fmt` -- takes <1 second. Formats all Rust code.
  - `cargo clippy` -- takes ~5 seconds. Checks for common mistakes and improvements.
  - `cargo doc --no-deps` -- takes ~4 seconds. Generates documentation.
- Create a test project to validate functionality (see Functional Validation section below).

## Functional Validation

Since this is a procedural macro library, you MUST validate changes by creating a test project that uses the macro:

### Create Test Project
```bash
# Create test directory (use /tmp to avoid committing test files)
mkdir -p /tmp/leptos_styles_test/src

# Create Cargo.toml
cat > /tmp/leptos_styles_test/Cargo.toml << 'EOF'
[package]
name = "leptos_styles_test"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = "0.6.12"
leptos_styles = { path = "/home/runner/work/leptos_styles/leptos_styles" }
EOF

# Create test CSS file
cat > /tmp/leptos_styles_test/test.css << 'EOF'
p {
    color: blue;
    font-size: 1.2em;
}
EOF

# Create test SCSS file
cat > /tmp/leptos_styles_test/test.scss << 'EOF'
$primary-color: blue;
$font-size: 1.2em;

p {
    color: $primary-color;
    font-size: $font-size;
    
    &:hover {
        color: darken($primary-color, 20%);
    }
}
EOF

# Create main.rs that uses the macro
cat > /tmp/leptos_styles_test/src/main.rs << 'EOF'
use leptos::*;
use leptos_styles::styles;

#[component]
#[styles("../test.css")]
pub fn TestComponent() -> impl IntoView {
    view! {
        <p>"Hello World!"</p>
    }
}

#[component]
#[styles("../test.scss")]
pub fn TestComponentScss() -> impl IntoView {
    view! {
        <p>"Hello SCSS World!"</p>
    }
}

fn main() {
    println!("Test components compiled successfully");
}
EOF
```

### Validate Test Project
```bash
cd /tmp/leptos_styles_test
cargo build  # Should compile successfully, takes ~30-60 seconds for first build
```

CRITICAL: ALWAYS run this functional validation after making ANY changes to the macro code. The macro must correctly process both CSS and SCSS files.

## Development Workflow

### Making Changes to the Macro
1. Edit `src/lib.rs` to modify the `#[styles]` procedural macro
2. Run `cargo check` to verify syntax (~4 seconds)
3. Run `cargo clippy` to check for issues (~5 seconds)
4. Create and build test project (see Functional Validation above)
5. Run `cargo fmt` before committing
6. Run `cargo test` to ensure no regressions

### Common File Paths
- Main macro implementation: `src/lib.rs`
- Project configuration: `Cargo.toml`
- Documentation: `README.md`

## Technical Details

### Build System
- This is a proc-macro crate (`proc-macro = true` in Cargo.toml)
- Uses `grass` crate for SCSS compilation
- Uses `syn`, `quote`, and `proc-macro2` for macro implementation
- Optional `leptos` dependency for integration

### Macro Functionality
- Reads CSS/SCSS files at compile time using `include_str!`
- Generates unique component IDs based on filename hash
- Wraps component output in a div with the unique ID
- Injects scoped styles using CSS ID selectors
- Supports both CSS and SCSS file formats

### Dependencies
Key dependencies and their purposes:
- `syn` + `quote` + `proc-macro2`: Macro implementation
- `grass`: SCSS compilation
- `leptos`: Optional, for Leptos integration

## Troubleshooting

### Build Issues
- If build fails, ensure CSS/SCSS file paths are correct relative to `CARGO_MANIFEST_DIR`
- Macro errors will show at compile time of projects using the macro, not when building the library itself
- Check that included CSS/SCSS files exist and are readable

### Validation Issues
- If test project fails to build, check file paths in `#[styles("path")]` attribute
- Ensure CSS/SCSS syntax is valid
- Verify leptos version compatibility

### Performance Notes
- Build times are fast (~9 seconds for clean builds)
- No long-running operations - all builds complete quickly
- Incremental builds are very fast (<1 second)

## Repository Structure
```
leptos_styles/
├── src/
│   └── lib.rs          # Main macro implementation
├── target/             # Build artifacts (gitignored)
├── Cargo.toml          # Project configuration
├── README.md           # Project documentation
└── .gitignore          # Git ignore rules
```

## References
- Main documentation: `README.md`
- Generated docs: Run `cargo doc --no-deps` and open `target/doc/leptos_styles/index.html`
- Leptos documentation: https://leptos.dev/