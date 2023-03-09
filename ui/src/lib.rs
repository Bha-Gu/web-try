use yew::prelude::*;

mod components;
use components::unit::main_title::MainTitle;
use components::union::custom_form::CustomForm;

#[function_component(App)]
pub fn app() -> Html {
    html!{
        <div>
            <MainTitle title="Title" />
            <CustomForm />
        </div>
    }
}