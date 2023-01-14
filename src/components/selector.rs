use yew::prelude::*;

use crate::components::level::*;

const EXTRA_PANELS_BEFORE: usize = 2;
const EXTRA_PANELS_AFTER: usize = 1;

#[function_component(Selector)]
pub fn show_level_selector() -> Html {
    let panel_id = use_state(|| EXTRA_PANELS_BEFORE); // Stores the current panel id
    let unlocked_level = use_state(|| 0); // Stores the higest level unlocked
    let code = use_state(|| vec![]); // Stores the current code entered by the player
    let indicator = use_state(|| false); // Indicate the last answer correctness

    let levels = get_levels();

    let mut level_id: Option<usize> = None;
    if *panel_id >= EXTRA_PANELS_BEFORE && *panel_id < EXTRA_PANELS_BEFORE + levels.len() {
        level_id = Some(*panel_id - EXTRA_PANELS_BEFORE);
    }

    let forward = {
        let panel_id = panel_id.clone();
        let unlocked_level = unlocked_level.clone();
        let code = code.clone();
        let indicator = indicator.clone();
        Callback::from(move |_| {
            if level_id.is_none() || level_id.expect("level_id") < *unlocked_level {
                panel_id.set(*panel_id + 1);
                code.set(vec![]);
                indicator.set(false);
            }
        })
    };

    let forward_button;
    if (level_id.is_none()
        && *panel_id + 1 < levels.len() + EXTRA_PANELS_BEFORE + EXTRA_PANELS_AFTER)
        || (level_id.is_some() && level_id.expect("level_id") < *unlocked_level)
    {
        forward_button = html! { <td class={"nav"}> <span class={"nav_button"} onclick={forward}>{'>'}</span> </td> };
    } else {
        forward_button = html! { <td class={"nav"}></td> }
    }

    let backward = {
        let panel_id = panel_id.clone();
        let code = code.clone();
        let indicator = indicator.clone();
        Callback::from(move |_| {
            if *panel_id > 0 {
                panel_id.set(*panel_id - 1);
                code.set(vec![]);
                indicator.set(false);
            }
        })
    };

    let backward_button;
    if *panel_id > 0 {
        backward_button = html! { <td class={"nav"}> <span class={"nav_button"} onclick={backward}>{'<'}</span> </td> };
    } else {
        backward_button = html! { <td class={"nav"}></td> }
    }

    let unlock = {
        let unlocked_level = unlocked_level.clone();
        Callback::from(move |_| {
            if let Some(level_id) = level_id {
                if level_id >= *unlocked_level {
                    unlocked_level.set(level_id + 1);
                }
            }
        })
    };

    // Render panels

    if *panel_id == 0 {
        return show_credit(forward_button);
    }

    if *panel_id == 1 {
        return show_achievements(
            String::from("Achievements"),
            forward_button,
            backward_button,
            *unlocked_level,
            levels.len(),
        );
    }

    if level_id.is_none() {
        // All levels are done
        return show_achievements(
            String::from("Congratulation!"),
            forward_button,
            backward_button,
            *unlocked_level,
            levels.len(),
        );
    }

    let level = levels
        .get(
            level_id
                .expect(format!("The level_id is None. The panel_id is {}.", *panel_id).as_str()),
        )
        .expect(format!("The {} level does not exists!", level_id.expect("level_id")).as_str())
        .clone();

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

fn show_credit(forward_button: Html) -> Html {
    html! {
        <table>
            <tr>
                <table>
                    <tr> <th colspan="2"> {"Credit"} </th> </tr>
                    <tr>
                        <td class="credit">{"Puzzle:"}</td>
                        <td class="credit"><a href="https://mmcelebration.com/level/4/31/" target="_blank">{"rubenscube"}</a></td>
                    </tr>
                    <tr>
                        <td class="credit">{"Tech:"}</td>
                        <td class="credit"><a href="https://yew.rs/" target="_blank">{"Yew + 🦀"}</a></td>
                    </tr>
                    <tr>
                        <td class="credit">{"Colors:"}</td>
                        <td class="credit"><a href="https://rosepinetheme.com" target="_blank">{"Rosé Pine"}</a></td>
                    </tr>
                    <tr>
                        <td class="credit">{"Programer:"}</td>
                        <td class="credit"><a href="https://github.com/varpeti" target="_blank">{"varpeti"}</a></td>
                    </tr>
                </table>
                {forward_button}
            </tr>
        </table>
    }
}

fn show_achievements(
    msg: String,
    forward_button: Html,
    backward_button: Html,
    unlocked_level: usize,
    levels_len: usize,
) -> Html {
    html! {
        <table>
            <tr>
                {backward_button}
                <p> {msg} </p>
                <p> {unlocked_level}{"/"}{levels_len} </p>
                //TODO
                {forward_button}
            </tr>
        </table>
    }
}
