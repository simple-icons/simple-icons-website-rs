//! Track the ids of components, to ensure that they are unique.
//!
//! Minify identifiers in release mode.

/*
pub enum IdsOld {
    SearchInput,
    IconDetailsModal,
    PreviewCopyButton,
    PreviewSaveButton,
    PreviewDownloadSVGButton,
    PreviewUploadSVGButton,

    ViewSVGPath,
    DownloadSVGPath,
    DownloadThinSVGPath,
    ControlsSVGPath,
    CrossSVGPath,
    UploadSVGPath,
    SaveSVGPath,
    CopySVGPath,
}

impl IdsOld {
    pub fn as_str(&self) -> &'static str {
        #[cfg(debug_assertions)]
        match self {
            IdsOld::IconDetailsModal => "icon-details-modal",
            IdsOld::SearchInput => "search-input",
            IdsOld::PreviewCopyButton => "preview-copy-button",
            IdsOld::PreviewSaveButton => "preview-save-button",
            IdsOld::PreviewDownloadSVGButton => "preview-download-svg-button",
            IdsOld::PreviewUploadSVGButton => "preview-upload-svg-button",

            IdsOld::ViewSVGPath => "view-path",
            IdsOld::DownloadSVGPath => "download-path",
            IdsOld::DownloadThinSVGPath => "download-thin-path",
            IdsOld::ControlsSVGPath => "controls-path",
            IdsOld::CrossSVGPath => "cross-path",
            IdsOld::UploadSVGPath => "upload-path",
            IdsOld::SaveSVGPath => "save-path",
            IdsOld::CopySVGPath => "copy-path",
        }
        #[cfg(not(debug_assertions))]
        match self {
            IdsOld::IconDetailsModal => "i",
            IdsOld::SearchInput => "f",
            IdsOld::PreviewCopyButton => "b",
            IdsOld::PreviewSaveButton => "j",
            IdsOld::PreviewDownloadSVGButton => "k",
            IdsOld::PreviewUploadSVGButton => "l",

            IdsOld::ViewSVGPath => "v",
            IdsOld::DownloadSVGPath => "d",
            IdsOld::DownloadThinSVGPath => "h",
            IdsOld::ControlsSVGPath => "c",
            IdsOld::CrossSVGPath => "x",
            IdsOld::UploadSVGPath => "u",
            IdsOld::SaveSVGPath => "s",
            IdsOld::CopySVGPath => "p",
        }
    }
}
*/

use leptos_unique_ids::leptos_unique_ids;

/// Some documentation
#[leptos_unique_ids(
    "icon-details-modal",
    "search-input",
    "preview-copy-button",
    "preview-save-button",
    "preview-download-svg-button",
    "preview-upload-svg-button",
    "view-svg-path",
    "download-svg-path",
    "download-thin-svg-path",
    "controls-svg-path",
    "cross-svg-path",
    "upload-svg-path",
    "save-svg-path",
    "copy-svg-path"
)]
pub enum Ids {}
