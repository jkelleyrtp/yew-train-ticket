// use crate::routes::AppRoute;
use yew::{html, Callback, Html, Properties};
use yew_functional::{use_context, FunctionComponent, FunctionProvider};
// use yew_router::prelude::*;
use super::super::routes::index::StoreModel;
use std::rc::Rc;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub on_exchange_from_to: Callback<()>,
}

pub struct JourneyFC {}
impl FunctionProvider for JourneyFC {
    type TProps = Props;

    fn run(props: &Self::TProps) -> Html {
        let context = use_context::<Rc<StoreModel>>();

        let ctx = &context.unwrap();

        let from = &ctx.from;
        let to = &ctx.to;
        let on_exchange_from_to = props.on_exchange_from_to.clone();

        return html! {
            <div class="journey">
                <div
                    class="journey-station"
                    // onClick={() => showCitySelector(true)}
                >
                    <input
                        type="text"
                        readOnly =true
                        name="from"
                        value={from}
                        class="journey-input journey-from"
                    />
                </div>
                <div class="journey-switch"
                onclick=Callback::from(move |_| on_exchange_from_to.emit(()))
                >
                    {"<>"}
                </div>
                <div
                    class="journey-station"
                    // onClick={() => showCitySelector(false)}
                >
                    <input
                        type="text"
                        readOnly=true
                        name="to"
                        value={to}
                        class="journey-input journey-to"
                    />
                </div>
            </div>
        };
    }
}
pub type Journey = FunctionComponent<JourneyFC>;
