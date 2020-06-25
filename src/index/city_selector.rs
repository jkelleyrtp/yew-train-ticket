// use crate::routes::AppRoute;
use yew::{html, Callback, Html, MouseEvent, Properties};
use yew_functional::{use_context, FunctionComponent, FunctionProvider};
// use yew_router::prelude::*;
use crate::store::store::{Action, StoreDispatch, StoreModel};
// use yew_router::prelude::*;
use crate::components::header::Header;
use chrono::prelude::*;
use std::rc::Rc;

pub struct CitySelectorFC {}
pub type CitySelector = FunctionComponent<CitySelectorFC>;
impl FunctionProvider for CitySelectorFC {
    type TProps = ();

    fn run(_: &Self::TProps) -> Html {
        let context = use_context::<Rc<StoreModel>>();
        let ctx = &context.unwrap();
        let StoreModel {
            city_selector_visible: show,
            ..
        } = &***ctx;

        let hidden_class = if *show { "" } else { "hidden" };

        let context_dispatch = use_context::<StoreDispatch>();
        let onclick: Callback<MouseEvent> = Callback::from(move |_| match &context_dispatch {
            Some(dispatch) => {
                let dispatch = &*dispatch;
                dispatch.emit(Action::ToggleCitySelectorVisible);
                return ();
            }
            _ => (),
        });

        return html! {
            <div
            class=format!("{} {}", "city-selector" ,hidden_class )
            >
            <div class="city-search">
                <div class="search-back"
                // onClick={() => onBack()}
                >
                <div class="header-back"  style=" width=42 "
                onclick=onclick
                >
                {"<"}
                </div>
                </div>
                <div class="search-input-wrapper">
                    <input
                        type="text"
                        // value={searchKey}
                        class="search-input"
                        placeholder="城市、车站的中文或拼音"
                        // onChange={e => setSearchKey(e.target.value)}
                    />
                </div>
                <i
                    // onClick={() => setSearchKey('')}
                    // class={classnames('search-clean', {
                    //     hidden: key.length === 0,
                    // })}
                >
                   { "&#xf063;"}
                </i>
            </div>
            // {Boolean(key) && (
            //     <Suggest searchKey={key} onSelect={key => onSelect(key)} />
            // )}
            // {outputCitySections()}
        </div>
        };
    }
}
