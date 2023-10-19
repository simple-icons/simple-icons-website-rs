use leptos::*;

#[component]
pub fn Button<T>(
    svg_path: &'static str,
    title: T,
    #[prop(optional)] class: &'static str,
) -> impl IntoView
where
    T: Fn() -> String + 'static + Copy,
{
    view! {
        <button title=title class=class type="button">
            <svg aria-hidden="true" viewBox="0 0 24 24" width="24" height="24">
                <path d=svg_path></path>
            </svg>
            {title}
        </button>
    }
}
