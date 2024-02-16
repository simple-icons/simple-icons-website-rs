use crate::controls::button::ControlButtonIcon;
use crate::storage::LocalStorage;
use crate::Url;
use icondata::{ChMoon, ChSun, TbSunMoon};
use leptos::*;
use leptos_fluent::move_tr;
use leptos_use::ColorMode;

pub fn initial_color_scheme() -> ColorMode {
    match Url::params::get(&Url::params::Names::ColorScheme).and_then(|value| {
        match value.as_ref() {
            "light" => Some(ColorMode::Light),
            "dark" => Some(ColorMode::Dark),
            "system" | "auto" => Some(ColorMode::Auto),
            _ => None,
        }
    }) {
        Some(color_scheme) => {
            LocalStorage::set(
                LocalStorage::Keys::ColorScheme,
                &color_scheme.to_string(),
            );
            color_scheme
        }
        None => ColorMode::Auto,
    }
}

#[component]
pub fn ColorSchemeControl() -> impl IntoView {
    let (color_scheme, set_color_scheme) =
        expect_context::<(Signal<ColorMode>, WriteSignal<ColorMode>)>();

    view! {
        <div class="control">
            <label>{move_tr!("theme")}</label>
            <div>
                <ControlButtonIcon
                    title=move_tr!("light-color-scheme")
                    icon=ChSun
                    active=Signal::derive(move || color_scheme() == ColorMode::Light)
                    on:click=move |_| set_color_scheme(ColorMode::Light)
                />

                <ControlButtonIcon
                    title=move_tr!("dark-color-scheme")
                    icon=ChMoon
                    active=Signal::derive(move || color_scheme() == ColorMode::Dark)
                    on:click=move |_| set_color_scheme(ColorMode::Dark)
                />

                <ControlButtonIcon
                    title=move_tr!("system-color-scheme")
                    icon=TbSunMoon
                    active=Signal::derive(move || color_scheme() == ColorMode::Auto)
                    on:click=move |_| set_color_scheme(ColorMode::Auto)
                />

            </div>
        </div>
    }
}
