use yew::prelude::*;

pub struct Kitty;

impl Component for Kitty {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h2>{ "Kitty" }</h2>
                <p>{ "This is where the kitty will be displayed." }</p>
            </div>
        }
    }
}