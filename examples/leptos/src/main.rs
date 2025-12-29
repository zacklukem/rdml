use leptos::prelude::*;
use rdml_leptos::rdml;

#[component]
fn App() -> impl IntoView {
    let items = RwSignal::new(vec!["Item 1".to_owned(), "Item 2".into()]);
    let value = RwSignal::new("".to_owned());

    rdml! {
        ol {
            #[key(item.clone())]
            for item in items.get() {
                li {
                    (item)
                }
            }
        }

        form(
            on:submit=move |e| {
                e.prevent_default();
                items.write().push(value.get());
                value.set("".to_owned());
            }
        ) {
            input(bind:value=value) {}
            button { "Add Item" }
        }

        match value.read().len() {
            0 => "Input is required",
            0..=10 => {},
            11.. => "That input is too long",
        }
    }
}

fn main() {
    mount_to_body(App);
}
