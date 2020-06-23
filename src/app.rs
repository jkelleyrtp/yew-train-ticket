use yew::prelude::*;
use yew_router::switch::Permissive;
use yew_router::{prelude::*, route::Route};

use crate::routes::{about::About, index::Index, AppRoute};

/// Root component
pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Router<AppRoute, ()>
                    render = Router::render(|switch: AppRoute | {
                        match switch {
                            AppRoute::Index => html!{ <Index /> },
                            AppRoute::Home => html!{ <Index /> },
                            AppRoute::About => html!{ <About /> },
                            AppRoute::PageNotFound(Permissive(None)) => html!{"Page not found"},
                            AppRoute::PageNotFound(Permissive(Some(missed_route))) => html!{format!("Page '{}' not found", missed_route)}
                        }
                    } )
                    redirect = Router::redirect(|route: Route<()>| {
                        AppRoute::PageNotFound(Permissive(Some(route.route)))
                    })
                />
            </>
        }
    }
}
