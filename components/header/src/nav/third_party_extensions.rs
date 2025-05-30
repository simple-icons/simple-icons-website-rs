use crate::{HeaderStateSignal, nav::button::HeaderMenuButton};
use icondata::FaPuzzlePieceSolid;
use leptos::{prelude::*, task::spawn_local};
use leptos_fluent::move_tr;
use simple_icons_macros::{
    get_simple_icons_3rd_party_extensions, get_simple_icons_3rd_party_libraries,
};
use simple_icons_website_modal::{Modal, ModalOpen, ModalOpenSignal};
use simple_icons_website_types::ThirdPartyExtension;
use web_sys_simple_fetch::fetch_text;

static THIRD_PARTY_EXTENSIONS: &[&ThirdPartyExtension] =
    get_simple_icons_3rd_party_extensions!();
static THIRD_PARTY_LIBRARIES: &[&ThirdPartyExtension] =
    get_simple_icons_3rd_party_libraries!();

#[component]
fn ThirdPartyExtensionsTableRow(
    extension: &'static ThirdPartyExtension,
) -> impl IntoView {
    let (icon_path, set_icon_path) = signal("".to_string());

    spawn_local(async move {
        if let Ok(content) = fetch_text(extension.icon_image_src).await {
            let path = content
                .split("<path d=\"")
                .nth(1)
                .unwrap_or(" \"")
                .split_once('"')
                .unwrap()
                .0;
            set_icon_path.set(path.to_string());
        }
    });

    view! {
        <tr>
            <td>
                <a href=extension.url target="_blank">
                    <svg width="24" height="24" role="img" aria-hidden="true" viewBox="0 0 24 24">
                        <path d=icon_path fill="currentColor"></path>
                    </svg>
                    <span>{extension.name}</span>
                </a>
            </td>
            <td>
                <a target="_blank" href=extension.author_url>
                    {extension.author_name}
                </a>
            </td>
        </tr>
    }
}

#[component]
fn ThirdPartyExtensionsOrLibrariesTable(
    items: &'static [&'static ThirdPartyExtension],
) -> impl IntoView {
    view! {
        <table class="third-party-extensions">
            <tbody>
                <For
                    each=move || items
                    key=move |extension| extension.name
                    children=move |extension| {
                        view! { <ThirdPartyExtensionsTableRow extension=extension /> }
                    }
                />

            </tbody>
        </table>
    }
}

#[component]
fn ThirdPartyExtensionsTableTitle(title: Signal<String>) -> impl IntoView {
    view! { <h3 class="ml-4 pb-1 font-bold">{title}</h3> }
}

/// Third party extensions button
#[component]
fn ThirdPartyExtensionsButton() -> impl IntoView {
    let header_state = expect_context::<HeaderStateSignal>().0;
    let modal_open = expect_context::<ModalOpenSignal>();

    view! {
        <HeaderMenuButton
            on:click=move |_| modal_open.set_extensions()
            icon=FaPuzzlePieceSolid
            attr:class=move || if header_state().menu_open { "block" } else { "hidden lg:block" }
            attr:title=move_tr!("third-party-extensions")
        />
    }
}

/// Third party extensions
#[component]
pub fn ThirdPartyExtensions() -> impl IntoView {
    let modal_open = expect_context::<ModalOpenSignal>();

    view! {
        <ThirdPartyExtensionsButton />
        <Modal
            title=move_tr!("third-party-extensions")
            is_open=Signal::derive(move || modal_open.is_open(ModalOpen::Extensions))
            on_close=Signal::derive(move || modal_open.set_none())
            on_close_focus_search_bar=true
        >
            <ThirdPartyExtensionsTableTitle title=move_tr!("extensions") />
            <ThirdPartyExtensionsOrLibrariesTable items=THIRD_PARTY_EXTENSIONS />
            <br />
            <ThirdPartyExtensionsTableTitle title=move_tr!("libraries") />
            <ThirdPartyExtensionsOrLibrariesTable items=THIRD_PARTY_LIBRARIES />
        </Modal>
    }
}
