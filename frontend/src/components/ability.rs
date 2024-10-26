use yew::prelude::*;

pub struct Ability;

impl Component for Ability {
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
                <h2>{ "Ability" }</h2>
                <p>{ "This is where the ability will be displayed." }</p>
            </div>
        }
    }
}
