use yew::{html, Component, Context, Html};

pub struct GameComponent;

impl Component for GameComponent {
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
