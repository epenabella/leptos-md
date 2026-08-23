# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2026-08-23

### Added
- `MarkdownClassMap` with `prose()` (default), `tailwind()` and `none()` presets; override individual elements via struct-update syntax
- `MarkdownOptions::with_classes(MarkdownClassMap)`
- Table header cells now render as `<th>`

### Changed
- `Markdown` wrapper class comes from `classes.wrapper`; the `class` prop still appends to it
- `TailwindMarkdownClasses` is `MarkdownClasses` again (plain consts)

### Removed (breaking)
- `use_explicit_classes` / `with_explicit_classes` — use `.with_classes(MarkdownClassMap::tailwind())` instead
- `MDClasses` trait and the generic parameter on `MarkdownOptions` / `Markdown`

Builds on #3 by @andyquinterom.

## [0.1.0] - 2025-12-18

### Added
- Initial release
- Markdown component with Tailwind CSS styling
- GFM support (tables, footnotes, strikethrough, task lists)
- Configurable code block themes (GitHub, Monokai, Dark, Light)
- Math expression support
- Builder pattern for `MarkdownOptions`
- `render_markdown_string` and `render_markdown_with_options` helpers
