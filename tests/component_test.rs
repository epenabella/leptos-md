use leptos::prelude::*;
use leptos_md::Markdown;
#[test]
fn plain_usage_infers() {
    let _v = view! { <Markdown content="# hi" /> };
}
