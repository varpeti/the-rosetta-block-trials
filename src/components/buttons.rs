use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
struct ButtonProp {
    value: bool,
    onclick: Callback<MouseEvent>,
    indicator: bool,
}

#[function_component(Button)]
fn show_button(props: &ButtonProp) -> Html {
    let props = props.clone();
    html! { 
        <td> 
            <span class="buttons" onclick={props.onclick} indicator={props.indicator.to_string()}> 
                { 
                    match (props.value, props.indicator) {
                        (true, true) => '▢',
                        (true, false) => '◯',
                        (false, _) => ' '
                    }
                } 
            </span> 
        </td> 
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonsProp {
    pub size: usize,
    pub solution: Vec<usize>,
    pub unlock: Callback<u8>,
    pub code: UseStateHandle<Vec<usize>>,
    pub indicator: UseStateHandle<bool>,
    pub flawless: UseStateHandle<bool>,
}

#[function_component(Buttons)]
pub fn show_buttons(props: &ButtonsProp) -> Html {
    let mut buttons = vec![];

    for i in 0..props.size {
        let onclick = {
            let props = props.clone();
            Callback::from(move |_| {
                let mut new_code = props.code.iter().cloned().collect::<Vec<usize>>();
                new_code.push(i);
                if new_code.len() == props.size {
                    if new_code == props.solution {
                        props.unlock.emit(0); // unlock next level
                        props.indicator.set(true);
                    } else {
                        props.indicator.set(false);
                        props.flawless.set(false);
                    }
                    new_code.clear();
                }
                props.code.set(new_code);
            })
        };
        if !props.code.contains(&i) {
            buttons.push(html! { <Button value={true} onclick={onclick} indicator={*props.indicator}/> });
        } else {
            buttons.push(html! { <Button value={false} onclick={Callback::from(move |_| {})} indicator={*props.indicator}/> });
        }
    }
    html! {
        <tr>
            {buttons}
        </tr>
    }
}
