use self::Achi::*;
use std::collections::{HashSet, HashMap};

use yew::prelude::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Achi {
    Tutorial,
    Bigger,
    NewToy,
    NewToyAgain,
    GettingSerious,
    Completed,
    Credit,
    Flawless,
}

impl Achi {
    pub fn iter() -> impl Iterator<Item = Achi> {
        [
            Credit,
            Tutorial,
            Bigger,
            NewToy,
            NewToyAgain,
            GettingSerious,
            Completed,
            Flawless,
        ]
        .iter()
        .cloned()
    }
    pub fn to_string(&self) -> &str {
        match self {
            &Tutorial => "Tutorial",
            &Bigger => "Bigger",
            &NewToy => "+",
            &NewToyAgain => "▣",
            &GettingSerious => "Getting serious...",
            &Completed => "Completed!",
            &Credit => "Credit",
            &Flawless => "Flawless",
        }
    }

    pub fn get_req() -> HashMap<usize,Achi> {
        let mut ret = HashMap::new();
        ret.insert(4, Tutorial);
        ret.insert(6, Bigger);
        ret.insert(13, NewToy);
        ret.insert(19, NewToyAgain);
        ret.insert(24, GettingSerious);
        ret.insert(27, Completed);
        ret
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct AchievementsProps {
    pub msg: String,
    pub unlocked_level: usize,
    pub levels_len: usize,
    pub achis: HashSet<Achi>,
}

#[function_component(Achievements)]
pub fn show_achievements(props: &AchievementsProps) -> Html {
    let achis = Achi::iter().map(|achi| {
        if props.achis.contains(&achi) {
            html! {
                <span class="show tooltip"> {"✷"}
                    <span class="tooltiptext"> {achi.to_string()} </span>
                </span>
            }
        } else {
            html! {
                <span class="locked"> {"✷"} </span>
            }
        }
    });

    let props = props.clone();
    html! {
        <td>
            <p> {props.msg} </p>
            <p> {props.unlocked_level}{"/"}{props.levels_len} </p>
            <p> {achis.collect::<Vec<Html>>()} </p>
        </td>
    }
}


#[derive(Clone, PartialEq, Properties)]
pub struct NewAchievementProps {
    pub new_achi: Option<Achi>,
}

#[function_component(NewAchievement)]
pub fn show_new_achievement(props: &NewAchievementProps) -> Html {
    let props = props.clone();
    if let Some(achi) = props.new_achi {
        html! {
            <span class="new_achi show tooltip"> {"✷"}
                <span class="tooltiptext"> {"New achievement:"} <br/> {achi.to_string()} </span>
            </span>
        }
    } else {
     html!{}
    }
}
