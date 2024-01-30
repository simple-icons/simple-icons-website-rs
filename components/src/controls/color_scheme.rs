use crate::controls::button::ControlButtonSVGPath;
use crate::storage::LocalStorage;
use crate::Url;
use i18n::move_tr;
use leptos::*;
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
            set_color_scheme_on_localstorage(&color_scheme);
            color_scheme
        }
        None => match color_scheme_from_localstorage() {
            Some(color_scheme) => color_scheme,
            None => ColorMode::Auto,
        },
    }
}

fn color_scheme_from_localstorage() -> Option<ColorMode> {
    match LocalStorage::get(LocalStorage::Keys::ColorScheme) {
        None => None,
        Some(value) => match value.as_ref() {
            "light" => Some(ColorMode::Light),
            "dark" => Some(ColorMode::Dark),
            "system" | "auto" => Some(ColorMode::Auto),
            _ => None,
        },
    }
}

fn set_color_scheme_on_localstorage(color_scheme: &ColorMode) {
    LocalStorage::set(
        LocalStorage::Keys::ColorScheme,
        &color_scheme.to_string(),
    );
}

#[component]
pub fn ColorSchemeControl() -> impl IntoView {
    let (color_scheme, set_color_scheme) =
        expect_context::<(Signal<ColorMode>, WriteSignal<ColorMode>)>();

    view! {
        <div class="control">
            <label>{move_tr!("theme")}</label>
            <div>
                <ControlButtonSVGPath
                    title=move_tr!("light-color-scheme")
                    svg_path="M12 18c-3.309 0-6-2.691-6-6s2.691-6 6-6 6 2.691 6 6-2.691 6-6 6zm0-10c-2.206 0-4 1.794-4 4s1.794 4 4 4 4-1.794 4-4-1.794-4-4-4zm0-4a1 1 0 0 1-1-1V1a1 1 0 0 1 2 0v2a1 1 0 0 1-1 1zm0 20a1 1 0 0 1-1-1v-2a1 1 0 1 1 2 0v2a1 1 0 0 1-1 1zM5.64 6.64a.997.997 0 0 1-.707-.293l-1.42-1.42a.999.999 0 1 1 1.414-1.414l1.42 1.42A.999.999 0 0 1 5.64 6.64zm14.139 14.139a.997.997 0 0 1-.707-.293l-1.42-1.42a.999.999 0 1 1 1.414-1.414l1.42 1.42a.999.999 0 0 1-.707 1.707zM3 13H1a1 1 0 1 1 0-2h2a1 1 0 0 1 0 2zm20 0h-2a1 1 0 1 1 0-2h2a1 1 0 1 1 0 2zM4.22 20.779a.999.999 0 0 1-.707-1.707l1.42-1.42a.999.999 0 1 1 1.414 1.414l-1.42 1.42a.993.993 0 0 1-.707.293zM18.359 6.64a.999.999 0 0 1-.707-1.707l1.42-1.42a.999.999 0 1 1 1.414 1.414l-1.42 1.42a.997.997 0 0 1-.707.293z"
                    active=Signal::derive(move || color_scheme() == ColorMode::Light)
                    on:click=move |_| set_color_scheme(ColorMode::Light)
                />

                <ControlButtonSVGPath
                    title=move_tr!("dark-color-scheme")
                    svg_path="M12.048 21.963c-.308 0-.618-.015-.93-.043-2.66-.246-5.064-1.513-6.771-3.567s-2.512-4.651-2.266-7.311a10.004 10.004 0 0 1 9.038-9.038 1 1 0 0 1 .896 1.589 6.008 6.008 0 0 0 1.258 8.392c2.078 1.536 5.055 1.536 7.133 0a1 1 0 0 1 1.591.896 9.951 9.951 0 0 1-9.949 9.082zM9.315 4.438a8.006 8.006 0 0 0-5.244 6.787 7.954 7.954 0 0 0 1.813 5.849 7.95 7.95 0 0 0 5.417 2.854 7.95 7.95 0 0 0 8.266-5.243 8.01 8.01 0 0 1-2.729.476 7.946 7.946 0 0 1-4.755-1.565C9.174 11.443 8.145 7.68 9.315 4.438z"
                    active=Signal::derive(move || color_scheme() == ColorMode::Dark)
                    on:click=move |_| set_color_scheme(ColorMode::Dark)
                />

                <ControlButtonSVGPath
                    title=move_tr!("system-color-scheme")
                    svg_path="M12 4a1 1 0 0 1-1-1V1a1 1 0 0 1 2 0v2a1 1 0 0 1-1 1zM4.933 6.348a.997.997 0 0 0 1.414 0 .999.999 0 0 0 0-1.414l-1.42-1.42a.999.999 0 1 0-1.414 1.414l1.42 1.42zM1 13h2a1 1 0 1 0 0-2H1a1 1 0 0 0 0 2zm19.486-8.072 3.221-3.221A.999.999 0 1 0 22.293.293l-3.221 3.221-1.42 1.42-2.19 2.19A5.955 5.955 0 0 0 12 6c-3.309 0-6 2.691-6 6 0 1.258.406 2.453 1.124 3.462l-2.105 2.105c-.026.021-.058.03-.083.055s-.033.056-.055.082l-1.368 1.368-.001.002-3.219 3.219a.999.999 0 1 0 1.414 1.414l3.987-3.987a10.03 10.03 0 0 0 6.332 2.262c5.103-.001 9.473-3.902 9.951-9.081a1 1 0 0 0-1.591-.896 5.96 5.96 0 0 1-7.037.06l5.717-5.716 1.42-1.421zm-.945 9.78c-1.21 3.337-4.564 5.587-8.257 5.238a8.019 8.019 0 0 1-4.165-1.651l4.802-4.802c.05.038.093.082.144.12a7.955 7.955 0 0 0 7.476 1.095zm-10.979-.684A3.968 3.968 0 0 1 8 12c0-2.206 1.794-4 4-4a3.98 3.98 0 0 1 2.025.561l-5.463 5.463z"
                    active=Signal::derive(move || color_scheme() == ColorMode::Auto)
                    on:click=move |_| set_color_scheme(ColorMode::Auto)
                />

            </div>
        </div>
    }
}
