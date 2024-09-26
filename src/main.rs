mod net;

use yew::prelude::*;

struct App {}

enum Msg {
    SendMessage,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
       Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        todo!();
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div id="app">
                <h1 class={classes!("titles")}>{"ChatApp.rs"}</h1>
                <div id={"mainInputContainer"}>
                    <input id={"mainTextInput"} class={classes!("mainInputs")} />
                    <button id={"mainButtonInput"} class={classes!("mainInputs")}>
                        <img width=32 height=30 src={"Static/send-message.png"} />
                    </button>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}