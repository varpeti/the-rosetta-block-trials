use std::collections::HashSet;
use yew::prelude::*;

use crate::components::achievements::*;
use crate::components::credit::*;
use crate::components::level::*;

const EXTRA_PANELS_BEFORE: usize = 2;
const EXTRA_PANELS_AFTER: usize = 1;

#[function_component(Selector)]
pub fn show_level_selector() -> Html {
    let panel_id = use_state(|| EXTRA_PANELS_BEFORE); // Stores the current panel id
    let unlocked_level = use_state(|| 0); // Stores the higest level unlocked
    let code = use_state(|| vec![]); // Stores the current code entered by the player
    let indicator = use_state(|| false); // Indicate the last answer correctness
    let achis = use_state(|| HashSet::<Achi>::from([])); // Stores the earned Achievements
    let flawless = use_state(|| true); // Turns false if there any mistake

    // Level

    let levels = get_levels();

    let mut level_id: Option<usize> = None;
    if *panel_id >= EXTRA_PANELS_BEFORE && *panel_id < EXTRA_PANELS_BEFORE + levels.len() {
        level_id = Some(*panel_id - EXTRA_PANELS_BEFORE);
    }

    // Nav buttons

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
        let achis = achis.clone();
        Callback::from(move |_| {
            if let Some(level_id) = level_id {
                if level_id >= *unlocked_level {
                    unlocked_level.set(level_id + 1);
                    if let Some((_, achi)) = Achi::get_req().iter().find(|(&id, _)| id == level_id)
                    {
                        let mut a = achis.iter().cloned().collect::<HashSet<Achi>>();
                        a.insert(achi.clone());
                        achis.set(a);
                    }
                }
            }
        })
    };

    // Achi

    if achis.contains(&Achi::Completed) && *flawless && !achis.contains(&Achi::Flawless) {
        let mut a = achis.iter().cloned().collect::<HashSet<Achi>>();
        a.insert(Achi::Flawless);
        achis.set(a);
    }

    // Render panels

    if *panel_id == 0 {
        if !achis.contains(&Achi::Credit) {
            let mut a = achis.iter().cloned().collect::<HashSet<Achi>>();
            a.insert(Achi::Credit);
            achis.set(a);
        }

        return html! { <Credit forward_button={forward_button}/> };
    }

    if *panel_id == 1 {
        return html! {
            <Achievements msg={"Achievements"} forward_button={forward_button} backward_button={backward_button} unlocked_level={*unlocked_level} levels_len={levels.len()} achis={(*achis).clone()} />
        };
    }

    if level_id.is_none() {
        // All levels are done
        return html! {
            <Achievements msg={"Congratulation!"} forward_button={forward_button} backward_button={backward_button} unlocked_level={*unlocked_level} levels_len={levels.len()} achis={(*achis).clone()} />
        };
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
                <td> <Level level={level} unlock={unlock} code={code} indicator={indicator} flawless={flawless} /> </td>
                {forward_button}
            </tr>
        </table>
    }
}
