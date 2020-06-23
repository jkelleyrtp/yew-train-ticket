// use crate::routes::AppRoute;
use yew::{html, Callback, Html, Properties};
use yew_functional::{FunctionComponent, FunctionProvider};
// use yew_router::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub is_high_speed: bool,
    // pub on_toggle_is_high_speed: Callback<()>,
}

pub struct HighSpeedFC {}
pub type HighSpeed = FunctionComponent<HighSpeedFC>;

impl FunctionProvider for HighSpeedFC {
    type TProps = Props;

    fn run(props: &Self::TProps) -> Html {
        let Props {
            is_high_speed,
            // on_toggle_is_high_speed,
        } = props;
        // let on_toggle_is_high_speed = on_toggle_is_high_speed.clone();
        let checked = if *is_high_speed { "checked" } else { "" };

        return html! {
            <div class="high-speed">
            <div class="high-speed-label">{"只看高铁/动车"}</div>
            <div class="high-speed-switch"
            // onclick=Callback::from(move |_| on_toggle_is_high_speed.emit(()))

            >
                <input type="hidden" name="highSpeed" value={is_high_speed} />
                <div
                    class=format!("{} {}", "high-speed-track", checked)
                >
                    <span
                    class=format!("{} {}", "high-speed-handle", checked)
                    ></span>
                </div>
            </div>
        </div>
        };
    }
}
