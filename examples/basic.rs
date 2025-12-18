use leptos::prelude::*;
use leptos_md::{CodeBlockTheme, Markdown, MarkdownOptions};

#[component]
fn App() -> impl IntoView {
    let markdown_content = r#"
# Hello Markdown!

This is a **bold** statement and this is *emphasized*.

## Code Example

```rust
fn main() {
    println!("Hello, world!");
}
```

## Lists

- Item 1
- Item 2
  - Nested item
- Item 3

1. First ordered item
2. Second ordered item
3. Third ordered item

## Task Lists

- [x] Completed task
- [ ] Incomplete task
- [x] Another completed task

## Links and Images

[Visit Rust](https://www.rust-lang.org/)

![Rust Logo](https://www.rust-lang.org/static/images/rust-logo-blk.svg)

## Tables

| Column 1 | Column 2 | Column 3 |
|----------|----------|----------|
| Row 1    | Data     | More data |
| Row 2    | Info     | More info |

## Blockquotes

> This is a blockquote with some important information.
> 
> It can span multiple paragraphs.

## Inline Elements

This paragraph contains `inline code`, **bold text**, *italic text*, and ~~strikethrough text~~.

---

That's all folks!
"#;

    let options = MarkdownOptions {
        enable_gfm: true,
        code_theme: Some(CodeBlockTheme::GitHub),
        syntax_highlighting_language_classes: true,
        open_links_in_new_tab: true,
        allow_raw_html: true,
        use_explicit_classes: false,
    };

    view! {
        <div class="min-h-screen bg-white dark:bg-gray-900">
            <div class="container mx-auto px-4 py-8 max-w-4xl">
                <header class="mb-8">
                    <h1 class="text-4xl font-bold text-gray-900 dark:text-gray-100 mb-2">
                        "Leptos Markdown with Tailwind"
                    </h1>
                    <p class="text-gray-600 dark:text-gray-400">
                        "Beautiful markdown rendering with Tailwind CSS utility classes"
                    </p>
                </header>

                <Markdown
                    content=markdown_content.to_string()
                    class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 border border-gray-200 dark:border-gray-700".to_string()
                    options=options
                />

                <footer class="mt-8 pt-6 border-t border-gray-200 dark:border-gray-700">
                    <p class="text-sm text-gray-500 dark:text-gray-400 text-center">
                        "Powered by Leptos + Tailwind CSS"
                    </p>
                </footer>
            </div>
        </div>
    }
}

fn main() {
    // This example demonstrates the API usage.
    // To run in a browser, you would need a full Leptos app setup with trunk or cargo-leptos.
    println!("Leptos Markdown Library Example");
    println!();
    println!("This example shows the component API. To run in a browser:");
    println!("1. Create a new Leptos project with `cargo leptos new`");
    println!("2. Add `leptos-md` to your dependencies");
    println!("3. Use the <Markdown /> component in your app");
    println!();
    println!("See the README for full usage examples.");
}
