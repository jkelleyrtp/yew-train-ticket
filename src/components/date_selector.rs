// use crate::routes::AppRoute;
use yew::{html, Callback, Html, Properties};
use yew_functional::{use_context, FunctionComponent, FunctionProvider};
// use yew_router::prelude::*;
use crate::store::store::{Action, StoreDispatch, StoreModel};
// use yew_router::prelude::*;
use crate::components::header::Header;
use chrono::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct DayProps {
    pub date: Option<DateTime<Local>>,
}
pub struct DayFC {}
pub type Day = FunctionComponent<DayProps>;
impl FunctionProvider for DayProps {
    type TProps = DayProps;

    fn run(props: &Self::TProps) -> Html {
        let DayProps { date } = &props;

        let (day, is_today) = match *date {
            Some(day) => {
                let now = Local::now();
                (
                    day.day().to_string(),
                    now.month() == day.month() && now.day() == day.day(),
                )
            }
            None => ("".to_string(), false),
        };

        return html! {
            <td
            // class={classnames(classes)
            // }
            // onClick={() => onSelect(day)}
            >
            { if is_today { "今天".to_string() } else { day } }
         </td>
        };
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct WeekProps {
    pub week: Vec<Option<DateTime<Local>>>,
}
pub struct WeekFC {}
pub type Week = FunctionComponent<WeekFC>;
impl FunctionProvider for WeekFC {
    type TProps = WeekProps;

    fn run(props: &Self::TProps) -> Html {
        let WeekProps { week } = &props;

        return html! {
            <tr
            class="date-table-days"
            // class={classnames(classes)
            // }
            // onClick={() => onSelect(day)}
            >
            {for week.iter().map(|date| {
                html! { <Day date=date /> }
            })}
        </tr>
        };
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct MonthProps {
    pub date: DateTime<Local>,
}
pub struct MonthFC {}
pub type Month = FunctionComponent<MonthFC>;
impl FunctionProvider for MonthFC {
    type TProps = MonthProps;

    fn run(props: &Self::TProps) -> Html {
        let MonthProps { date } = &props;
        let month = date.month();
        let next_month_date = date.with_month(month + 1).unwrap_or(*date);
        let year_month = date.format("%Y年%m月").to_string();
        let first_day_in_month = date.with_day(1).unwrap_or(*date);
        let mut day_in_month = date.with_day(1).unwrap_or(*date);
        let mut day_list: Vec<Option<DateTime<Local>>> = vec![];

        let weekday = first_day_in_month.weekday();
        let offset = match weekday {
            chrono::Weekday::Mon => 0,
            chrono::Weekday::Tue => 1,
            chrono::Weekday::Wed => 2,
            chrono::Weekday::Thu => 3,
            chrono::Weekday::Fri => 4,
            chrono::Weekday::Sat => 5,
            chrono::Weekday::Sun => 6,
        };

        for _ in 0..offset {
            day_list.push(None);
        }

        // 先算出当月日期数组
        while month == day_in_month.month() {
            let day = day_in_month.day();
            day_list.push(Some(day_in_month));
            day_in_month = day_in_month.with_day(day + 1).unwrap_or(next_month_date);
        }

        let rest = day_list.iter().len() % 7;

        for _ in 0..(7 - rest) {
            day_list.push(None);
        }

        let len = day_list.iter().len();

        let rows = len / 7;

        let matrix: Vec<Vec<Option<DateTime<Local>>>> = (0..rows)
            .map(|i| {
                return (0..7)
                    .map(|ii| {
                        let xy = i * 7 + ii;
                        return day_list[xy];
                    })
                    .collect();
            })
            .collect();

        return html! {
            <table class="date-table">
            <thead>
                <tr>
                    <td colSpan="7">
                        <h5>
                         {year_month}
                        </h5>
                    </td>
                </tr>
            </thead>
            <tbody>
                <tr class="data-table-weeks">
                    <th>{"周一"}</th>
                    <th>{"周二"}</th>
                    <th>{"周三"}</th>
                    <th>{"周四"}</th>
                    <th>{"周五"}</th>
                    <th class="weekend">{"周六"}</th>
                    <th class="weekend">{"周日"}</th>
                </tr>
                {for matrix.iter().map(|week| {
                    html! { <Week week=week /> }
                })}
            </tbody>
        </table>
        };
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub show: bool,
}
pub struct DateSelectorFC {}
pub type DateSelector = FunctionComponent<DateSelectorFC>;
impl FunctionProvider for DateSelectorFC {
    type TProps = Props;

    fn run(props: &Self::TProps) -> Html {
        let Props { show } = &props;

        let local_time = Local::now();
        let month = local_time.month();
        let local_time1 = local_time.with_month(month + 1).unwrap_or(local_time);
        let local_time2 = local_time.with_month(month + 2).unwrap_or(local_time);

        let date_list: Vec<DateTime<Local>> = vec![local_time, local_time1, local_time2];

        let hidden_class = if *show { "" } else { "hidden" };

        let context_dispatch = use_context::<StoreDispatch>();
        let onclick = Callback::from(move |_| match &context_dispatch {
            Some(dispatch) => {
                let dispatch = &*dispatch;
                dispatch.emit(Action::ToggleDateSelectorVisible);
                return ();
            }
            _ => (),
        });

        return html! {
            <div class=format!("date-selector {}", hidden_class) >
            <Header title="日期选择"
              onback=Some(onclick)
            // onBack={onBack}
            />
            <div class="date-selector-tables">
            {for date_list.iter().map(|date| {
                html! { <Month date=date /> }
            })}
            </div>
        </div>
        };
    }
}
