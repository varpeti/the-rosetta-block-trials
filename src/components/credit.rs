use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct CreditProps {
    pub forward_button: Html,
}

#[function_component(Credit)]
pub fn show_credit(props: &CreditProps) -> Html {
    let props = props.clone();
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
                {props.forward_button}
            </tr>
        </table>
    }
}
