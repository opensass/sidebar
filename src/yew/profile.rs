use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ProfileProps {
    #[prop_or_default]
    pub user_name: &'static str,
    #[prop_or_default]
    pub designation: &'static str,
    #[prop_or_default]
    pub user_img: &'static str,
    #[prop_or_default]
    pub is_collapsed: bool,
    #[prop_or_default]
    pub on_logout: Callback<()>,

    #[prop_or(
        "display: flex; align-items: center; gap: 10px; margin: 1rem; padding: 1rem; background: #f0f0f0; border-radius: 8px;"
    )]
    pub style: &'static str,
    #[prop_or_default]
    pub class: &'static str,
}

#[function_component(Profile)]
pub fn profile(props: &ProfileProps) -> Html {
    if props.is_collapsed {
        html! {}
    } else {
        html! {
            <div
                class={props.class}
                style={props.style}
                aria-label="User Profile"
            >
                <img src={props.user_img} style="width: 40px; height: 40px; border-radius: 50%;" alt="User" />
                <div>
                    <strong>{ props.user_name }</strong>
                    <div style="font-size: 12px;">{ props.designation }</div>
                </div>
                <button onclick={props.on_logout.reform(|_| ())} style="margin-left:auto;">{ "⏻" }</button>
            </div>
        }
    }
}
