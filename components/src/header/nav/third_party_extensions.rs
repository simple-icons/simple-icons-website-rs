use crate::header::{nav::button::HeaderMenuButton, HeaderStateSignal};
use crate::modal::Modal;
use crate::modal::{ModalOpen, ModalOpenSignal};
use crate::svg::SVGIcon;
use leptos::*;
use leptos_fluent::i18n;
use simple_icons_macros::get_simple_icons_3rd_party_extensions;
use types::ThirdPartyExtension;

static THIRD_PARTY_EXTENSIONS: &[ThirdPartyExtension] =
    get_simple_icons_3rd_party_extensions!();

static THIRD_PARTY_EXTENSIONS_ICON_SVG_PATH: &str = concat!(
    "M16.513 23.996a.9.9 0 0 0 .885-.907v-4.972c.303-2.68",
    " 1.42-1.884 2.734-1.055 3.178 2.003 5.29-3.266 2.72-4.891-2.015",
    "-1.276-2.888.917-4.364.69-.57-.088-.967-.72-1.092-1.68V7.59c0",
    "-.5-.398-.907-.885-.907h-4.064c-3.355-.436-.377-2.339-.377-4.11C12.072",
    " 1.152 10.816 0 9.267 0 7.721 0 6.301 1.152 6.301 2.573c0 1.67 3.082",
    " 3.674-.32 4.11H.884A.898.898 0 0 0 0 7.59v3.583c.26 1.528 1.268 1.882",
    " 2.559.874.435-.341 1.17-.738 1.7-.738 1.385 0 2.51 1.285 2.51 2.871s",
    "-1.123 3.221-2.51 3.221c-.493 0-.954-.164-1.345-.45 0 .121-2.422-2.232",
    "-2.914.648v5.494c0 .5.398.907.885.907 2.728 0 5.453 0 8.18-.002.107",
    "-.525-.243-1.125-.571-1.646-2.582-4.1 7.463-4.508 4.88.128-.126.228",
    "-.253.45-.35.666-.124.27-.206.599-.188.852z"
);

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
    let i18n = i18n();

    view! {
        <HeaderMenuButton
            title=Signal::derive(move || i18n.tr("third-party-extensions"))
            class=Signal::derive(move || {
                if header_state().menu_open {
                    "block".to_string()
                } else {
                    "hidden lg:block".to_string()
                }
            })

            on:click=move |_| modal_open.set_extensions()
            svg_path=THIRD_PARTY_EXTENSIONS_ICON_SVG_PATH
        />
    }
}

/// Third party extensions
#[component]
pub fn ThirdPartyExtensions() -> impl IntoView {
    let modal_open = expect_context::<ModalOpenSignal>();
    let i18n = i18n();

    view! {
        <ThirdPartyExtensionsButton/>
        <Modal
            title=Signal::derive(move || i18n.tr("third-party-extensions"))
            is_open=Signal::derive(move || modal_open.0() == Some(ModalOpen::Extensions))
            on_close=Signal::derive(move || modal_open.set_none())
            on_close_focus_search_bar=true
        >
            <ThirdPartyExtensionsTable/>
        </Modal>
    }
}
