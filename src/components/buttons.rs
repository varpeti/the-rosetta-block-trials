use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
struct ButtonProp {
    value: bool,
    onclick: Callback<MouseEvent>,
}

#[function_component(Button)]
fn show_button(props: &ButtonProp) -> Html {
    html! { <td> <span class="buttons" onclick={props.onclick.clone()}> {if props.value {'〇'} else {' '} } </span> </td> }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonsProp {
    pub size: usize,
    pub solution: Vec<usize>,
    pub unlock: Callback<u8>,
}

#[function_component(Buttons)]
pub fn show_buttons(props: &ButtonsProp) -> Html {
    let code = use_state(|| vec![]);

    let mut buttons = vec![];

    for i in 0..props.size {
        let onclick = {
            let code = code.clone();
            let props = props.clone();
            Callback::from(move |_| {
                let mut new_code = Vec::<usize>::new();
                for v in code.iter() {
                    new_code.push(*v);
                }
                new_code.push(i);
                if new_code.len() == props.size {
                    if new_code == props.solution {
                        props.unlock.emit(0);
                    }
                    new_code.clear();
                }
                code.set(new_code);
            })
        };
        if !code.contains(&i) {
            buttons.push(html! { <Button value={true} onclick={onclick}/> });
        } else {
            buttons.push(html! { <Button value={false} onclick={Callback::from(move |_| {})}/> });
        }
    }
    html! {
        <tr>
            {buttons}
        </tr>
    }
}
