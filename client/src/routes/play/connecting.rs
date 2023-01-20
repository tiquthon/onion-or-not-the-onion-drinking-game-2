use yew::{html, Component, Context, Html};

pub struct ConnectingComponent;

impl Component for ConnectingComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>{"Connecting..."}</div>
        }
    }
}
