use leptos::prelude::*;
use rdml_leptos::rdml;

#[component]
fn App() -> impl IntoView {
    let items = RwSignal::new(vec!["Item 1".to_owned(), "Item 2".into()]);
    let value = RwSignal::new("".to_owned());

    rdml! {
        ol {
            #[key(item.clone())]
            for (i, item) in items.get().into_iter().enumerate() {
                #[with(let item1 = item.clone();)]
                #[show]
                if i % 2 == 0 {
                    li { (item.clone()) }
                } else {
                    li { (item1.clone()) }
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

        #[with(let length = value.read().len();)]
        match length {
            0 => "Input is required",
            0..=10 => {},
            11.. => "That input is too long",
        }
    }
}

fn main() {
    mount_to_body(App);
}
