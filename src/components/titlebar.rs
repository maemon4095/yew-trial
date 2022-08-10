use crate::glue::*;
use yew::prelude::*;

#[function_component(Titlebar)]
pub fn titlebar() -> Html {
    html! {
        <div class="titlebar">
            <button onclick={Callback::from(|_| Window::alert("alert"))}> { "titlebar" } </button>
        </div>
    }
}
