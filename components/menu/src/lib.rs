use leptos::{html::Ul, prelude::*};
use leptos_icons::Icon;

#[component]
pub fn Menu(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
    #[prop(optional, into)] node_ref: NodeRef<Ul>,
) -> impl IntoView {
    view! {
        <ul node_ref=node_ref class=move || format!("rounded-sm p-1 z-50 {}", class())>
            {children()}
        </ul>
    }
}

#[component]
pub fn MenuItem(
    class: &'static str,
    #[prop(optional, into)] text: Signal<String>,
    #[prop(optional, into)] icon: Option<Signal<icondata::Icon>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <li class=format!(
            concat!(
                "flex flex-row gap-x-2 px-3 py-1.5 cursor-pointer rounded-sm",
                " whitespace-nowrap my-auto {}",
            ),
            class,
        )>
            {match icon {
                Some(icon) => {
                    view! {
                        <span class="min-w-[24px] min-h-[24px]">
                            <Icon width="24px" height="24px" icon />
                        </span>
                    }
                        .into_any()
                }
                #[allow(clippy::unit_arg, clippy::unused_unit)]
                None => view!().into_any(),
            }} {text}
            {match children {
                Some(child) => child().into_any(),
                #[allow(clippy::unit_arg, clippy::unused_unit)]
                None => view!().into_any(),
            }}
        </li>
    }
}
