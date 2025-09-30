use std::usize;

use leptos::{html::q, prelude::*};
mod quotes;
use serde::Deserialize;
//use rand;
/*#[derive(Debug, Clone, Deserialize)]
pub struct Entry {
    author: String,
    text: String,
    theme: String,
    probability: u8,
}*/
/*#[derive(Debug, Deserialize)]
struct Entries {
    // This field name must match the TOML table array name (e.g., `[[quote]]` becomes `quote` here)
    quotes: Vec<Entry>,
}*/
/*fn get_quotes() -> String {
    //let num = rand::random_range(0..=4);
    let mut _entries: Vec<Entry> = Vec::new();
    let filename: String = "quotes.txt".to_string();
    let toml_content = std::fs::read_to_string(filename).unwrap();
    println!("AHHHHHHHH{}", toml_content);
    let quotes: Entries = toml::from_str(&toml_content).expect("FUCK");
    let _meow: &str = "kys";
    //let q = quotes.quotes.iter().nth(0).unwrap().text.clone();
    let result = quotes.quotes.iter().nth(0).expect("failed");
    result.text.clone()
}*/
#[component]
fn App() -> impl IntoView {
    //let current_quote = get_quotes();
    let current_quote: Vec<quotes::quotes::Entry> = quotes::quotes::get_quotes();
    let total_quotes: usize = current_quote.len();
    let mut current_quote_num: u8 = 0;
    let cuc: quotes::quotes::Entry = current_quote
        .into_iter()
        .nth(current_quote_num.into())
        .unwrap();
    let (quote_num, set_quite_num) = create_signal(current_quote_num);
    let (cuc_signal, _set_cuc_signal) = create_signal(cuc.text.to_string());
    let (cuc_signal_1, _set_cuc_signal) = create_signal(cuc.author.to_string());
    view! {
        <div class="QuoteDiv"><p>{ move || cuc_signal.get()}</p><p>{move || cuc_signal_1.get()}</p></div>
        <button on:click=move |_| *set_quite_num.write() +=change_num(0) >{move || quote_num.get()}get random quote</button>

    }
}
fn change_num(mut num: u8) -> u8 {
    if num < 4 {
        num = num + 1
    }
    0
}
fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
