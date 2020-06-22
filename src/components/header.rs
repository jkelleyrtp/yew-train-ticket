use crate::routes::AppRoute;
use yew::{html, Callback, Html};
use yew_functional::{use_state, FunctionComponent, FunctionProvider};
use yew_router::prelude::*;

pub struct NavFC {}
impl FunctionProvider for NavFC {
    type TProps = ();

    fn run(_: &Self::TProps) -> Html {
        let (counter, set_counter) = use_state(|| 0);
        let counter1 = counter.clone();

        let onclick = Callback::from(move |_| set_counter(*counter + 1));

        return html! {
            <>
            <button onclick=onclick> {"click me"}</button>
            <nav>
                <ul>
                    <li><RouterAnchor<AppRoute> route=AppRoute::Home classes="app-link" >{*counter1}{ "Home" } </RouterAnchor<AppRoute>></li>
                    <li><RouterAnchor<AppRoute> route=AppRoute::About classes="app-link">{ "About" }</RouterAnchor<AppRoute>></li>
                </ul>
            </nav>
            </>
        };
    }
}
pub type Nav = FunctionComponent<NavFC>;
