// use crate::routes::AppRoute;
use yew::{html, Html, Properties};
use yew_functional::{FunctionComponent, FunctionProvider};
// use yew_router::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub is_high_speed: bool,
}

pub struct HighSpeedFC {}
pub type HighSpeed = FunctionComponent<HighSpeedFC>;

impl FunctionProvider for HighSpeedFC {
    type TProps = Props;

    fn run(props: &Self::TProps) -> Html {
        let Props { is_high_speed } = props;

        let checked = if *is_high_speed { "checked" } else { "" };

        return html! {
            <div class="high-speed">
            <div class="high-speed-label">{"只看高铁/动车"}</div>
            <div class="high-speed-switch"
            // onClick={() => toggle()
            // }
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
