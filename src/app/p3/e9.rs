use leptos::prelude::*;

use itertools::Itertools;
pub fn App() -> impl IntoView {
    view! {
        // <FancyForm>
        <fieldset>
            <label>"Some Input" <input type="text" name="something" /></label>
        </fieldset>
        <button>"Submit"</button>
    }
}
