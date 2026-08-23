use leptos::prelude::*;

use itertools::Itertools;
// fn Uncontroll_forms() -> impl IntoView {
//     leptos::html;
//     // leptos::S
//     let (name, set_name) = signal("Uncontrolled".to_string());
//
//     let input_element: NodeRef<html::Input> = NodeRef::new();
//     // let on_submit = move |ev: SubmitEvent| {
//     //     // stop the page from reloading!
//     //     ev.prevent_default();
//     //
//     //     // here, we'll extract the value from the input
//     //     let value = input_element
//     //         .get()
//     //         // event handlers can only fire after the view
//     //         // is mounted to the DOM, so the `NodeRef` will be `Some`
//     //         .expect("<input> should be mounted")
//     //         // `leptos::HtmlElement<html::Input>` implements `Deref`
//     //         // to a `web_sys::HtmlInputElement`.
//     //         // this means we can call`HtmlInputElement::value()`
//     //         // to get the current value of the input
//     //         .value();
//     //     set_name.set(value);
//     // };
//
//     view! {
//         // on_submit defined below
//         <form on:submit=on_submit>
//             <input type="text" value=name node_ref=input_element />
//             <input type="submit" value="Submit" />
//         </form>
//         <p>"Name is: " {name}</p>
//     }
// }
pub fn forms() -> impl IntoView {
    let (name, set_name) = signal("Controlled".to_string());

    view! {
        <input
            type="text"
            // adding :target gives us typed access to the element
            // that is the target of the event that fires
            on:input:target=move |ev| {
                set_name.set(ev.target().value());
            }
            prop:value=name
        />
        <br />
        <input
            type="text"
            // adding :target gives us typed access to the element
            // that is the target of the event that fires
            on:input:target=move |ev| {
                set_name.set(ev.target().value());
            }
            prop:value=name.get()
        />
        <input
            type="text"
            // adding :target gives us typed access to the element
            // that is the target of the event that fires
            bind:name=(name, set_name)
            // bind:type=(name, set_name)
            bind:value=(name, set_name)
        />

        // the `prop:` syntax lets you update a DOM property,
        // rather than an attribute.
        <p>"Name is: " {name}</p>
        <p>"Name is: " {name}</p>
    }
    // <Uncontroll_forms />
}
