mod components;
mod glue;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

enum Msg {
    AddOne,
}

struct App {
    value: i64,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <>
                <components::Titlebar/>
                <div>
                    <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                    <p>{ self.value }</p>
                </div>
            </>
        }
    }
}

fn main() {
    let app = web_sys::window()
        .expect_throw("window was not found.")
        .document()
        .expect_throw("document was not found in window.")
        .get_element_by_id("#app")
        .expect_throw("app was not found in document.");

    yew::start_app_in_element::<App>(app);
}
