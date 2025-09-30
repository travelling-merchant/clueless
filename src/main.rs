use std::usize;

use leptos::prelude::*;
mod quotes;
use js_sys::Math;

#[component]
fn App() -> impl IntoView {
    let quotes: Vec<quotes::quotes::Entry> = quotes::quotes::get_quotes();
    let total_quotes: usize = quotes.len();

    let (quote_num, set_quite_num) = signal(0);
    let current_quote = Memo::new(move |_| {
        quotes
            .get(quote_num.get())
            .cloned()
            .unwrap_or(quotes::quotes::Entry {
                text: "No quite found".into(),
                author: "Nice try".into(),
                theme: "nothing  lol".into(),
                probability: 0,
            })
    });
    view! {
            <div class="ContentDiv">
            <button class="generateQuoteButton" on:click=move |_|{
                let new_quote_index = random_index(total_quotes,quote_num.get());
            set_quite_num.set(new_quote_index);
            }>
        generate random quote
            </button>
            <div class="QuoteDiv">
            <p>{ move || {
                current_quote.get().text
            }} </p>
            <p>{ move || {
                current_quote.get().author
            }} </p>
            </div>
    </div>
        }
}
fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
fn random_index(max: usize, old_me_haha: usize) -> usize {
    loop {
        let random = Math::random();
        let n = (random * max as f64).floor() as usize;
        if n != old_me_haha {
            return n;
        }
    }
}
