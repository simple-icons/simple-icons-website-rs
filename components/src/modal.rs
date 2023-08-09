use crate::copy::copy_inner_text_on_click;
use crate::svg_defs::SVGDefs;
use crate::Url;
use core::fmt;
use i18n::move_tr;
use leptos::{ev::MouseEvent, *};
use web_sys;

pub trait TitleFn = Fn() -> String + 'static;
pub trait IsOpenFn = Fn() -> bool + 'static;
pub trait OnCloseFn = Fn(MouseEvent) + 'static + Copy;

#[component]
fn ModalHeader<T, C>(
    /// Title of the modal
    title: T,
    /// Indicates whether the title is copyable
    #[prop(optional)]
    title_is_copyable: bool,
    /// Function executed when the close button is clicked
    /// or the user clicks outside the modal
    on_close: C,
) -> impl IntoView
where
    T: TitleFn,
    C: OnCloseFn,
{
    view! {
        <div>
            <h2
                class:copyable=move || title_is_copyable
                on:click=move |ev: MouseEvent| {
                    if title_is_copyable {
                        copy_inner_text_on_click(ev);
                    }
                }
            >
                {title}
            </h2>
            <button type="button" title=move_tr!("close") on:click=on_close>
                <svg role="img" viewBox="0 0 24 24">
                    <use_ href=format!("#{}", SVGDefs::CrossPath.id())></use_>
                </svg>
            </button>
        </div>
    }
}

#[component]
fn ModalBody(children: Children) -> impl IntoView {
    view! { <div>{children()}</div> }
}

#[component]
fn ModalShadow<O, C>(
    children: Children,
    /// Indicates whether the modal is open or not
    is_open: O,
    /// Function executed when the user clicks in the shadow of the modal
    on_close: C,
) -> impl IntoView
where
    O: IsOpenFn,
    C: OnCloseFn,
{
    let class: &'static str = "modal-shadow";

    view! {
        <div
            class=class
            class:hidden=move || !is_open()
            on:click=move |ev: MouseEvent| {
                let target = event_target::<web_sys::HtmlElement>(&ev);
                if target.class_list().contains(class) {
                    on_close(ev);
                }
            }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn Modal<T, O, C>(
    children: Children,
    /// Title of the modal
    title: T,
    /// Indicates whether the title is copyable
    #[prop(optional)]
    title_is_copyable: bool,
    /// Indicates whether the modal is open or not
    is_open: O,
    /// Function executed when the close button is clicked
    /// or the user clicks outside the modal
    on_close: C,
) -> impl IntoView
where
    O: IsOpenFn,
    T: TitleFn,
    C: OnCloseFn,
{
    view! {
        <ModalShadow is_open=is_open on_close=on_close>
            <div class="modal">
                <ModalHeader title=title title_is_copyable=title_is_copyable on_close=on_close/>
                <ModalBody>{children()}</ModalBody>
            </div>
        </ModalShadow>
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum ModalOpen {
    Extensions,
    Languages,
    Icon,
}

impl fmt::Display for ModalOpen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModalOpen::Extensions => write!(f, "extensions"),
            ModalOpen::Languages => write!(f, "languages"),
            ModalOpen::Icon => write!(f, "icon"),
        }
    }
}

impl TryFrom<&str> for ModalOpen {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "extensions" => Ok(ModalOpen::Extensions),
            "languages" => Ok(ModalOpen::Languages),
            "icon" => Ok(ModalOpen::Icon),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone)]
pub struct ModalOpenSignal(pub RwSignal<Option<ModalOpen>>);

impl ModalOpenSignal {
    pub fn set_none(&self) {
        self.0.update(|mo| *mo = None);
        Url::params::update(&Url::params::Names::Modal, "");
    }

    pub fn set_extensions(&self) {
        self.0.update(|mo| *mo = Some(ModalOpen::Extensions));
        Url::params::update(
            &Url::params::Names::Modal,
            &ModalOpen::Extensions.to_string(),
        );
    }

    pub fn set_languages(&self) {
        self.0.update(|mo| *mo = Some(ModalOpen::Languages));
        Url::params::update(
            &Url::params::Names::Modal,
            &ModalOpen::Languages.to_string(),
        );
    }

    pub fn set_icon(&self) {
        self.0.update(|mo| *mo = Some(ModalOpen::Icon));
        Url::params::update(
            &Url::params::Names::Modal,
            &ModalOpen::Icon.to_string(),
        );
    }
}

fn modal_open_from_url() -> Option<ModalOpen> {
    match Url::params::get(&Url::params::Names::Modal) {
        Some(modal) => {
            if let Ok(modal) = ModalOpen::try_from(modal.as_str()) {
                return Some(modal);
            }
            None
        }
        None => None,
    }
}

pub fn provide_modal_open_context() {
    provide_context(ModalOpenSignal(create_rw_signal(modal_open_from_url())));
}
