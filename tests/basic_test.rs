#[cfg(test)]
mod tests {
    use leptos_md::{
        render_markdown_string, render_markdown_with_options, CodeBlockTheme, MarkdownClasses,
        MarkdownOptions,
    };

    #[test]
    fn test_basic_markdown_rendering() {
        let markdown = "# Hello World\n\nThis is **bold** text.";
        let result = render_markdown_string(markdown);
        assert!(result.is_ok(), "Basic markdown should render successfully");
    }

    #[test]
    fn test_empty_markdown() {
        let result = render_markdown_string("");
        assert!(result.is_ok(), "Empty markdown should render successfully");
    }

    #[test]
    fn test_markdown_options_builder() {
        // Test that builder pattern works correctly
        let options = MarkdownOptions::new()
            .with_gfm(false)
            .with_code_theme(CodeBlockTheme::Dark)
            .with_language_classes(false)
            .with_new_tab_links(false)
            .with_allow_raw_html(false)
            .with_explicit_classes(true);

        assert!(!options.enable_gfm);
        assert_eq!(options.code_theme, Some(CodeBlockTheme::Dark));
        assert!(!options.syntax_highlighting_language_classes);
        assert!(!options.open_links_in_new_tab);
        assert!(!options.allow_raw_html);
        assert!(options.use_explicit_classes);
    }

    #[test]
    fn test_markdown_options_defaults() {
        let options = MarkdownOptions::new();
        assert!(options.enable_gfm, "GFM should be enabled by default");
        assert!(
            options.code_theme.is_some(),
            "Code theme should be set by default"
        );
        assert!(
            options.syntax_highlighting_language_classes,
            "Language classes should be enabled by default"
        );
        assert!(
            options.open_links_in_new_tab,
            "Links should open in new tab by default"
        );
        assert!(
            options.allow_raw_html,
            "Raw HTML should be allowed by default"
        );
        assert!(
            !options.use_explicit_classes,
            "Explicit classes should be disabled by default"
        );
    }

    #[test]
    fn test_gfm_features() {
        let markdown = r#"
# GitHub Flavored Markdown

- [x] Task lists work
- [ ] Unchecked item

| Column 1 | Column 2 |
|----------|----------|
| Data     | More data |

```rust
fn main() {
    println!("Hello, world!");
}
```

> This is a blockquote

~~Strikethrough text~~
"#;

        let options = MarkdownOptions::new()
            .with_gfm(true)
            .with_code_theme(CodeBlockTheme::GitHub);

        let result = render_markdown_with_options(markdown, options);
        assert!(result.is_ok());
    }

    #[test]
    fn test_links_and_images() {
        let markdown = r#"
[Visit Rust](https://www.rust-lang.org/)

![Alt text](https://example.com/image.png "Title text")
"#;

        let result = render_markdown_string(markdown);
        assert!(result.is_ok());
    }

    #[test]
    fn test_math_expressions() {
        let markdown = r#"
Inline math: $E = mc^2$

Display math:
$\int_{-\infty}^{\infty} e^{-x^2} dx = \sqrt{\pi}$
"#;

        let result = render_markdown_string(markdown);
        assert!(result.is_ok());
    }

    #[test]
    fn test_definition_lists() {
        let markdown = r#"
Term 1
: Definition for term 1

Term 2  
: Definition for term 2
"#;

        let result = render_markdown_string(markdown);
        assert!(result.is_ok());
    }

    #[test]
    fn test_tailwind_classes() {
        // Test that classes are defined and contain expected Tailwind utilities
        assert!(!MarkdownClasses::H1.is_empty());
        assert!(!MarkdownClasses::CODE_BLOCK.is_empty());
        assert!(!MarkdownClasses::TABLE.is_empty());
        assert!(
            MarkdownClasses::H1.contains("text-3xl"),
            "H1 should have text-3xl class"
        );
        assert!(
            MarkdownClasses::BLOCKQUOTE.contains("border-l-4"),
            "Blockquote should have border-l-4 class"
        );
        assert!(
            MarkdownClasses::LINK.contains("text-blue"),
            "Link should have blue text color"
        );
        assert!(
            MarkdownClasses::INLINE_CODE.contains("font-mono"),
            "Inline code should have monospace font"
        );
    }

    #[test]
    fn test_tailwind_classes_new_constants() {
        // Test the new constants added for explicit classes mode
        assert!(
            MarkdownClasses::EM.contains("italic"),
            "EM should have italic class"
        );
        assert!(
            MarkdownClasses::STRONG.contains("font-bold"),
            "STRONG should have font-bold class"
        );
        assert!(
            MarkdownClasses::DEL.contains("line-through"),
            "DEL should have line-through class"
        );
        assert!(!MarkdownClasses::DL.is_empty(), "DL should be defined");
        assert!(!MarkdownClasses::DT.is_empty(), "DT should be defined");
        assert!(!MarkdownClasses::DD.is_empty(), "DD should be defined");
        assert!(!MarkdownClasses::SUP.is_empty(), "SUP should be defined");
        assert!(!MarkdownClasses::SUB.is_empty(), "SUB should be defined");
        assert!(
            !MarkdownClasses::THEAD.is_empty(),
            "THEAD should be defined"
        );
        assert!(!MarkdownClasses::TR.is_empty(), "TR should be defined");
        assert!(!MarkdownClasses::TD.is_empty(), "TD should be defined");
        assert!(!MarkdownClasses::TH.is_empty(), "TH should be defined");
    }

    #[test]
    fn test_code_themes() {
        // Test that all code themes are distinct
        use leptos_md::get_code_theme_classes;

        let default = get_code_theme_classes(&CodeBlockTheme::Default);
        let dark = get_code_theme_classes(&CodeBlockTheme::Dark);
        let light = get_code_theme_classes(&CodeBlockTheme::Light);
        let github = get_code_theme_classes(&CodeBlockTheme::GitHub);
        let monokai = get_code_theme_classes(&CodeBlockTheme::Monokai);

        assert_ne!(default, dark, "Default and Dark themes should differ");
        assert_ne!(light, dark, "Light and Dark themes should differ");
        assert_ne!(github, monokai, "GitHub and Monokai themes should differ");

        // Check that themes contain expected patterns
        assert!(
            dark.contains("bg-gray-900"),
            "Dark theme should have dark background"
        );
        assert!(
            monokai.contains("272822"),
            "Monokai should have its signature color"
        );
    }

    #[test]
    fn test_render_with_explicit_classes() {
        let markdown = "# Hello\n\nWorld";
        let options = MarkdownOptions::new().with_explicit_classes(true);
        let result = render_markdown_with_options(markdown, options);
        assert!(
            result.is_ok(),
            "Rendering with explicit classes should succeed"
        );
    }

    #[test]
    fn test_render_without_code_theme() {
        let markdown = "```rust\nfn main() {}\n```";
        let options = MarkdownOptions::new().without_code_theme();
        assert!(
            options.code_theme.is_none(),
            "Code theme should be None after without_code_theme()"
        );
        let result = render_markdown_with_options(markdown, options);
        assert!(
            result.is_ok(),
            "Rendering without code theme should succeed"
        );
    }
}
