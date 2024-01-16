use leptos::{html::Ul, *};
use leptos_icons::Icon;

#[component]
pub fn Menu(
    class: Signal<String>,
    children: Children,
    #[prop(optional)] ref_: NodeRef<Ul>,
) -> impl IntoView {
    view! {
        <ul ref_=ref_ class=move || format!("rounded-sm p-1 z-50 {}", class())>
            {children()}
        </ul>
    }
}

#[component]
pub fn MenuItem(
    #[prop(optional)] class: String,
    text: Signal<String>,
    #[prop(optional)] icon: Option<Signal<icondata::Icon>>,
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
                            <Icon width="24px" height="24px" icon/>
                        </span>
                    }
                        .into_view()
                }
                None => Fragment::new(vec![]).into_view(),
            }}
            {text}
            {match children {
                Some(child) => child().into_view(),
                None => Fragment::new(vec![]).into_view(),
            }}

        </li>
    }
}
