//! # leptos-md
//!
//! A simple, signal-free Markdown renderer for [Leptos](https://leptos.dev). Styled with Tailwind
//! out of the box, but every element's class is configurable via [`MarkdownClassMap`], so it works
//! with any CSS framework.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use leptos::prelude::*;
//! use leptos_md::Markdown;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <Markdown content="# Hello World\n\nThis is **markdown**!" />
//!     }
//! }
//! ```
//!
//! ## Features
//!
//! - **Dead simple API** - `<Markdown content=md />` and you're done
//! - **Beautiful by default** - Tailwind prose styling with dark mode support
//! - **Any CSS framework** - Override per-element classes with [`MarkdownClassMap`]
//! - **GitHub Flavored Markdown** - Tables, task lists, strikethrough, footnotes
//! - **Code block themes** - Built-in Tailwind themes (GitHub, Monokai, Dark, Light)
//! - **External highlighter ready** - Outputs `language-xxx` classes for Prism.js, highlight.js
//! - **SSR ready** - Works seamlessly with Leptos server-side rendering
//!
//! ## Customization
//!
//! Use [`MarkdownOptions`] for fine-grained control:
//!
//! ```rust,ignore
//! use leptos_md::{Markdown, MarkdownOptions, CodeBlockTheme};
//!
//! let options = MarkdownOptions::new()
//!     .with_gfm(true)
//!     .with_code_theme(CodeBlockTheme::GitHub)
//!     .with_language_classes(true)
//!     .with_new_tab_links(true);
//!
//! view! {
//!     <Markdown content="# Hello" options=options />
//! }
//! ```
//!
//! ## Custom classes
//!
//! Pick a preset (`prose()` is the default, `tailwind()` puts utility classes on every element,
//! `none()` emits no classes) or override individual elements with struct-update syntax:
//!
//! ```rust,ignore
//! use leptos_md::{Markdown, MarkdownOptions, MarkdownClassMap};
//!
//! let options = MarkdownOptions::new().with_classes(MarkdownClassMap {
//!     wrapper: "content".into(),
//!     h1: "display-4".into(),
//!     table: "table table-striped".into(),
//!     ..MarkdownClassMap::none()
//! });
//!
//! view! {
//!     <Markdown content="# Hello" options=options />
//! }
//! ```

use leptos::prelude::*;

mod components;
mod renderer;

pub use components::{
    get_code_theme_classes, get_enhanced_prose_classes, CodeBlockTheme, MarkdownClassMap,
    MarkdownClasses, MarkdownOptions, MarkdownStyles,
};
pub use renderer::MarkdownRenderer;

/// Main component for rendering Markdown content
#[component]
pub fn Markdown(
    /// The markdown content as a string
    #[prop(into)]
    content: String,
    /// Optional CSS class for the wrapper (appended to `options.classes.wrapper`)
    #[prop(optional)]
    class: Option<String>,
    /// Markdown rendering options
    #[prop(optional)]
    options: Option<MarkdownOptions>,
) -> impl IntoView {
    let options = options.unwrap_or_default();
    let wrapper_class = [options.classes.wrapper.as_str(), class.as_deref().unwrap_or("")]
        .iter()
        .filter(|c| !c.is_empty())
        .copied()
        .collect::<Vec<_>>()
        .join(" ");
    let renderer = MarkdownRenderer::new(options);

    match renderer.render(&content) {
        Ok(rendered_content) => view! {
            <div class=wrapper_class>
                {rendered_content}
            </div>
        }
        .into_any(),
        Err(err) => {
            leptos::logging::error!("Failed to render markdown: {}", err);
            view! {
                <div class="bg-red-50 dark:bg-red-950/30 border border-red-200 dark:border-red-800 rounded-lg p-4 text-red-800 dark:text-red-200">
                    <p class="font-medium">"Failed to render markdown content"</p>
                    <p class="text-sm mt-1">{err}</p>
                </div>
            }.into_any()
        }
    }
}

/// Utility function to render markdown string directly to AnyView with default options
pub fn render_markdown_string(content: &str) -> Result<AnyView, String> {
    let renderer = MarkdownRenderer::new(MarkdownOptions::default());
    renderer.render(content)
}

/// Utility function to render markdown with custom options and styling
pub fn render_markdown_with_options(
    content: &str,
    options: MarkdownOptions,
) -> Result<AnyView, String> {
    let renderer = MarkdownRenderer::new(options);
    renderer.render(content)
}
