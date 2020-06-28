// use crate::routes::AppRoute;
use yew::{html, Callback, Html, MouseEvent};
use yew_functional::{use_context,use_effect_with_deps,use_state, FunctionComponent, FunctionProvider};
// use yew_router::prelude::*;
use crate::store::store::{Action, StoreDispatch, StoreModel};
use web_sys::console;
use std::rc::Rc;
use crate::service::api::get_city_list;
use crate::service::future::handle_future;
use crate::service::api::CityResult;



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



        let (city, set_city) = use_state(||"".to_string());


        use_effect_with_deps(
            move|_| {
                let future = async {
                    get_city_list().await
                };

                handle_future(future,move |value :CityResult|  set_city( match value.hotCities.get(0) {
                    Some(hotCities) => hotCities.name.clone(),
                    None => "".to_string(),
                }));

                return || ();
            },
            (),
        );

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
                        value=city
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
                {city}
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
