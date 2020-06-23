use crate::components::header::Header;
use crate::index::depart_date::DepartDate;
use crate::index::high_speed::HighSpeed;
use crate::index::journey::Journey;
use crate::index::submit::Submit;

use chrono::prelude::*;
use std::rc::Rc;

use yew::{html, Callback, Html};
use yew_functional::{use_reducer_with_init, ContextProvider, FunctionComponent, FunctionProvider};

#[derive(Clone, Debug)]
pub struct StoreModel {
    pub from: String,
    pub to: String,
}

pub struct IndexFC {}
impl FunctionProvider for IndexFC {
    type TProps = ();

    fn run(_: &Self::TProps) -> Html {
        let initail_state = StoreModel {
            from: "北京".to_string(),
            to: "上海".to_string(),
        };

        let (store, dispatch) = use_reducer_with_init(
            |_prev: std::rc::Rc<StoreModel>, action: StoreModel| StoreModel { ..action },
            initail_state,
            |initail_state: StoreModel| initail_state,
        );

        let store1 = Rc::clone(&store);
        let on_exchange_from_to = move |_| {
            let ctx = &store1;
            let from = &ctx.from;
            let to = &ctx.to;
            dispatch(StoreModel {
                to: from.clone(),
                from: to.clone(),
            });
            return ();
        };

        type StoreModelContextProvider = ContextProvider<Rc<StoreModel>>;
        let local_time: DateTime<Local> = Local::now();

        let is_high_speed = true;
        return html! {
            <>

            <StoreModelContextProvider context=store>
                <div class="header-wrapper">
                    <Header title="火车票"/>
                </div>
                <form action="./query.html" class="form">
                    <Journey on_exchange_from_to=Callback::from(move |_| on_exchange_from_to(())) />
                    <DepartDate date_time={local_time}
                    // {...departDateCbs}
                    />
                    <HighSpeed is_high_speed={is_high_speed}
                    //  {...highSpeedCbs}
                      />
                    <Submit />
                </form>

            </StoreModelContextProvider >

        </>
        };
    }
}
pub type Index = FunctionComponent<IndexFC>;
