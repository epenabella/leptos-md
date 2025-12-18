use leptos_md::{
    render_markdown_string, render_markdown_with_options, CodeBlockTheme, MarkdownOptions,
};

fn main() {
    println!("=== Leptos Markdown Library Demo ===\n");

    // Basic markdown rendering
    let simple_markdown = "# Hello World\n\nThis is **bold** and this is *italic* text.";

    match render_markdown_string(simple_markdown) {
        Ok(_) => println!("✅ Basic markdown rendering: SUCCESS"),
        Err(e) => println!("❌ Basic markdown rendering failed: {}", e),
    }

    // GFM features
    let gfm_markdown = r#"
# GitHub Flavored Markdown Test

## Task Lists
- [x] Completed task
- [ ] Incomplete task

## Tables
| Feature | Status |
|---------|--------|
| Tables  | ✅ Working |
| Tasks   | ✅ Working |

## Code Blocks
```rust
fn main() {
    println!("Hello, Rust!");
}
```

## Strikethrough
This is ~~incorrect~~ correct.
"#;

    let options = MarkdownOptions::new()
        .with_gfm(true)
        .with_code_theme(CodeBlockTheme::GitHub)
        .with_language_classes(true)
        .with_new_tab_links(true);

    match render_markdown_with_options(gfm_markdown, options) {
        Ok(_) => println!("✅ GFM features rendering: SUCCESS"),
        Err(e) => println!("❌ GFM features rendering failed: {}", e),
    }

    // Math expressions
    let math_markdown = r#"
# Math Support

Inline math: $E = mc^2$

Display math:
$$\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}$$
"#;

    match render_markdown_string(math_markdown) {
        Ok(_) => println!("✅ Math expressions rendering: SUCCESS"),
        Err(e) => println!("❌ Math expressions rendering failed: {}", e),
    }

    // Links and images
    let media_markdown = r#"
# Links and Images

[Visit Rust Website](https://www.rust-lang.org/)

![Rust Logo](https://www.rust-lang.org/logos/rust-logo-512x512.png "Rust Programming Language")
"#;

    match render_markdown_string(media_markdown) {
        Ok(_) => println!("✅ Links and images rendering: SUCCESS"),
        Err(e) => println!("❌ Links and images rendering failed: {}", e),
    }

    println!("\n=== All tests completed! ===");
    println!("\nTo run the web example:");
    println!("cargo run --example basic --features csr");
    println!("\nTo run tests:");
    println!("cargo test");
}
