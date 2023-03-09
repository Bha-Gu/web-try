use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: Option<String>,
}

#[function_component(MainTitle)]
pub fn main_title(props: &Props) -> Html {
    html!{
        <h1>
            {
                if let Some(title) = &props.title {
                    title
                } else {
                    "This is main title"
                }
            }
        </h1>
    }
}