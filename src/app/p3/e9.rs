#[allow(unused_imports)]
use itertools::Itertools;
use leptos::prelude::*;
#[allow(non_snake_case)]
pub fn App() -> impl IntoView {
    view! {
        // <FancyForm>
        <fieldset>
            <label>"Some Input" <input type="text" name="something" /></label>
        </fieldset>
        <button>"Submit"</button>
    }
}
