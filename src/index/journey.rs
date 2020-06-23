// use crate::routes::AppRoute;
use yew::{html, Callback, Html};
use yew_functional::{use_context, FunctionComponent, FunctionProvider};
// use yew_router::prelude::*;
use crate::store::store::{Action, StoreDispatch, StoreModel};

use std::rc::Rc;

pub struct JourneyFC {}
pub type Journey = FunctionComponent<JourneyFC>;
impl FunctionProvider for JourneyFC {
    type TProps = ();

    fn run(_: &Self::TProps) -> Html {
        let context = use_context::<Rc<StoreModel>>();
        let ctx = &context.unwrap();
        let StoreModel { to, from, .. } = &***ctx;

        let context_dispatch = use_context::<StoreDispatch>();
        let onclick = Callback::from(move |_| match &context_dispatch {
            Some(dispatch) => {
                let dispatch = &*dispatch;
                dispatch.emit(Action::ExchangeFromTo);
                return ();
            }
            _ => (),
        });

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
                 onclick=onclick
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
