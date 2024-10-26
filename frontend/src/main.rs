use yew::prelude::*;
use yew::Renderer;
use yew_router::prelude::*;

mod components;

#[derive(Routable, PartialEq, Clone, Debug)]
enum AppRoute {
    #[at("/")]
    Home,
    #[at("/profile")]
    Profile,
    #[at("/kitty")]
    Kitty,
    #[at("/ability")]
    Ability,
}

struct App;

impl Component for App {
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
            <BrowserRouter>
                <Switch<AppRoute> render={switch} />
            </BrowserRouter>
        }
    }
}

fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { <h1>{ "Welcome to the Kitty App" }</h1> },
        AppRoute::Profile => html! { <components::profile::Profile /> },
        AppRoute::Kitty => html! { <components::kitty::Kitty /> },
        AppRoute::Ability => html! { <components::ability::Ability /> },
    }
}

fn main() {
    Renderer::<App>::new().render();
}
