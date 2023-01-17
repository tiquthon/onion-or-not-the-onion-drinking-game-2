use yew::{html, Component, Context, Html};

pub struct AftermathComponent;

impl Component for AftermathComponent {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>{"it works!"}</div>
        }
    }
}
