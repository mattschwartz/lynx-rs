use yew::prelude::*;

pub struct Profile;

impl Component for Profile {
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
                <h2>{ "User Profile" }</h2>
                <p>{ "This is where profile information will be displayed." }</p>
            </div>
        }
    }
}
