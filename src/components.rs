use leptos::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum CodeBlockTheme {
    #[default]
    Default,
    Dark,
    Light,
    GitHub,
    Monokai,
}

#[derive(Clone, Debug)]
pub struct MarkdownOptions {
    pub enable_gfm: bool,
    /// Code block theme. `Some(theme)` applies Tailwind styling, `None` outputs no theme classes.
    pub code_theme: Option<CodeBlockTheme>,
    /// Whether to emit `language-xxx` classes on code blocks (for external syntax highlighters).
    pub syntax_highlighting_language_classes: bool,
    pub open_links_in_new_tab: bool,
    pub allow_raw_html: bool,
    /// Use explicit Tailwind utility classes on each element instead of relying on prose.
    /// When `false` (default), relies on Tailwind's `prose` classes for styling.
    /// When `true`, applies `MarkdownClasses::*` constants directly to elements.
    pub use_explicit_classes: bool,
}

impl Default for MarkdownOptions {
    fn default() -> Self {
        Self {
            enable_gfm: true,
            code_theme: Some(CodeBlockTheme::default()),
            syntax_highlighting_language_classes: true,
            open_links_in_new_tab: true,
            allow_raw_html: true,
            use_explicit_classes: false,
        }
    }
}

impl MarkdownOptions {
    /// Create a new MarkdownOptions with default values
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable or disable GitHub Flavored Markdown features
    #[must_use]
    pub fn with_gfm(mut self, enable: bool) -> Self {
        self.enable_gfm = enable;
        self
    }

    /// Set the code block theme (applies Tailwind styling)
    #[must_use]
    pub fn with_code_theme(mut self, theme: CodeBlockTheme) -> Self {
        self.code_theme = Some(theme);
        self
    }

    /// Disable code block theme styling (useful when using external syntax highlighters)
    #[must_use]
    pub fn without_code_theme(mut self) -> Self {
        self.code_theme = None;
        self
    }

    /// Enable or disable `language-xxx` classes on code blocks
    #[must_use]
    pub fn with_language_classes(mut self, enable: bool) -> Self {
        self.syntax_highlighting_language_classes = enable;
        self
    }

    /// Configure whether links open in new tabs
    #[must_use]
    pub fn with_new_tab_links(mut self, enable: bool) -> Self {
        self.open_links_in_new_tab = enable;
        self
    }

    /// Configure whether raw HTML in markdown is rendered
    #[must_use]
    pub fn with_allow_raw_html(mut self, enable: bool) -> Self {
        self.allow_raw_html = enable;
        self
    }

    /// Use explicit Tailwind utility classes on each element instead of relying on prose.
    /// When `false` (default), relies on Tailwind's `prose` classes for styling.
    /// When `true`, applies `MarkdownClasses::*` constants directly to elements.
    #[must_use]
    pub fn with_explicit_classes(mut self, enable: bool) -> Self {
        self.use_explicit_classes = enable;
        self
    }
}

/// Tailwind CSS class names for markdown elements
pub struct MarkdownClasses;

impl MarkdownClasses {
    // Base wrapper
    pub const CONTENT: &'static str =
        "leptos-mdx-content prose prose-gray max-w-none dark:prose-invert";

    // Headings
    pub const H1: &'static str =
        "text-3xl font-bold text-gray-900 dark:text-gray-100 mt-6 mb-4 first:mt-0";
    pub const H2: &'static str =
        "text-2xl font-semibold text-gray-900 dark:text-gray-100 mt-5 mb-3";
    pub const H3: &'static str = "text-xl font-semibold text-gray-900 dark:text-gray-100 mt-4 mb-2";
    pub const H4: &'static str = "text-lg font-medium text-gray-900 dark:text-gray-100 mt-3 mb-2";
    pub const H5: &'static str = "text-base font-medium text-gray-900 dark:text-gray-100 mt-3 mb-2";
    pub const H6: &'static str = "text-sm font-medium text-gray-600 dark:text-gray-400 mt-3 mb-2";

    // Text elements
    pub const PARAGRAPH: &'static str = "mb-4 leading-relaxed text-gray-700 dark:text-gray-300";
    pub const BLOCKQUOTE: &'static str = "border-l-4 border-blue-500 pl-4 py-2 my-4 bg-blue-50 dark:bg-blue-950/30 text-gray-700 dark:text-gray-300 italic";

    // Code
    pub const INLINE_CODE: &'static str = "bg-gray-100 dark:bg-gray-800 text-gray-800 dark:text-gray-200 px-1.5 py-0.5 rounded text-sm font-mono";
    pub const CODE_BLOCK: &'static str = "bg-gray-50 dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded-lg p-4 my-4 overflow-x-auto";
    pub const CODE_BLOCK_CODE: &'static str =
        "font-mono text-sm leading-relaxed text-gray-800 dark:text-gray-200";

    // Lists
    pub const UL: &'static str =
        "list-disc list-inside mb-4 space-y-1 text-gray-700 dark:text-gray-300";
    pub const OL: &'static str =
        "list-decimal list-inside mb-4 space-y-1 text-gray-700 dark:text-gray-300";
    pub const LI: &'static str = "leading-relaxed";

