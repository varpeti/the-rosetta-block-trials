use yew::prelude::*;

use crate::components::buttons::*;

#[derive(Clone, PartialEq)]
pub struct LevelStruct {
    pub map: Vec<Vec<Block>>,
    pub solution: Vec<usize>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct LevelProp {
    pub level: LevelStruct,
    pub unlock: Callback<u8>,
    pub code: UseStateHandle<Vec<usize>>,
    pub indicator: UseStateHandle<bool>,
    pub flawless: UseStateHandle<bool>,
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
                    Self::Mark => '▢',
                    Self::Mask => '?',
                    Self::Generator => '+',
                    Self::Negative => '▣',
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
    let blocks = props.level.map.iter().map(|line| {
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
            <Buttons size={props.level.map.len()} solution={props.level.solution} unlock={props.unlock} code={props.code} indicator={props.indicator} flawless={props.flawless}/>
        </table>
    }
}

pub fn create_level(size: u8, data: &str) -> LevelStruct {
    if ((size * size + size) as usize) != data.len() {
        panic!("{size} != len('{data}')")
    }
    let mut level = LevelStruct {
        map: vec![],
        solution: vec![],
    };
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
        level
            .solution
            .push((chars.next().expect("Not enough data!!") as u8 - 48) as usize)
    }
    level
}


pub fn get_levels() -> [LevelStruct; 28] {
    [
        // Tutorial 4
        create_level(3, "  . .....012"),
        create_level(3, " . .. ...201"),
        create_level(3, "? .? .?..102"),
        create_level(3, "?  ?.????210"),
        create_level(3, "?? ? ????120"),
        // Bigger 6
        create_level(4, " ????? ?...?????3201"),
        create_level(4, " ?? .?????..????1302"),
        // New toy? 13
        create_level(5, " .   ..   . .   + . . ...43201"),
        create_level(3, "  . . . +012"),
        create_level(3, "   +   ..210"),
        create_level(3, ".     + +120"),
        create_level(3, "  .    ?+012"),
        create_level(4, "?     ? ?     ? 3120"),
        create_level(5, "      ???? ?  ? ?  ? ????02134"),
        // New toy again? 19
        create_level(3, "--..-....102"),
        create_level(4, "--.---.  -.   . 1032"),
        create_level(3, "? -?  ?. 120"),
        create_level(3, " -?  . + 012"),
        create_level(3, "+-    . .210"),
        create_level(3, "? ?     ?102"),
        // Getting serious... 24
        create_level(4, "?   ??    ??  ? 3021"),
        create_level(4, "? ???? ? ? ?    3021"),
        create_level(4, "??  ??    ??  ??3021"),
        create_level(4, "?  ?  ?? ?  ??  0312"),
        create_level(4, "??   ?    ???  ?2310"),
        // Completed! 27
        create_level(5, "?????  ?      ? ? ? ?    10243"),
        create_level(5, "?? ????          ??   ?? 43021"),
        create_level(6, "??  ???    ?          ?? ?? ?   ?   123054"),
    ]
}
