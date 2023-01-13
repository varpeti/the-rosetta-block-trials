use yew::prelude::*;

use crate::components::level::*;

#[function_component(Selector)]
pub fn show_level_selector() -> Html {
    let level_id = use_state(|| 0); // Stores the current level id
    let unlocked_level = use_state(|| 0); // Stores the higest level unlocked
    let code = use_state(|| vec![]); // Stores the current code entered by the player
    let indicator = use_state(|| false); // Indicate the last answer correctness
 
    // TODO add credit
    // TODO add achivements

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
        create_level(3, ".     + +120"),
        create_level(3, "  .    ?+012"),
        create_level(4, "?     ? ?     ? 3120"),
        create_level(5, "      ???? ?  ? ?  ? ????02134"),
        create_level(3, "--..-....102"),
        create_level(4, "--.---.  -.   . 1032"),
        create_level(3, "? -?  ?. 120"),
        create_level(3, " -?  . + 012"),
        create_level(3, "+-    . .210"),
        create_level(3, "? ?     ?102"),
        create_level(4, "?   ??    ??  ? 3021"),
        create_level(4, "? ???? ? ? ?    3021"),
        create_level(4, "??  ??    ??  ??3021"),
        create_level(4, "?  ?  ?? ?  ??  0312"),
        create_level(4, "??   ?    ???  ?2310"),
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
        let code = code.clone();
        let indicator = indicator.clone();
        Callback::from(move |_| {
            if *level_id + 1 <= *unlocked_level && *level_id < levels_len - 1 {
                level_id.set(*level_id + 1);
                code.set(vec![]);
                indicator.set(false);
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
        let code = code.clone();
        let indicator = indicator.clone();
        Callback::from(move |_| {
            if *level_id > 0 {
                level_id.set(*level_id - 1);
                code.set(vec![]);
                indicator.set(false);
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
                unlocked_level.set(*level_id + 1);
            }
        })
    };

    html! {
        <table>
            <tr>
                {backward_button}
                <td> <Level level={level} unlock={unlock} code={code} indicator={indicator} /> </td>
                {forward_button}
            </tr>
        </table>
    }
}
