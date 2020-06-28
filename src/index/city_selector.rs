
use yew::{html, Callback, Html, MouseEvent,Properties};

use yew_functional::{use_context,use_effect_with_deps,use_state, FunctionComponent, FunctionProvider};
use crate::store::store::{Action, StoreDispatch, StoreModel};
use std::rc::Rc;
use crate::service::api::get_city_list;
use crate::service::future::handle_future;
use crate::service::api::{CityResult,City};



#[derive(Properties, Clone, PartialEq)]
pub struct AlphaIndexProps {
    pub alpha: char
}
pub struct AlphaIndexFC {}
pub type AlphaIndex = FunctionComponent<AlphaIndexFC>;
impl FunctionProvider for AlphaIndexFC {
    type TProps = AlphaIndexProps;

    fn run(props: &Self::TProps) -> Html {
        let AlphaIndexProps { alpha } = &props;

        return html! {
            <i class="city-index-item" 
            // onClick={() => onClick(alpha)}
            >
                {alpha}
            </i>
        };
    }
}


#[derive(Properties, Clone, PartialEq)]
pub struct CityItemProps {
    pub name: String
}
pub struct CityItemFC {}
pub type CityItem = FunctionComponent<CityItemFC>;
impl FunctionProvider for CityItemFC {
    type TProps = CityItemProps;

    fn run(props: &Self::TProps) -> Html {
        let CityItemProps { name } = &props;

        return html! {
            <li class="city-li" 
            // onClick={() => onSelect(name)}
            >
            {name}
        </li>
        };
    }
}



#[derive(Properties, Clone, PartialEq)]
pub struct CitySectionProps {
    pub city_names: Vec<String>,
    pub title: String
}
pub struct CitySectionFC {}
pub type CitySection = FunctionComponent<CitySectionFC>;
impl FunctionProvider for CitySectionFC {
    type TProps = CitySectionProps;

    fn run(props: &Self::TProps) -> Html {
        let CitySectionProps { title,city_names } = &props;

        return html! {
            <ul class="city-ul">
                <li class="city-li"  data-cate={title}>
                    {title}
                </li>
                {for city_names.iter().map(|name| {
                    html!{
                        <CityItem
                            name={name}
                            // onSelect={onSelect}
                        />}
                    
                })}
             </ul>
        };
    }
}


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


        let show1 = show.clone();

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


        let (loading, set_loading) = use_state(||false);


        let (search_word, set_search_word) = use_state(||"".to_string());

        let has_search_word = search_word.len() > 0;

        let clear_hidden_class = if has_search_word {""} else { "hidden"};

        let set_search_word =Rc::new(set_search_word);
        let set_search_word1 = Rc::clone(&set_search_word);

        let oninput :Callback<yew::html::InputData>  = Callback::from(move |evt:yew::html::InputData|set_search_word(evt.value));

        let onclear :Callback<MouseEvent>  = Callback::from(move |_|set_search_word1("".to_string()));


        let (city_data, set_city_data) = use_state(||CityResult::new());
        let sections =&(*city_data).cityList;

        let alphabet: Vec<char> = "ABCDEFGIJKLMNOPQRSTUVWXYZ".chars().collect();

        use_effect_with_deps(
            move|_| {

                if show1  {
                    set_loading(true);
                    let future = async {
                        get_city_list().await
                    };
    
                    handle_future(future,move |value :CityResult| {                
                        set_loading(false);
                        set_city_data(value)}
                );
                }

                return || ();
            },
            (show1),
        );

        return html! {
            <div
            class=format!("{} {}", "city-selector" ,hidden_class )
            >
            <div class="city-search">
                <div class="search-back"
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
                        value=search_word
                        class="search-input"
                        placeholder="城市、车站的中文或拼音"
                        oninput=oninput
                    />
                </div>
                <div
                    onclick=onclear
                    class=format!("{} {}", "search-clean" ,clear_hidden_class )   
                >
                   { "x"}
                </div>
            </div>
            // {Boolean(key) && (
            //     <Suggest searchKey={key} onSelect={key => onSelect(key)} />
            // )}
            {if *loading { html! {<div>{"loading"}</div>} } else { 
            html! {   
                <div class="city-list">
                    <div class="city-cate">
                        {for sections.iter().map(move |section| {
                            let city_names: Vec<String> = section.citys.iter().map(|city|{
                                let City {name} =city;
                                name.to_string()
                            }).collect();
                            html! {
                                <CitySection
                                    title={section.title.clone()}
                                    city_names=city_names
                                    // onSelect={onSelect}
                                />
                            }
                        })}
                    </div>
                    <div class="city-index">
                        {for alphabet.iter().map(|alpha|{
                            html! {
                                <AlphaIndex
                                    alpha={alpha}
                                    // onClick={toAlpha}
                                />
                            }
                        })}
                    </div>
                </div>
                } } }
                // {outputCitySections()}
            </div>
        };
    }
}
