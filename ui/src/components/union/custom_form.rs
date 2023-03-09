use std::ops::Deref;

use yew::prelude::*;
use crate::components::unit::{
    text_input::TextInput, 
    text_button::TextButton
};

#[derive(Default, Clone)]
struct Data {
    username: String,
    saved: bool,
}

#[function_component(CustomForm)]
pub fn custom_form() -> Html {
    let state = use_state(|| Data::default());
    let cloned_state = state.clone();
    let username_changed = Callback::from(move |username| {
        cloned_state.set(
            Data {
                username,
                saved: true,
            }
        );
    });

    let cloned_state = state.clone();
    let button_clicked = Callback::from(move |_| {
        cloned_state.set(
            Data {
                saved: true,
                ..cloned_state.deref().clone()
            }
        );
    });
    html!{
        <div>
            <TextInput name="User Name" handle_onchange={username_changed}/>
            <p />
            <TextButton label="Save" onclick={button_clicked} />
            <p>{"Username: "}{&state.username}{if !state.saved {" (Not saved)"} else {" (Saved)"}}</p>
        </div>

    }
}