use crate::components::{get_code_theme_classes, MarkdownClasses, MarkdownOptions};
use leptos::prelude::*;
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag};

pub struct MarkdownRenderer {
    options: MarkdownOptions,
}

impl MarkdownRenderer {
    pub fn new(options: MarkdownOptions) -> Self {
        Self { options }
    }

    pub fn render(&self, content: &str) -> Result<AnyView, String> {
        let mut parser_options = Options::empty();

        if self.options.enable_gfm {
            parser_options.insert(Options::ENABLE_TABLES);
            parser_options.insert(Options::ENABLE_FOOTNOTES);
            parser_options.insert(Options::ENABLE_STRIKETHROUGH);
            parser_options.insert(Options::ENABLE_TASKLISTS);
        }

        let parser = Parser::new_ext(content, parser_options);
        let events: Vec<Event> = parser.collect();

        Ok(self.render_events(&events))
    }

    fn render_events(&self, events: &[Event]) -> AnyView {
        let mut result = Vec::new();
        let mut i = 0;

        while i < events.len() {
            let (rendered, consumed) = self.render_event(&events[i..]);
            result.push(rendered);
            i += consumed;
        }

        result.into_iter().collect_view().into_any()
    }

    fn render_event(&self, events: &[Event]) -> (AnyView, usize) {
        match &events[0] {
            Event::Start(tag) => self.render_start_tag(tag, events),
            Event::End(_) => {
                // End tags are handled by their corresponding start tags
                ("".into_any(), 1)
            }
            Event::Text(text) => (text.to_string().into_any(), 1),
            Event::Code(code) => {
                let class = if self.options.use_explicit_classes {
                    MarkdownClasses::INLINE_CODE
                } else {
                    "inline-code"
                };
                (
                    view! {
                        <code class=class>{code.to_string()}</code>
                    }
                    .into_any(),
                    1,
                )
            }
            Event::Html(html) => {
                // For safety, we'll render HTML as text by default
                (
                    view! {
                        <span class="raw-html">{html.to_string()}</span>
                    }
                    .into_any(),
                    1,
                )
            }
            Event::SoftBreak => (view! { <span>" "</span> }.into_any(), 1),
            Event::HardBreak => (view! { <br /> }.into_any(), 1),
            Event::Rule => {
                let class = if self.options.use_explicit_classes {
                    MarkdownClasses::HR
                } else {
                    "markdown-hr"
                };
                (view! { <hr class=class /> }.into_any(), 1)
            }
            Event::FootnoteReference(reference) => {
                let class = if self.options.use_explicit_classes {
                    MarkdownClasses::FOOTNOTE_REF
                } else {
                    "footnote-ref"
                };
                (
                    view! {
                        <sup class=class>
                            <a href=format!("#{}", reference)>{reference.to_string()}</a>
                        </sup>
                    }
                    .into_any(),
                    1,
                )
            }
            Event::TaskListMarker(checked) => {
                let class = if self.options.use_explicit_classes {
                    MarkdownClasses::CHECKBOX
                } else {
                    ""
                };
                (
                    view! {
                        <input type="checkbox" class=class checked=*checked disabled />
                    }
                    .into_any(),
                    1,
                )
            }
            Event::InlineMath(expr) => {
                let class = if self.options.use_explicit_classes {
                    MarkdownClasses::MATH_INLINE
                } else {
                    "math math-inline"
                };
                (
                    view! {
                        <span class=class>{expr.to_string()}</span>
                    }
                    .into_any(),
                    1,
                )
            }
            Event::DisplayMath(expr) => {
                let class = if self.options.use_explicit_classes {
                    MarkdownClasses::MATH_DISPLAY
                } else {
                    "math math-display"
                };
                (
                    view! {
                        <div class=class>{expr.to_string()}</div>
                    }
                    .into_any(),
                    1,
                )
            }
            Event::InlineHtml(raw) => {
                if self.options.allow_raw_html {
                    (
                        view! {
                            <span inner_html=raw.to_string()></span>
                        }
                        .into_any(),
                        1,
                    )
                } else {
                    (raw.to_string().into_any(), 1)
                }
            }
        }
    }

