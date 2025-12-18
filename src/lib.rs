//! # leptos-md
//!
//! A simple, signal-free Markdown renderer for [Leptos](https://leptos.dev) with Tailwind CSS styling.
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

use leptos::prelude::*;

mod components;
mod renderer;

pub use components::{
    get_code_theme_classes, get_enhanced_prose_classes, CodeBlockTheme, MarkdownClasses,
    MarkdownOptions, MarkdownStyles,
};
pub use renderer::MarkdownRenderer;

/// Main component for rendering Markdown content with Tailwind CSS styling
#[component]
pub fn Markdown(
    /// The markdown content as a string
    #[prop(into)]
    content: String,
    /// Optional CSS class for the wrapper (will be combined with Tailwind prose classes)
    #[prop(optional)]
    class: Option<String>,
    /// Markdown rendering options
    #[prop(optional)]
    options: Option<MarkdownOptions>,
) -> impl IntoView {
    let renderer = MarkdownRenderer::new(options.unwrap_or_default());

    match renderer.render(&content) {
        Ok(rendered_content) => {
            let base_classes = get_enhanced_prose_classes();
            let wrapper_class = match class {
                Some(c) => format!("{} {}", base_classes, c),
                None => base_classes.to_string(),
            };

            view! {
                <div class=wrapper_class>
                    {rendered_content}
                </div>
            }
            .into_any()
        }
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

/// Utility function to render markdown string directly to AnyView with Tailwind styling
pub fn render_markdown_string(content: &str) -> Result<AnyView, String> {
    let renderer = MarkdownRenderer::new(MarkdownOptions::default());
    renderer.render(content)
}

/// Utility function to render markdown with custom options and Tailwind styling
pub fn render_markdown_with_options(
    content: &str,
    options: MarkdownOptions,
) -> Result<AnyView, String> {
    let renderer = MarkdownRenderer::new(options);
    renderer.render(content)
}