    // Links and images
    pub const LINK: &'static str = "text-blue-600 dark:text-blue-400 hover:text-blue-800 dark:hover:text-blue-300 underline underline-offset-2 hover:underline-offset-4 transition-all";
    pub const IMAGE: &'static str = "max-w-full h-auto rounded-lg shadow-sm my-4";

    // Tables
    pub const TABLE: &'static str = "min-w-full divide-y divide-gray-200 dark:divide-gray-700 my-4 border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden";
    pub const THEAD: &'static str = "bg-gray-50 dark:bg-gray-800";
    pub const TR: &'static str =
        "bg-white dark:bg-gray-900 even:bg-gray-50 dark:even:bg-gray-800/50";
    pub const TD: &'static str = "px-6 py-4 text-sm text-gray-900 dark:text-gray-100";
    pub const TH: &'static str = "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider";

    // Other elements
    pub const HR: &'static str = "border-0 h-px bg-gradient-to-r from-transparent via-gray-300 dark:via-gray-600 to-transparent my-8";
    pub const CHECKBOX: &'static str = "mr-2 accent-blue-600";

    // Math
    pub const MATH_INLINE: &'static str = "font-serif italic text-gray-800 dark:text-gray-200";
    pub const MATH_DISPLAY: &'static str = "font-serif italic text-center my-4 p-3 bg-gray-50 dark:bg-gray-800 rounded-lg text-gray-800 dark:text-gray-200";

    // Definition lists
    pub const DL: &'static str = "my-4";
    pub const DT: &'static str = "font-semibold text-gray-900 dark:text-gray-100 mt-4 first:mt-0";
    pub const DD: &'static str = "ml-6 mb-2 text-gray-700 dark:text-gray-300";

    // Superscript/Subscript
    pub const SUP: &'static str = "text-xs align-super";
    pub const SUB: &'static str = "text-xs align-sub";

    // Emphasis
    pub const EM: &'static str = "italic";
    pub const STRONG: &'static str = "font-bold";
    pub const DEL: &'static str = "line-through text-gray-500 dark:text-gray-400";

    // Special elements
    pub const FOOTNOTE_REF: &'static str = "text-xs align-super text-blue-600 dark:text-blue-400 hover:text-blue-800 dark:hover:text-blue-300";
    pub const FOOTNOTE_DEF: &'static str = "text-sm border-t border-gray-200 dark:border-gray-700 mt-8 pt-4 text-gray-600 dark:text-gray-400";
    pub const RAW_HTML_BLOCK: &'static str = "bg-yellow-50 dark:bg-yellow-950/30 border border-yellow-200 dark:border-yellow-800 rounded-lg p-3 my-4 font-mono text-sm text-yellow-800 dark:text-yellow-200 whitespace-pre-wrap";
    pub const INLINE_HTML: &'static str = "bg-yellow-100 dark:bg-yellow-900/50 text-yellow-800 dark:text-yellow-200 px-2 py-1 rounded text-xs font-mono border border-yellow-300 dark:border-yellow-700";

    // Theme-specific code block classes
    pub const THEME_DEFAULT: &'static str = "bg-gray-50 dark:bg-gray-900";
    pub const THEME_DARK: &'static str = "bg-gray-900 text-gray-100";
    pub const THEME_LIGHT: &'static str = "bg-white text-gray-900 border";
    pub const THEME_GITHUB: &'static str =
        "bg-[#f6f8fa] dark:bg-[#0d1117] text-[#24292f] dark:text-[#f0f6fc]";
    pub const THEME_MONOKAI: &'static str = "bg-[#272822] text-[#f8f8f2]";
}

/// Get theme-specific classes for code blocks
pub fn get_code_theme_classes(theme: &CodeBlockTheme) -> &'static str {
    match theme {
        CodeBlockTheme::Default => MarkdownClasses::THEME_DEFAULT,
        CodeBlockTheme::Dark => MarkdownClasses::THEME_DARK,
        CodeBlockTheme::Light => MarkdownClasses::THEME_LIGHT,
        CodeBlockTheme::GitHub => MarkdownClasses::THEME_GITHUB,
        CodeBlockTheme::Monokai => MarkdownClasses::THEME_MONOKAI,
    }
}

/// Enhanced Tailwind prose configuration for better markdown styling
pub fn get_enhanced_prose_classes() -> &'static str {
    "leptos-mdx-content prose prose-gray max-w-none dark:prose-invert prose-headings:font-bold prose-headings:text-gray-900 dark:prose-headings:text-gray-100 prose-p:text-gray-700 dark:prose-p:text-gray-300 prose-a:text-blue-600 dark:prose-a:text-blue-400 prose-strong:text-gray-900 dark:prose-strong:text-gray-100 prose-code:text-gray-800 dark:prose-code:text-gray-200 prose-pre:bg-gray-50 dark:prose-pre:bg-gray-900"
}

/// Placeholder component - Tailwind handles all styling
#[component]
pub fn MarkdownStyles() -> impl IntoView {
    // With Tailwind 4, no custom styles needed
    ""
}
