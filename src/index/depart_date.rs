// use crate::routes::AppRoute;
use yew::{html, Html};
use yew_functional::{use_context, FunctionComponent, FunctionProvider};
// use yew_router::prelude::*;
use crate::store::store::{Action, StoreDispatch, StoreModel};
use chrono::prelude::*;

use std::rc::Rc;

pub struct DepartDateFC {}
pub type DepartDate = FunctionComponent<DepartDateFC>;

impl FunctionProvider for DepartDateFC {
    type TProps = ();

    fn run(_: &Self::TProps) -> Html {
        let context = use_context::<Rc<StoreModel>>();
        let ctx = &context.unwrap();
        let StoreModel {
            local_time: date_time,
            ..
        } = &***ctx;

        let week = date_time.weekday();
        let time = date_time.format("%Y-%m-%d").to_string();
        let weekday_str = match week {
            chrono::Weekday::Mon => "周一",
            chrono::Weekday::Tue => "周二",
            chrono::Weekday::Wed => "周三",
            chrono::Weekday::Thu => "周四",
            chrono::Weekday::Fri => "周五",
            chrono::Weekday::Sat => "周六",
            chrono::Weekday::Sun => "周日",
        };

        return html! {
            <div class="depart-date"
            // nClick={onClick}
            >
                <input type="hidden" name="date"
                value=time
                />
                {time}
                <span class="depart-week">{weekday_str}{"(今天)"}</span>
            </div>
        };
    }
}
