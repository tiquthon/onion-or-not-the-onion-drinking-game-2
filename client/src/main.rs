use onion_or_not_the_onion_drinking_game_2_client::AppComponent;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<AppComponent>::new().render();
}
