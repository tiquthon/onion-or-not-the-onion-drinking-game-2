use yew::prelude::*;

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <p>{"It works!"}</p>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
