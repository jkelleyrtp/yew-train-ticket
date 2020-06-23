// use crate::routes::AppRoute;
use yew::{html, Html, Properties};
use yew_functional::{FunctionComponent, FunctionProvider};
// use yew_router::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

pub struct SubmitFC {}
pub type Submit = FunctionComponent<SubmitFC>;

impl FunctionProvider for SubmitFC {
    type TProps = Props;

    fn run(_props: &Self::TProps) -> Html {
        return html! {
            <div class="submit">
            <button type="submit" class="submit-button">
                {"搜索"}
            </button>
        </div>
        };
    }
}