    fn render_start_tag(&self, tag: &Tag, events: &[Event]) -> (AnyView, usize) {
        let (end_index, consumed) = self.find_matching_end(events);
        let inner_events = &events[1..end_index];

        let use_explicit = self.options.use_explicit_classes;

        match tag {
            Tag::Paragraph => {
                let inner_content = self.render_events(inner_events);
                if use_explicit {
                    (
                        view! { <p class=MarkdownClasses::PARAGRAPH>{inner_content}</p> }
                            .into_any(),
                        consumed,
                    )
                } else {
                    (view! { <p>{inner_content}</p> }.into_any(), consumed)
                }
            }
            Tag::Heading { level, .. } => {
                let inner_content = self.render_events(inner_events);
                if use_explicit {
                    match level {
                        HeadingLevel::H1 => (
                            view! { <h1 class=MarkdownClasses::H1>{inner_content}</h1> }.into_any(),
                            consumed,
                        ),
                        HeadingLevel::H2 => (
                            view! { <h2 class=MarkdownClasses::H2>{inner_content}</h2> }.into_any(),
                            consumed,
                        ),
                        HeadingLevel::H3 => (
                            view! { <h3 class=MarkdownClasses::H3>{inner_content}</h3> }.into_any(),
                            consumed,
                        ),
                        HeadingLevel::H4 => (
                            view! { <h4 class=MarkdownClasses::H4>{inner_content}</h4> }.into_any(),
                            consumed,
                        ),
                        HeadingLevel::H5 => (
                            view! { <h5 class=MarkdownClasses::H5>{inner_content}</h5> }.into_any(),
                            consumed,
                        ),
                        HeadingLevel::H6 => (
                            view! { <h6 class=MarkdownClasses::H6>{inner_content}</h6> }.into_any(),
                            consumed,
                        ),
                    }
                } else {
                    match level {
                        HeadingLevel::H1 => {
                            (view! { <h1>{inner_content}</h1> }.into_any(), consumed)
                        }
                        HeadingLevel::H2 => {
                            (view! { <h2>{inner_content}</h2> }.into_any(), consumed)
                        }
                        HeadingLevel::H3 => {
                            (view! { <h3>{inner_content}</h3> }.into_any(), consumed)
                        }
                        HeadingLevel::H4 => {
                            (view! { <h4>{inner_content}</h4> }.into_any(), consumed)
                        }
                        HeadingLevel::H5 => {
                            (view! { <h5>{inner_content}</h5> }.into_any(), consumed)
                        }
                        HeadingLevel::H6 => {
                            (view! { <h6>{inner_content}</h6> }.into_any(), consumed)
                        }
                    }
                }
            }
            Tag::BlockQuote(_) => {
                let inner_content = self.render_events(inner_events);
                let class = if use_explicit {
                    MarkdownClasses::BLOCKQUOTE
                } else {
                    "markdown-blockquote"
                };
                (
                    view! {
                        <blockquote class=class>
                            {inner_content}
                        </blockquote>
                    }
                    .into_any(),
                    consumed,
                )
            }
            Tag::CodeBlock(kind) => {
                let code_content = self.extract_text_content(inner_events);

                // Determine language class if syntax_highlighting_language_classes is enabled
                let language_class = if self.options.syntax_highlighting_language_classes {
                    match kind {
                        CodeBlockKind::Indented => Some("language-text".to_string()),
                        CodeBlockKind::Fenced(lang) => {
                            if lang.is_empty() {
                                Some("language-text".to_string())
                            } else {
                                Some(format!("language-{}", lang))
                            }
                        }
                    }
                } else {
                    None
                };

                // Get Tailwind theme classes if a theme is set
                let theme_classes = self
                    .options
                    .code_theme
                    .as_ref()
                    .map(|theme| get_code_theme_classes(theme));

                // Base class for <pre>
                let base_pre_class = if use_explicit {
                    MarkdownClasses::CODE_BLOCK
                } else {
                    "markdown-code-block"
                };

                // Build the combined class for <pre>
                let combined_class = match (&language_class, theme_classes) {
                    (Some(lang), Some(theme)) => {
                        format!("{} {} {}", base_pre_class, lang, theme)
                    }
                    (Some(lang), None) => format!("{} {}", base_pre_class, lang),
                    (None, Some(theme)) => format!("{} {}", base_pre_class, theme),
                    (None, None) => base_pre_class.to_string(),
                };

                // Build the class for <code>
                let code_class = if use_explicit {
                    match &language_class {
                        Some(lang) => format!("{} {}", MarkdownClasses::CODE_BLOCK_CODE, lang),
                        None => MarkdownClasses::CODE_BLOCK_CODE.to_string(),
                    }
                } else {
                    language_class.unwrap_or_default()
                };

                (
                    view! {
                        <pre class=combined_class>
                            <code class=code_class>{code_content}</code>
                        </pre>
                    }
                    .into_any(),
                    consumed,
                )
            }
            Tag::List(start_number) => {
                let inner_content = self.render_events(inner_events);
                if let Some(start) = start_number {
                    if use_explicit {
                        (
                            view! {
                                <ol class=MarkdownClasses::OL start=start.to_string()>{inner_content}</ol>
                            }
                            .into_any(),
                            consumed,
                        )
                    } else {
                        (
                            view! {
                                <ol start=start.to_string()>{inner_content}</ol>
                            }
                            .into_any(),
                            consumed,
                        )
                    }
                } else if use_explicit {
                    (
                        view! {
                            <ul class=MarkdownClasses::UL>{inner_content}</ul>
                        }
                        .into_any(),
                        consumed,
                    )
                } else {
                    (
                        view! {
                            <ul>{inner_content}</ul>
                        }
                        .into_any(),
                        consumed,
                    )
                }
            }
            Tag::Item => {
                let inner_content = self.render_events(inner_events);
                if use_explicit {
                    (
                        view! { <li class=MarkdownClasses::LI>{inner_content}</li> }.into_any(),
                        consumed,
                    )
                } else {
                    (view! { <li>{inner_content}</li> }.into_any(), consumed)
                }
            }
            Tag::Emphasis => {
                let inner_content = self.render_events(inner_events);
                if use_explicit {
                    (
                        view! { <em class=MarkdownClasses::EM>{inner_content}</em> }.into_any(),
                        consumed,
                    )
                } else {
                    (view! { <em>{inner_content}</em> }.into_any(), consumed)
                }
            }
            Tag::Strong => {
                let inner_content = self.render_events(inner_events);
                if use_explicit {
                    (
                        view! { <strong class=MarkdownClasses::STRONG>{inner_content}</strong> }
                            .into_any(),
                        consumed,
                    )
                } else {
                    (
                        view! { <strong>{inner_content}</strong> }.into_any(),
                        consumed,
                    )
                }
            }
            Tag::Strikethrough => {
                let inner_content = self.render_events(inner_events);
                if use_explicit {
                    (
                        view! { <del class=MarkdownClasses::DEL>{inner_content}</del> }.into_any(),
                        consumed,
                    )
                } else {
                    (view! { <del>{inner_content}</del> }.into_any(), consumed)
                }
            }
            Tag::Link {
                dest_url, title, ..
            } => {
                let inner_content = self.render_events(inner_events);
                let href = dest_url.to_string();
                let link_class = if use_explicit {
                    MarkdownClasses::LINK
                } else {
                    ""
                };

                if !title.is_empty() {
                    if self.options.open_links_in_new_tab {
                        (
                            view! {
                            <a class=link_class href=href title=title.to_string() target="_blank" rel="noopener noreferrer">
                                {inner_content}
                            </a>
                        }
                            .into_any(),
                            consumed,
                        )
                    } else {
                        (
                            view! {
                                <a class=link_class href=href title=title.to_string()>
                                    {inner_content}
                                </a>
                            }
                            .into_any(),
                            consumed,
                        )
                    }
                } else if self.options.open_links_in_new_tab {
                    (
                        view! {
                            <a class=link_class href=href target="_blank" rel="noopener noreferrer">
                                {inner_content}
                            </a>
                        }
                        .into_any(),
                        consumed,
                    )
                } else {
                    (
                        view! {
                            <a class=link_class href=href>
                                {inner_content}
                            </a>
                        }
                        .into_any(),
                        consumed,
                    )
                }
            }
            Tag::Image {
                dest_url, title, ..
            } => {
                let src = dest_url.to_string();
                let alt = self.extract_text_content(inner_events);
                let img_class = if use_explicit {
                    MarkdownClasses::IMAGE
                } else {
                    "markdown-image"
                };

                if !title.is_empty() {
                    (
                        view! {
                            <img src=src alt=alt title=title.to_string() class=img_class />
                        }
                        .into_any(),
                        consumed,
                    )
                } else {
                    (
                        view! {
                            <img src=src alt=alt class=img_class />
                        }
                        .into_any(),
                        consumed,
                    )
                }
            }
            Tag::Table(_) => {
                let inner_content = self.render_events(inner_events);
                let class = if use_explicit {
                    MarkdownClasses::TABLE
                } else {
                    "markdown-table"
                };
                (
                    view! {
                        <table class=class>
                            {inner_content}
                        </table>
                    }
                    .into_any(),
                    consumed,
                )
            }
            Tag::TableHead => {
                let inner_content = self.render_events(inner_events);
                if use_explicit {
                    (
                        view! { <thead class=MarkdownClasses::THEAD>{inner_content}</thead> }
                            .into_any(),
                        consumed,
                    )
                } else {
                    (
                        view! { <thead>{inner_content}</thead> }.into_any(),
                        consumed,
                    )
                }
            }
            Tag::TableRow => {
                let inner_content = self.render_events(inner_events);
                if use_explicit {
                    (
                        view! { <tr class=MarkdownClasses::TR>{inner_content}</tr> }.into_any(),
                        consumed,
                    )
                } else {
                    (view! { <tr>{inner_content}</tr> }.into_any(), consumed)
                }
            }
            Tag::TableCell => {
                let inner_content = self.render_events(inner_events);
                if use_explicit {
                    (
                        view! { <td class=MarkdownClasses::TD>{inner_content}</td> }.into_any(),
                        consumed,
                    )
                } else {
                    (view! { <td>{inner_content}</td> }.into_any(), consumed)
                }
            }
            Tag::FootnoteDefinition(label) => {
                let inner_content = self.render_events(inner_events);
                let class = if use_explicit {
                    MarkdownClasses::FOOTNOTE_DEF
                } else {
                    "footnote-definition"
                };
                (
                    view! {
                        <div class=class id=label.to_string()>
                            {inner_content}
                        </div>
                    }
                    .into_any(),
                    consumed,
                )
            }
            Tag::HtmlBlock => {
                let raw_html = self.extract_text_content(inner_events);
                if self.options.allow_raw_html {
                    (
                        view! {
                            <div inner_html=raw_html></div>
                        }
                        .into_any(),
                        consumed,
                    )
                } else {
                    let class = if use_explicit {
                        MarkdownClasses::RAW_HTML_BLOCK
                    } else {
                        "raw-html-block"
                    };
                    (
                        view! {
                            <pre class=class>{raw_html}</pre>
                        }
                        .into_any(),
                        consumed,
                    )
                }
            }
            Tag::DefinitionList => {
                let inner_content = self.render_events(inner_events);
                if use_explicit {
                    (
                        view! { <dl class=MarkdownClasses::DL>{inner_content}</dl> }.into_any(),
                        consumed,
                    )
                } else {
                    (view! { <dl>{inner_content}</dl> }.into_any(), consumed)
                }
            }
            Tag::DefinitionListTitle => {
                let inner_content = self.render_events(inner_events);
                if use_explicit {
                    (
                        view! { <dt class=MarkdownClasses::DT>{inner_content}</dt> }.into_any(),
                        consumed,
                    )
                } else {
                    (view! { <dt>{inner_content}</dt> }.into_any(), consumed)
                }
            }
            Tag::DefinitionListDefinition => {
                let inner_content = self.render_events(inner_events);
                if use_explicit {
                    (
                        view! { <dd class=MarkdownClasses::DD>{inner_content}</dd> }.into_any(),
                        consumed,
                    )
                } else {
                    (view! { <dd>{inner_content}</dd> }.into_any(), consumed)
                }
            }
            Tag::Superscript => {
                let inner_content = self.render_events(inner_events);
                if use_explicit {
                    (
                        view! { <sup class=MarkdownClasses::SUP>{inner_content}</sup> }.into_any(),
                        consumed,
                    )
                } else {
                    (view! { <sup>{inner_content}</sup> }.into_any(), consumed)
                }
            }
            Tag::Subscript => {
                let inner_content = self.render_events(inner_events);
                if use_explicit {
                    (
                        view! { <sub class=MarkdownClasses::SUB>{inner_content}</sub> }.into_any(),
                        consumed,
                    )
                } else {
                    (view! { <sub>{inner_content}</sub> }.into_any(), consumed)
                }
            }
            Tag::MetadataBlock(_) => {
                // Metadata blocks are currently ignored. You could expose the data through callbacks if desired.
                ("".into_any(), consumed)
            }
        }
    }

    fn find_matching_end(&self, events: &[Event]) -> (usize, usize) {
        let mut depth = 0;
        for (i, event) in events.iter().enumerate() {
            match event {
                Event::Start(_) => depth += 1,
                Event::End(_) => {
                    depth -= 1;
                    if depth == 0 {
                        return (i, i + 1);
                    }
                }
                _ => {}
            }
        }
        // If no matching end found, consume all remaining events
        (events.len(), events.len())
    }

    fn extract_text_content(&self, events: &[Event]) -> String {
        events
            .iter()
            .filter_map(|event| match event {
                Event::Text(text) => Some(text.as_ref()),
                Event::Code(code) => Some(code.as_ref()),
                _ => None,
            })
            .collect::<Vec<&str>>()
            .join("")
    }
}
