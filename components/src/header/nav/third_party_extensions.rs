use crate::header::{nav::button::HeaderMenuButton, HeaderStateSignal};
use crate::modal::Modal;
use crate::modal::{ModalOpen, ModalOpenSignal};
use crate::svg::SVGIcon;
use icondata::FaPuzzlePieceSolid;
use leptos::*;
use leptos_fluent::move_tr;
use simple_icons_macros::get_simple_icons_3rd_party_extensions;
use types::ThirdPartyExtension;

static THIRD_PARTY_EXTENSIONS: &[ThirdPartyExtension] =
    get_simple_icons_3rd_party_extensions!();

#[component]
fn ThirdPartyExtensionsTableRow(
    extension: &'static ThirdPartyExtension,
) -> impl IntoView {
    view! {
        <tr>
            <td>
                <a href=extension.url target="_blank">
                    <SVGIcon fill="currentColor" path=extension.icon_slug/>
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

/// Third party extensions table
#[component]
fn ThirdPartyExtensionsTable() -> impl IntoView {
    view! {
        <table class="third-party-extensions">
            <tbody>
                <For
                    each=move || THIRD_PARTY_EXTENSIONS.iter()
                    key=move |extension| extension.name
                    children=move |extension| {
                        view! { <ThirdPartyExtensionsTableRow extension=extension/> }
                    }
                />

            </tbody>
        </table>
    }
}

/// Third party extensions button
#[component]
fn ThirdPartyExtensionsButton() -> impl IntoView {
    let header_state = expect_context::<HeaderStateSignal>().0;
    let modal_open = expect_context::<ModalOpenSignal>();

    view! {
        <HeaderMenuButton
            title=move_tr!("third-party-extensions")
            class=Signal::derive(move || {
                if header_state().menu_open {
                    "block".to_string()
                } else {
                    "hidden lg:block".to_string()
                }
            })

            on:click=move |_| modal_open.set_extensions()
            icon=FaPuzzlePieceSolid
        />
    }
}

/// Third party extensions
#[component]
pub fn ThirdPartyExtensions() -> impl IntoView {
    let modal_open = expect_context::<ModalOpenSignal>();

    view! {
        <ThirdPartyExtensionsButton/>
        <Modal
            title=move_tr!("third-party-extensions")
            is_open=Signal::derive(move || modal_open.0() == Some(ModalOpen::Extensions))
            on_close=Signal::derive(move || modal_open.set_none())
            on_close_focus_search_bar=true
        >
            <ThirdPartyExtensionsTable/>
        </Modal>
    }
}
