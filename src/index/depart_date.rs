// use crate::routes::AppRoute;
use yew::{html, Html, Properties};
use yew_functional::{FunctionComponent, FunctionProvider};
// use yew_router::prelude::*;

use chrono::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub date_time: DateTime<Local>,
}

pub struct DepartDateFC {}
pub type DepartDate = FunctionComponent<DepartDateFC>;

impl FunctionProvider for DepartDateFC {
    type TProps = Props;

    fn run(props: &Self::TProps) -> Html {
        let Props { date_time } = &props;
        // let is_today = date_time.
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
