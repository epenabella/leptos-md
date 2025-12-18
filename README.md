# leptos-md

[![Crates.io](https://img.shields.io/crates/v/leptos-md.svg)](https://crates.io/crates/leptos-md)
[![Documentation](https://docs.rs/leptos-md/badge.svg)](https://docs.rs/leptos-md)
[![Leptos 0.8](https://img.shields.io/badge/Leptos-0.8-blue.svg)](https://leptos.dev)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)

**Markdown rendering for Leptos 0.8+, beautifully styled out of the box.**

A lightweight Markdown-to-view component for [Leptos](https://leptos.dev) with built-in Tailwind CSS styling, dark mode support, and GitHub Flavored Markdown.

> **Leptos Version:** This library targets **Leptos 0.8**. For older Leptos versions, check the [releases](https://github.com/epenabella/leptos-md/releases) for compatible versions.

## Features

- **Dead simple API** - `<Markdown content=md />` and you're done
- **Beautiful by default** - Tailwind prose styling with dark mode support
- **GitHub Flavored Markdown** - Tables, task lists, strikethrough, footnotes
- **Code block themes** - Built-in Tailwind themes (GitHub, Monokai, Dark, Light)
- **External highlighter ready** - Outputs `language-xxx` classes for Prism.js, highlight.js, etc.
- **Configurable** - Builder pattern for fine-grained control
- **SSR/SSG ready** - Works with server-side rendering and static site generation
- **Zero JavaScript** - Pure Rust, renders to static HTML

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
leptos-md = "0.1"
leptos = "0.8"
```

### Feature Flags

| Feature | Description |
|---------|-------------|
| `default` | Standard build (no SIMD) |
| `simd` | Enable SIMD acceleration for markdown parsing |
| `full` | All features including SIMD |

For faster parsing on supported platforms:

```toml
leptos-md = { version = "0.1", features = ["simd"] }
```

## Quick Start

```rust
use leptos::prelude::*;
use leptos_md::Markdown;

#[component]
fn App() -> impl IntoView {
    view! {
        <Markdown content="# Hello World\n\nThis is **markdown**!" />
    }
}
```

That's it! The component handles parsing, styling, and dark mode automatically.

## Customization

Use `MarkdownOptions` for fine-grained control:

```rust
use leptos_md::{Markdown, MarkdownOptions, CodeBlockTheme};

#[component]
fn App() -> impl IntoView {
    let options = MarkdownOptions::new()
        .with_gfm(true)                           // GitHub Flavored Markdown
        .with_code_theme(CodeBlockTheme::GitHub)  // Code block theme (Tailwind styling)
        .with_language_classes(true)              // Emit language-xxx classes for syntax highlighters
        .with_new_tab_links(true)                 // Open links in new tab
        .with_allow_raw_html(true)                // Render raw HTML in markdown
        .with_explicit_classes(false);            // Use prose classes (default)

    view! {
        <Markdown
            content="# My Post\n\n```rust\nfn main() {}\n```"
            options=options
        />
    }
}
```

## Supported Markdown Features

| Feature | Syntax | Supported |
|---------|--------|-----------|
| Headings | `# H1` to `###### H6` | Yes |
| Bold | `**text**` | Yes |
| Italic | `*text*` | Yes |
| Strikethrough | `~~text~~` | Yes |
| Links | `[text](url)` | Yes |
| Images | `![alt](url)` | Yes |
| Code (inline) | `` `code` `` | Yes |
| Code blocks | ` ```lang ` | Yes |
| Blockquotes | `> quote` | Yes |
| Ordered lists | `1. item` | Yes |
| Unordered lists | `- item` | Yes |
| Task lists | `- [x] done` | Yes |
| Tables | GFM tables | Yes |
| Footnotes | `[^1]` | Yes |
| Horizontal rules | `---` | Yes |
| Raw HTML | `<div>` | Configurable |

## Code Block Themes

These are **Tailwind-based background/frame themes** for code blocks. They style the container (`<pre>`) with colors and borders — they do not perform token-level syntax highlighting.

| Theme | Description |
|-------|-------------|
| `CodeBlockTheme::Default` | Adapts to light/dark mode |
| `CodeBlockTheme::Dark` | Dark background, always |
| `CodeBlockTheme::Light` | Light background, always |
| `CodeBlockTheme::GitHub` | GitHub's code styling |
| `CodeBlockTheme::Monokai` | Classic Monokai colors |
| `None` (via `.without_code_theme()`) | No styling, for use with external highlighters |

```rust
// Use a built-in Tailwind theme
let options = MarkdownOptions::new()
    .with_code_theme(CodeBlockTheme::Monokai);

// Or disable themes entirely (useful with external highlighters)
let options = MarkdownOptions::new()
    .without_code_theme();
```

## Syntax Highlighting with External Libraries

`leptos-md` outputs `language-xxx` classes on code blocks (e.g., `language-rust`, `language-javascript`). These classes are automatically recognized by popular syntax highlighting libraries.

### Using Prism.js

Prism is a lightweight, extensible syntax highlighter.

Add to your HTML `<head>`:

```html
<link rel="stylesheet" href="https://unpkg.com/prismjs/themes/prism-tomorrow.min.css" />
```

Add before `</body>`:

```html
<script src="https://unpkg.com/prismjs/prism.js"></script>
<script src="https://unpkg.com/prismjs/components/prism-rust.min.js"></script>
```

📖 [Prism.js Documentation](https://prismjs.com/)

### Using highlight.js

highlight.js supports 190+ languages with automatic language detection.

Add to your HTML `<head>`:

```html
<link rel="stylesheet" href="https://unpkg.com/highlight.js@11/styles/github-dark.min.css" />
```

Add before `</body>`:

```html
<script src="https://unpkg.com/highlight.js@11/lib/common.min.js"></script>
<script>hljs.highlightAll();</script>
```

📖 [highlight.js Documentation](https://highlightjs.org/)

### Recommended Configuration

When using external highlighters, disable the built-in Tailwind theme to avoid style conflicts:

```rust
let options = MarkdownOptions::new()
    .without_code_theme()        // No Tailwind theme (let highlighter handle it)
    .with_language_classes(true); // Emit language-xxx classes (default)
```

## MarkdownOptions Reference

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `enable_gfm` | `bool` | `true` | Enable GitHub Flavored Markdown |
| `code_theme` | `Option<CodeBlockTheme>` | `Some(Default)` | Tailwind theme for code blocks (`None` = no styling) |
| `syntax_highlighting_language_classes` | `bool` | `true` | Add `language-xxx` classes for external highlighters |
| `open_links_in_new_tab` | `bool` | `true` | Add `target="_blank"` to links |
| `allow_raw_html` | `bool` | `true` | Render raw HTML in markdown |
| `use_explicit_classes` | `bool` | `false` | Use explicit Tailwind classes instead of prose |

All options use a builder pattern with `#[must_use]` for safety:

```rust
MarkdownOptions::new()
    .with_gfm(true)
    .with_code_theme(CodeBlockTheme::GitHub)  // or .without_code_theme()
    .with_language_classes(true)
    .with_new_tab_links(false)
    .with_allow_raw_html(true)
    .with_explicit_classes(false)
```

### Explicit Classes Mode

By default, `leptos-md` relies on Tailwind's `prose` classes for styling. If you're not using the `@tailwindcss/typography` plugin or want full control over each element's styling, enable explicit classes:

```rust
let options = MarkdownOptions::new()
    .with_explicit_classes(true);  // Apply MarkdownClasses::* directly
```

When enabled, elements receive explicit Tailwind utility classes from `MarkdownClasses` constants (e.g., `MarkdownClasses::H1`, `MarkdownClasses::PARAGRAPH`). You can customize these by overriding the CSS or using Tailwind's `@apply` directive.

## Why leptos-md?

| Feature | leptos-md | Raw HTML | Other solutions |
|---------|-----------|----------|-----------------|
| Type-safe | Yes | No | Varies |
| Tailwind styling | Built-in | Manual | Usually not |
| Dark mode | Automatic | Manual | Varies |
| SSR/SSG support | Yes | Yes | Varies |
| Bundle size | Minimal | N/A | Often larger |
| Leptos 0.8 | Yes | N/A | Often outdated |

**leptos-md** is designed specifically for Leptos applications. It renders Markdown directly to Leptos views (not raw HTML strings), includes beautiful Tailwind styling out of the box, and stays up-to-date with the latest Leptos releases.

## Acknowledgments

This project was inspired by [rambip/rust-web-markdown](https://github.com/rambip/rust-web-markdown) (formerly rambip/leptos-markdown).

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.