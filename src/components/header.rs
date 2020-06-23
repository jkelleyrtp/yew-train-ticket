// use crate::routes::AppRoute;
use yew::{html, Html, Properties};
use yew_functional::{FunctionComponent, FunctionProvider};
// use yew_router::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub title: String,
}

pub struct HeaderFC {}
impl FunctionProvider for HeaderFC {
    type TProps = Props;

    fn run(props: &Self::TProps) -> Html {
        let title = &props.title;

        return html! {
            <div class="header">
                <div class="header-back"  style=" width=42 height=42" >
                </div>
                <h1 class="header-title">{title}</h1>
            </div>
        };
    }
}
pub type Header = FunctionComponent<HeaderFC>;
