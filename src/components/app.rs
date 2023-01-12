use yew::prelude::*;

use crate::components::selector::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <Selector/>
        </main>
    }
}
