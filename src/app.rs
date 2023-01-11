use yew::prelude::*;

struct Level {
    map: Vec<Vec<Block>>,
    solution: Vec<usize>,
}

enum Block {
    Air,
    Mark,
    Mask,
    Generator,
    Negative,
}

impl Block {
    fn get_html(&self) -> Html {
        html! {
            <td>
            {
                match self {
                    Self::Air => ' ', //non-breaking space
                    Self::Mark => '☐',
                    Self::Mask => '?',
                    Self::Generator => '+',
                    Self::Negative => '☒',
                }
            }
            </td>
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

fn show_level(level: Level) -> Html {
    let blocks = level.map.iter().map(|line| {
        let line = line.iter().map(|block| block.get_html());
        html! {
        <tr>
            {line.collect::<Html>()}
        </tr>
        }
    });

    html! {
        <table>
            { blocks.collect::<Html>() }
            <Buttons size={level.map.len()} solution={level.solution}/>
        </table>
    }
}

fn create_level(size: u8, data: String) -> Level {
    if ((size * size + size) as usize) != data.len() {
        panic!("{size} != len('{data}')")
    }
    let mut level = Level{map: vec![], solution: vec![]};
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

#[derive(Clone, PartialEq, Properties)]
struct ButtonProp {
    value: bool,
    onclick: Callback<MouseEvent>,
}

#[function_component(Button)]
fn show_button(props: &ButtonProp) -> Html {
    html! { <td> <span onclick={props.onclick.clone()}> {if props.value {'〇'} else {' '} } </span> </td> }
}

#[derive(Clone, PartialEq, Properties)]
struct ButtonsProp {
    size: usize,
    solution: Vec<usize>,
}

#[function_component(Buttons)]
fn show_buttons(props: &ButtonsProp) -> Html {
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
                        // Next level
                    } else {
                        new_code.clear();
                    }
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

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            { show_level(create_level(3, "  . .....012".to_string())) }
        </main>
    }
}
