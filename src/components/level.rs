use yew::prelude::*;

use crate::components::buttons::*;

#[derive(Clone, PartialEq, Properties)]
pub struct LevelProp {
    pub map: Vec<Vec<Block>>,
    pub solution: Vec<usize>,
    pub unlock: Callback<u8>,
}

impl LevelProp {
    fn new() -> Self {
        Self{map: vec![], solution: vec![], unlock: Callback::from(move |_| {}) }
    }
}

#[derive(Clone, PartialEq)]
pub enum Block {
    Air,
    Mark,
    Mask,
    Generator,
    Negative,
}

impl Block {
    fn get_html(&self) -> Html {
        html! {
            <td><span class={"block"}>
            {
                match self {
                    Self::Air => ' ', //non-breaking space
                    Self::Mark => '☐',
                    Self::Mask => '?',
                    Self::Generator => '+',
                    Self::Negative => '☒',
                }
            }
            </span></td>
        }
    }

    fn new(c: char) -> Self {
        // panic!("AAA5 {c}");
        match c {
            ' ' => Self::Air,
            '.' => Self::Mark,
            '?' => Self::Mask,
            '+' => Self::Generator,
            '-' => Self::Negative,
            _ => Self::Negative,
        }
    }
}

#[function_component(Level)]
pub fn show_level(props: &LevelProp) -> Html {
    let blocks = props.map.iter().map(|line| {
        let line = line.iter().map(|block| block.get_html());
        html! {
        <tr>
            {line.collect::<Html>()}
        </tr>
        }
    });
    let props = props.clone();
    html! {
        <table>
            { blocks.collect::<Html>() }
            <Buttons size={props.map.len()} solution={props.solution} unlock={props.unlock} />
        </table>
    }
}

pub fn create_level(size: u8, data: &str) -> LevelProp {
    if ((size * size + size) as usize) != data.len() {
        panic!("{size} != len('{data}')")
    }
    let mut level = LevelProp::new(); 
    let mut chars = data.chars();
    for _y in 0..size {
        let mut line = vec![];
        for _x in 0..size {
            line.push(Block::new(chars.next().expect("Not enough data!")));
        }
        level.map.push(line);
    }
    level.solution = vec![];
    for _i in 0..size {
        level.solution.push( (chars.next().expect("Not enough data!!") as u8 - 48) as usize )
    }
    level
}
