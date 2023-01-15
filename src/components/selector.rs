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
    let new_achi: UseStateHandle<Option<Achi>> = use_state(|| None); // Shows the newly earned acchivement
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
        let new_achi = new_achi.clone();
        Callback::from(move |_| {
            if level_id.is_none() || level_id.expect("level_id") < *unlocked_level {
                panel_id.set(*panel_id + 1);
                code.set(vec![]);
                indicator.set(false);
                if new_achi.is_some() {
                    new_achi.set(None);
                }
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
        let new_achi = new_achi.clone();
        Callback::from(move |_| {
            if *panel_id > 0 {
                panel_id.set(*panel_id - 1);
                code.set(vec![]);
                indicator.set(false);
                if new_achi.is_some() {
                    new_achi.set(None);
                }
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
        let new_achi = new_achi.clone();
        Callback::from(move |_| {
            if let Some(level_id) = level_id {
                if level_id >= *unlocked_level {
                    // Unlock
                    unlocked_level.set(level_id + 1);
                    // Achi
                    if let Some((_, achi)) = Achi::get_req().iter().find(|(&id, _)| id == level_id)
                    {
                        let mut a = achis.iter().cloned().collect::<HashSet<Achi>>();
                        a.insert(achi.clone());
                        achis.set(a);
                        new_achi.set(Some(achi.clone()));
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
        new_achi.set(Some(Achi::Flawless));
    }

    // Render panels

    let panel = match *panel_id {
        0 => {
            if !achis.contains(&Achi::Credit) {
                let mut a = achis.iter().cloned().collect::<HashSet<Achi>>();
                a.insert(Achi::Credit);
                achis.set(a);
                new_achi.set(Some(Achi::Credit));
            }

            html! { <Credit/> }
        }
        1 => {
            html! { <Achievements msg={"Achievements"} unlocked_level={*unlocked_level} levels_len={levels.len()} achis={(*achis).clone()} /> }
        }
        _ if level_id.is_none() => {
            // All levels are done
            html! { <Achievements msg={"Congratulation!"} unlocked_level={*unlocked_level} levels_len={levels.len()} achis={(*achis).clone()} /> }
        }

        _ => {
            let level = levels
                .get(level_id.expect(
                    format!("The level_id is None. The panel_id is {}.", *panel_id).as_str(),
                ))
                .expect(
                    format!("The {} level does not exists!", level_id.expect("level_id")).as_str(),
                )
                .clone();

            html! { <Level level={level} unlock={unlock} code={code} indicator={indicator} flawless={flawless} /> }
        }
    };

    html! {
        <table>
            <NewAchievement new_achi={(*new_achi).clone()}/>
            <tr>
                {backward_button}
                <td> {panel} <td>
                {forward_button}
            </tr>
        </table>
    }
}
