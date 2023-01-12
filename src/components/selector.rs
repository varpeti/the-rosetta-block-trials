use yew::prelude::*;

use crate::components::level::*;

#[function_component(Selector)]
pub fn show_level_selector() -> Html {
    let level_id = use_state(|| 0);
    let unlocked_level = use_state(|| 0);

    // TODO move code state here -> refresh code after map change
    // TODO feedback for good/bad solution
    
    // TODO add credit

    let levels = vec![
        create_level(3, "  . .....012"),
        create_level(3, " . .. ...201"),
        create_level(3, "? .? .?..102"),
        create_level(3, "?  ?.????210"),
        create_level(3, "?? ? ????120"),
        create_level(4, " ????? ?...?????3201"),
        create_level(4, " ?? .?????..????1302"),
        create_level(5, " .   ..   . .   + . . ...43201"),
        create_level(3, "  . . . +012"),
        create_level(3, "   +   ..210"),
        // TODO add levels
    ];
    
    let levels_len = levels.len();

    let level = levels
        .get(*level_id)
        .expect("The {level_id} level does not exists!")
        .clone();

    let forward = {
        let level_id = level_id.clone();
        let unlocked_level = unlocked_level.clone();
        Callback::from(move |_| {
            if *level_id + 1 <= *unlocked_level && *level_id < levels_len - 1 {
                level_id.set(*level_id + 1);
            }
        })
    };

    let forward_button;
    if *level_id + 1 <= *unlocked_level {
        forward_button = html! { <td class={"nav"} onclick={forward}>{'>'}</td> };
    } else {
        forward_button = html! { <td class={"nav"}>{' '}</td> }
    }

    let backward = {
        let level_id = level_id.clone();
        Callback::from(move |_| {
            if *level_id > 0 {
                level_id.set(*level_id - 1);
            }
        })
    };

    let backward_button;
    if *level_id > 0 {
        backward_button = html! { <td class={"nav"} onclick={backward}>{'<'}</td> };
    } else {
        backward_button = html! { <td class={"nav"}>{' '}</td> }
    }

    let unlock = {
        let level_id = level_id.clone();
        let unlocked_level = unlocked_level.clone();
        Callback::from(move |_| {
            if *level_id + 1 > *unlocked_level {
                unlocked_level.set(*level_id + 1)
            }
        })
    };

    html! {
        <table>
            <tr>
                {backward_button}
                <td> <Level map={level.map} solution={level.solution} unlock={unlock} /> </td>
                {forward_button}
            </tr>
        </table>
    }
}
