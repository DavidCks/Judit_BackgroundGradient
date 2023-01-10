#[allow(non_snake_case)]
use yew::prelude::*;
mod dummy_ee;
use dummy_ee::Dummy;
use rusty_css::*;
use append_to_string::*;
use bevy_reflect::{ Reflect };
mod GradientButtonStyles;
use GradientButtonStyles::{ Circular, Conic, Linear, Solid, RepeatigLinear, RepeatingCircular };
mod sub_components;
use sub_components::BackgroundVisibilityToggle::Toggle;

#[derive(Reflect)]
struct GradientButtonBaseStyle {
    width: String,
    height: String,
    margin: String,
    padding: String,
    border: String,
    border_radius: String,
}

impl Style for GradientButtonBaseStyle {
    fn create() -> Self {
        append_to_string!(
            Self {
                width: "20px",
                height: "20px",
                margin: "4px",
                padding: "0px",
                border: "none",
                border_radius: "4px",
            }
        )
    }
}


#[function_component]
fn App() -> Html {
    let solid_style = format!("{}{}",Solid::RCSS::create().inline(), GradientButtonBaseStyle::create().inline());
    let linear_style = format!("{}{}",Linear::RCSS::create().inline(), GradientButtonBaseStyle::create().inline());
    let circular_style = format!("{}{}",Circular::RCSS::create().inline(), GradientButtonBaseStyle::create().inline());
    let conic_style = format!("{}{}",Conic::RCSS::create().inline(), GradientButtonBaseStyle::create().inline());
    let repeating_linear_style = format!("{}{}",RepeatigLinear::RCSS::create().inline(), GradientButtonBaseStyle::create().inline());
    let repeating_circular_style = format!("{}{}",RepeatingCircular::RCSS::create().inline(), GradientButtonBaseStyle::create().inline());

    html! {
        <div style="position: absolute; width: fit-content; writing-mode: horizontal-tb; direction: ltr; z-index: 9999;  transform: rotateZ(-0deg) rotateY(-0deg) rotateX(-0deg)">
        <Dummy/>
            <div style="display: grid; grid-template-columns: 1fr 1fr 1fr 1fr; gap: 0px 0px; align-items: center; position: initial; border-radius: 10px;">
                <button style={solid_style}></button>
                <button style={linear_style}></button>
                <button style={repeating_linear_style}></button>
                <Toggle/>
                <button style={circular_style}></button>
                <button style={conic_style}></button>
                <button style={repeating_circular_style}></button>
                <svg width="24px" height="24px" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#3f3f3f">
                    <path d="M19 7V5H5v2M12 5v14m0 0h-2m2 0h2" stroke="#3f3f3f" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                    </path>
                </svg>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}