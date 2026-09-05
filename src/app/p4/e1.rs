#[allow(unused_imports)]
use itertools::Itertools;
use leptos::prelude::*;
pub fn App() -> impl IntoView {
    let (names, set_names) = signal(Vec::new());
    if names.get().is_empty() {
        set_names(vec!["alice".to_string()]);
    }

    // A
    let (count, set_count) = signal(1);
    // B is a function of A
    let derived_signal_double_count = move || count.get() * 2;
    // B is a function of A
    let memoized_double_count = Memo::new(move |_| count.get() * 2);
    // A
    let (first_name, set_first_name) = signal("Bridget".to_string());
    // B
    let (last_name, set_last_name) = signal("Jones".to_string());
    // C is a function of A and B
    let full_name = move || format!("{} {}", &*first_name.read(), &*last_name.read());
    // A
    let (age, set_age) = signal(32);
    // B
    let (favorite_number, set_favorite_number) = signal(42);
    // use this to handle a click on a `Clear` button
    let clear_handler = move |_| {
        // update both A and B
        set_age.set(0);
        set_favorite_number.set(0);
    };
    view! {
        {age}
        <br />
        {favorite_number}
        <br />
        <button on:click=clear_handler>asdfsdf</button>
    }
}
