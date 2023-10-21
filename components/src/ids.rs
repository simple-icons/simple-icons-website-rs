//! Module to track the ids of components, to ensure that they are unique.
//!
//! It minifies identifiers in release mode.

pub enum Ids {
    SearchInput,
    CopyInput,
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
    GridSVGPath,
    EyeBoxSVGPath,
    CopySVGPath,
}

impl Ids {
    pub fn as_str(&self) -> &'static str {
        #[cfg(debug_assertions)]
        match self {
            Ids::IconDetailsModal => "icon-details-modal",
            Ids::SearchInput => "search-input",
            Ids::CopyInput => "copy-input",
            Ids::PreviewCopyButton => "preview-copy-button",
            Ids::PreviewSaveButton => "preview-save-button",
            Ids::PreviewDownloadSVGButton => "preview-download-svg-button",
            Ids::PreviewUploadSVGButton => "preview-upload-svg-button",

            Ids::ViewSVGPath => "view-path",
            Ids::DownloadSVGPath => "download-path",
            Ids::DownloadThinSVGPath => "download-thin-path",
            Ids::ControlsSVGPath => "controls-path",
            Ids::CrossSVGPath => "cross-path",
            Ids::UploadSVGPath => "upload-path",
            Ids::SaveSVGPath => "save-path",
            Ids::GridSVGPath => "grid-path",
            Ids::EyeBoxSVGPath => "eye-box-path",
            Ids::CopySVGPath => "copy-path",
        }
        #[cfg(not(debug_assertions))]
        match self {
            Ids::IconDetailsModal => "i",
            Ids::SearchInput => "f",
            Ids::CopyInput => "y",
            Ids::PreviewCopyButton => "b",
            Ids::PreviewSaveButton => "j",
            Ids::PreviewDownloadSVGButton => "k",
            Ids::PreviewUploadSVGButton => "l",

            Ids::ViewSVGPath => "v",
            Ids::DownloadSVGPath => "d",
            Ids::DownloadThinSVGPath => "h",
            Ids::ControlsSVGPath => "c",
            Ids::CrossSVGPath => "x",
            Ids::UploadSVGPath => "u",
            Ids::SaveSVGPath => "s",
            Ids::GridSVGPath => "g",
            Ids::EyeBoxSVGPath => "e",
            Ids::CopySVGPath => "p",
        }
    }
}
