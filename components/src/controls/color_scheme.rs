use crate::controls::button::ControlButtonIcon;
use icondata::{ChMoon, ChSun, TbSunMoon};
use leptos::prelude::*;
use leptos_fluent::move_tr;
use leptos_use::ColorMode;

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
