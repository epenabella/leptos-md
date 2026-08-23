use crate::components::{get_code_theme_classes, MarkdownOptions};
use leptos::prelude::*;
use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag};

fn join(parts: &[&str]) -> String {
    parts
        .iter()
        .filter(|s| !s.is_empty())
        .copied()
        .collect::<Vec<_>>()
        .join(" ")
}

pub struct MarkdownRenderer {
    options: MarkdownOptions,
    in_thead: std::cell::Cell<bool>,
}

impl MarkdownRenderer {
    pub fn new(options: MarkdownOptions) -> Self {
        Self {
            options,
            in_thead: std::cell::Cell::new(false),
        }
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
        let classes = &self.options.classes;
        match &events[0] {
            Event::Start(tag) => self.render_start_tag(tag, events),
            Event::End(_) => {
                // End tags are handled by their corresponding start tags
                ("".into_any(), 1)
            }
            Event::Text(text) => (text.to_string().into_any(), 1),
            Event::Code(code) => (
                view! {
                    <code class=classes.inline_code.clone()>{code.to_string()}</code>
                }
                .into_any(),
                1,
            ),
            Event::Html(html) => (
                view! {
                    <span class=classes.inline_html.clone()>{html.to_string()}</span>
                }
                .into_any(),
                1,
            ),
            Event::SoftBreak => (view! { <span>" "</span> }.into_any(), 1),
            Event::HardBreak => (view! { <br /> }.into_any(), 1),
            Event::Rule => (
                view! { <hr class=classes.hr.clone() /> }.into_any(),
                1,
            ),
            Event::FootnoteReference(reference) => (
                view! {
                    <sup class=classes.footnote_ref.clone()>
                        <a href=format!("#{}", reference)>{reference.to_string()}</a>
                    </sup>
                }
                .into_any(),
                1,
            ),
            Event::TaskListMarker(checked) => (
                view! {
                    <input type="checkbox" class=classes.checkbox.clone() checked=*checked disabled />
                }
                .into_any(),
                1,
            ),
            Event::InlineMath(expr) => (
                view! {
                    <span class=classes.math_inline.clone()>{expr.to_string()}</span>
                }
                .into_any(),
                1,
            ),
            Event::DisplayMath(expr) => (
                view! {
                    <div class=classes.math_display.clone()>{expr.to_string()}</div>
                }
                .into_any(),
                1,
            ),
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
        let classes = &self.options.classes;

        match tag {
            Tag::Paragraph => {
                let inner_content = self.render_events(inner_events);
                (
                    view! { <p class=classes.paragraph.clone()>{inner_content}</p> }.into_any(),
                    consumed,
                )
            }
            Tag::Heading { level, .. } => {
                let inner_content = self.render_events(inner_events);
                match level {
                    HeadingLevel::H1 => (
                        view! { <h1 class=classes.h1.clone()>{inner_content}</h1> }.into_any(),
                        consumed,
                    ),
                    HeadingLevel::H2 => (
                        view! { <h2 class=classes.h2.clone()>{inner_content}</h2> }.into_any(),
                        consumed,
                    ),
                    HeadingLevel::H3 => (
                        view! { <h3 class=classes.h3.clone()>{inner_content}</h3> }.into_any(),
                        consumed,
                    ),
                    HeadingLevel::H4 => (
                        view! { <h4 class=classes.h4.clone()>{inner_content}</h4> }.into_any(),
                        consumed,
                    ),
                    HeadingLevel::H5 => (
                        view! { <h5 class=classes.h5.clone()>{inner_content}</h5> }.into_any(),
                        consumed,
                    ),
                    HeadingLevel::H6 => (
                        view! { <h6 class=classes.h6.clone()>{inner_content}</h6> }.into_any(),
                        consumed,
                    ),
                }
            }
            Tag::BlockQuote(_) => {
                let inner_content = self.render_events(inner_events);
                (
                    view! {
                        <blockquote class=classes.blockquote.clone()>
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
                let language_class = language_class.unwrap_or_default();

                // Get theme classes if a theme is set
                let theme_classes = self
                    .options
                    .code_theme
                    .as_ref()
                    .map(get_code_theme_classes)
                    .unwrap_or_default();

                let combined_class = join(&[&classes.code_block, &language_class, theme_classes]);
                let code_class = join(&[&classes.code_block_code, &language_class]);

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
                    (
                        view! {
                            <ol class=classes.ol.clone() start=start.to_string()>{inner_content}</ol>
                        }
                        .into_any(),
                        consumed,
                    )
                } else {
                    (
                        view! {
                            <ul class=classes.ul.clone()>{inner_content}</ul>
                        }
                        .into_any(),
                        consumed,
                    )
                }
            }
            Tag::Item => {
                let inner_content = self.render_events(inner_events);
                (
                    view! { <li class=classes.li.clone()>{inner_content}</li> }.into_any(),
                    consumed,
                )
            }
            Tag::Emphasis => {
                let inner_content = self.render_events(inner_events);
                (
                    view! { <em class=classes.em.clone()>{inner_content}</em> }.into_any(),
                    consumed,
                )
            }
            Tag::Strong => {
                let inner_content = self.render_events(inner_events);
                (
                    view! { <strong class=classes.strong.clone()>{inner_content}</strong> }
                        .into_any(),
                    consumed,
                )
            }
            Tag::Strikethrough => {
                let inner_content = self.render_events(inner_events);
                (
                    view! { <del class=classes.del.clone()>{inner_content}</del> }.into_any(),
                    consumed,
                )
            }
            Tag::Link {
                dest_url, title, ..
            } => {
                let inner_content = self.render_events(inner_events);
                let href = dest_url.to_string();
                let link_class = classes.link.clone();

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
                let img_class = classes.image.clone();

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
                (
                    view! {
                        <table class=classes.table.clone()>
                            {inner_content}
                        </table>
                    }
                    .into_any(),
                    consumed,
                )
            }
            Tag::TableHead => {
                let previous = self.in_thead.replace(true);
                let inner_content = self.render_events(inner_events);
                self.in_thead.set(previous);
                (
                    view! { <thead class=classes.thead.clone()>{inner_content}</thead> }
                        .into_any(),
                    consumed,
                )
            }
            Tag::TableRow => {
                let inner_content = self.render_events(inner_events);
                (
                    view! { <tr class=classes.tr.clone()>{inner_content}</tr> }.into_any(),
                    consumed,
                )
            }
            Tag::TableCell => {
                let inner_content = self.render_events(inner_events);
                if self.in_thead.get() {
                    (
                        view! { <th class=classes.th.clone()>{inner_content}</th> }.into_any(),
                        consumed,
                    )
                } else {
                    (
                        view! { <td class=classes.td.clone()>{inner_content}</td> }.into_any(),
                        consumed,
                    )
                }
            }
            Tag::FootnoteDefinition(label) => {
                let inner_content = self.render_events(inner_events);
                (
                    view! {
                        <div class=classes.footnote_def.clone() id=label.to_string()>
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
                    (
                        view! {
                            <pre class=classes.raw_html_block.clone()>{raw_html}</pre>
                        }
                        .into_any(),
                        consumed,
                    )
                }
            }
            Tag::DefinitionList => {
                let inner_content = self.render_events(inner_events);
                (
                    view! { <dl class=classes.dl.clone()>{inner_content}</dl> }.into_any(),
                    consumed,
                )
            }
            Tag::DefinitionListTitle => {
                let inner_content = self.render_events(inner_events);
                (
                    view! { <dt class=classes.dt.clone()>{inner_content}</dt> }.into_any(),
                    consumed,
                )
            }
            Tag::DefinitionListDefinition => {
                let inner_content = self.render_events(inner_events);
                (
                    view! { <dd class=classes.dd.clone()>{inner_content}</dd> }.into_any(),
                    consumed,
                )
            }
            Tag::Superscript => {
                let inner_content = self.render_events(inner_events);
                (
                    view! { <sup class=classes.sup.clone()>{inner_content}</sup> }.into_any(),
                    consumed,
                )
            }
            Tag::Subscript => {
                let inner_content = self.render_events(inner_events);
                (
                    view! { <sub class=classes.sub.clone()>{inner_content}</sub> }.into_any(),
                    consumed,
                )
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
