use crate::Ids;
use leptos::*;
use std::fmt;

/// SVG definitions
///
/// The definitions of the SVG paths used in the icons.
/// They are defined here to optimize the size of the site.
#[derive(PartialEq)]
pub enum SVGDef {
    Null,
    /// Used in view detail buttons
    View,
    /// Download icons
    Download,
    DownloadThin,
    /// Controls icon
    Controls,
    /// Cross icon
    Cross,
    /// Upload icon
    Upload,
    /// Save icon
    Save,
    /// Grid icon
    Grid,
    /// Eye inside a box icon
    EyeBox,
    /// Copy icon
    Copy,
    /// Circle arrow icons
    CircleArrowUp,
    CircleArrowDown,
    /// Warning icon
    Warning,
}

impl SVGDef {
    pub fn id(&self) -> &'static str {
        match self {
            Self::Null => "null",
            Self::View => Ids::ViewSVGPath.as_str(),
            Self::Download => Ids::DownloadSVGPath.as_str(),
            Self::DownloadThin => Ids::DownloadThinSVGPath.as_str(),
            Self::Controls => Ids::ControlsSVGPath.as_str(),
            Self::Cross => Ids::CrossSVGPath.as_str(),
            Self::Upload => Ids::UploadSVGPath.as_str(),
            Self::Save => Ids::SaveSVGPath.as_str(),
            Self::Grid => Ids::GridSVGPath.as_str(),
            Self::EyeBox => Ids::EyeBoxSVGPath.as_str(),
            Self::Copy => Ids::CopySVGPath.as_str(),
            Self::CircleArrowUp => Ids::CircleArrowUpSVGPath.as_str(),
            Self::CircleArrowDown => Ids::CircleArrowDownSVGPath.as_str(),
            Self::Warning => Ids::WarningSVGPath.as_str(),
        }
    }

    pub fn d(&self) -> &'static str {
        match self {
            Self::Null => "",
            Self::View => "m23.136 20.694-4.41-4.413a1.93 1.93 0 0 0-1.186-.551 9.632 9.632 0 0 0 2.13-6.044C19.67 4.344 15.325 0 9.983 0 4.642 0 .297 4.344.297 9.686c0 5.34 4.344 9.685 9.685 9.685 2.016 0 3.89-.62 5.44-1.677.01.48.195.957.563 1.325l4.413 4.413c.377.38.874.568 1.369.568s.992-.189 1.369-.568a1.935 1.935 0 0 0 0-2.738zm-13.154-4.55a6.465 6.465 0 0 1-6.458-6.458 6.465 6.465 0 0 1 6.458-6.458 6.465 6.465 0 0 1 6.458 6.458 6.465 6.465 0 0 1-6.458 6.458z",
            Self::Download => "M5,20H19V18H5M19,9H15V3H9V9H5L12,16L19,9",
            Self::DownloadThin => "M11.2 0a.8.8 0 0 0-.8.8v11.4L7.26 9.44a.803.803 0 0 0-1.13.074l-1.05 1.2a.8.8 0 0 0 .073 1.13l6.33 5.54a.795.795 0 0 0 1.05 0l6.32-5.54a.8.8 0 0 0 .074-1.13l-1.05-1.2a.804.804 0 0 0-1.13-.074l-3.14 2.76V.8a.8.8 0 0 0-.8-.8zm-8 20.8a.8.8 0 0 0-.8.8v1.6a.8.8 0 0 0 .8.8h17.6a.8.8 0 0 0 .8-.8v-1.6a.8.8 0 0 0-.8-.8z",
            Self::Controls => "M3.429 0A3.442 3.442 0 0 0 0 3.429a3.439 3.439 0 0 0 3.429 3.428c1.585 0 2.931-1.097 3.317-2.571h16.397A.857.857 0 0 0 24 3.429a.857.857 0 0 0-.857-.858H6.749A3.447 3.447 0 0 0 3.43 0Zm0 1.714c.956 0 1.714.76 1.714 1.715a1.7 1.7 0 0 1-1.714 1.714 1.699 1.699 0 0 1-1.715-1.714c0-.955.756-1.715 1.715-1.715zM12 8.571a3.446 3.446 0 0 0-3.319 2.572H.857A.857.857 0 0 0 0 12a.857.857 0 0 0 .857.857h7.828A3.446 3.446 0 0 0 12 15.43c1.586 0 2.931-1.098 3.317-2.572h7.826A.857.857 0 0 0 24 12a.857.857 0 0 0-.857-.857H15.32A3.447 3.447 0 0 0 12 8.57Zm0 1.715c.957 0 1.714.76 1.714 1.714A1.7 1.7 0 0 1 12 13.714 1.699 1.699 0 0 1 10.286 12c0-.955.756-1.714 1.714-1.714zm8.571 6.857a3.446 3.446 0 0 0-3.318 2.571H.857a.857.857 0 0 0-.857.857.857.857 0 0 0 .857.858h16.399A3.446 3.446 0 0 0 20.571 24 3.44 3.44 0 0 0 24 20.571a3.443 3.443 0 0 0-3.429-3.428zm0 1.714c.957 0 1.715.76 1.715 1.714a1.7 1.7 0 0 1-1.715 1.715 1.699 1.699 0 0 1-1.714-1.715c0-.954.756-1.714 1.714-1.714z",
            Self::Cross => "M12 10.586l5.657-5.657a1 1 0 1 1 1.414 1.414L13.414 12l5.657 5.657a1 1 0 0 1-1.414 1.414L12 13.414l-5.657 5.657a1 1 0 0 1-1.414-1.414L10.586 12 4.93 6.343a1 1 0 0 1 1.414-1.414L12 10.586z",
            Self::Upload => "M9,16V10H5L12,3L19,10H15V16H9M5,20V18H19V20H5",
            Self::Save => "M15,9H5V5H15M12,19A3,3 0 0,1 9,16A3,3 0 0,1 12,13A3,3 0 0,1 15,16A3,3 0 0,1 12,19M17,3H5C3.89,3 3,3.9 3,5V19A2,2 0 0,0 5,21H19A2,2 0 0,0 21,19V7L17,3Z",
            Self::Grid => "M14.67,5v6.5H9.33V5H14.67z M15.67,11.5H21V5h-5.33V11.5z M14.67,19v-6.5H9.33V19H14.67z M15.67,12.5V19H21v-6.5H15.67z M8.33,12.5H3V19h5.33V12.5z M8.33,11.5V5H3v6.5H8.33z",
            Self::EyeBox => "M19,3H5C3.89,3,3,3.9,3,5v14c0,1.1,0.89,2,2,2h14c1.1,0,2-0.9,2-2V5C21,3.9,20.11,3,19,3z M19,19H5V7h14V19z M13.5,13 c0,0.83-0.67,1.5-1.5,1.5s-1.5-0.67-1.5-1.5c0-0.83,0.67-1.5,1.5-1.5S13.5,12.17,13.5,13z M12,9c-2.73,0-5.06,1.66-6,4 c0.94,2.34,3.27,4,6,4s5.06-1.66,6-4C17.06,10.66,14.73,9,12,9z M12,15.5c-1.38,0-2.5-1.12-2.5-2.5c0-1.38,1.12-2.5,2.5-2.5 c1.38,0,2.5,1.12,2.5,2.5C14.5,14.38,13.38,15.5,12,15.5z",
            Self::Copy => "M18,2H9C7.9,2,7,2.9,7,4v12c0,1.1,0.9,2,2,2h9c1.1,0,2-0.9,2-2V4C20,2.9,19.1,2,18,2z M18,16H9V4h9V16z M3,15v-2h2v2H3z M3,9.5h2v2H3V9.5z M10,20h2v2h-2V20z M3,18.5v-2h2v2H3z M5,22c-1.1,0-2-0.9-2-2h2V22z M8.5,22h-2v-2h2V22z M13.5,22L13.5,22l0-2h2 v0C15.5,21.1,14.6,22,13.5,22z M5,6L5,6l0,2H3v0C3,6.9,3.9,6,5,6z",
            Self::CircleArrowUp => "M12 20c-4.41 0-8-3.59-8-8s3.59-8 8-8s8 3.59 8 8s-3.59 8-8 8m0 2c5.52 0 10-4.48 10-10S17.52 2 12 2S2 6.48 2 12s4.48 10 10 10zm-1-10v3c0 .55.45 1 1 1s1-.45 1-1v-3h1.79c.45 0 .67-.54.35-.85l-2.79-2.79c-.2-.2-.51-.2-.71 0l-2.79 2.79a.5.5 0 0 0 .36.85H11z",
            Self::CircleArrowDown => "M12 4c4.41 0 8 3.59 8 8s-3.59 8-8 8s-8-3.59-8-8s3.59-8 8-8m0-2C6.48 2 2 6.48 2 12s4.48 10 10 10s10-4.48 10-10S17.52 2 12 2zm1 10V9c0-.55-.45-1-1-1s-1 .45-1 1v3H9.21c-.45 0-.67.54-.35.85l2.79 2.79c.2.2.51.2.71 0l2.79-2.79a.5.5 0 0 0-.35-.85H13z",
            Self::Warning => "M1 21h22L12 2 1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z",
        }
    }
}

impl fmt::Display for SVGDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.d())
    }
}

impl Default for &SVGDef {
    fn default() -> Self {
        &SVGDef::Null
    }
}

impl From<&SVGDef> for &'static str {
    fn from(svg_def: &SVGDef) -> Self {
        svg_def.d()
    }
}

#[component]
pub fn SVGDefsDefinition() -> impl IntoView {
    view! {
        <svg class="hidden" aria-hidden=true>
            <defs>
                <path id=SVGDef::View.id() d=SVGDef::View.d()></path>
                <path id=SVGDef::Download.id() d=SVGDef::Download.d()></path>
                <path id=SVGDef::DownloadThin.id() d=SVGDef::DownloadThin.d()></path>
                <path id=SVGDef::Controls.id() d=SVGDef::Controls.d()></path>
                <path id=SVGDef::Cross.id() d=SVGDef::Cross.d()></path>
                <path id=SVGDef::Upload.id() d=SVGDef::Upload.d()></path>
                <path id=SVGDef::Save.id() d=SVGDef::Save.d()></path>
                <path id=SVGDef::Grid.id() d=SVGDef::Grid.d()></path>
                <path id=SVGDef::EyeBox.id() d=SVGDef::EyeBox.d()></path>
                <path id=SVGDef::Copy.id() d=SVGDef::Copy.d()></path>
                <path id=SVGDef::CircleArrowUp.id() d=SVGDef::CircleArrowUp.d()></path>
                <path id=SVGDef::CircleArrowDown.id() d=SVGDef::CircleArrowDown.d()></path>
                <path id=SVGDef::Warning.id() d=SVGDef::Warning.d()></path>
            </defs>
        </svg>
    }
}

#[component]
pub fn SVGIcon<P>(
    path: P,
    #[prop(optional, into)] aria_label: Option<MaybeSignal<String>>,
    #[prop(optional)] class: &'static str,
    #[prop(optional)] fill: &'static str,
    #[prop(optional, into, default = MaybeSignal::Static("24"))]
    width: MaybeSignal<&'static str>,
    #[prop(optional, into, default = MaybeSignal::Static("24"))]
    height: MaybeSignal<&'static str>,
    #[prop(optional, into, default = MaybeSignal::Static("".into()))] view_box: MaybeSignal<
        String,
    >,
    #[prop(optional, default = "img")] role: &'static str,
    #[prop(optional, default = true)] aria_hidden: bool,
) -> impl IntoView
where
    P: Into<&'static str>,
{
    view! {
        <svg
            class=class
            role=role
            aria-hidden=if aria_hidden { "true" } else { "false" }
            width=width
            height=height
            aria-label=move || match &aria_label {
                Some(aria_label) => aria_label(),
                None => "".to_string(),
            }

            viewBox=move || match view_box().as_str() {
                "" => "0 0 24 24".into(),
                bbox => bbox.to_string(),
            }
        >

            <path d=path.into() fill=fill></path>
        </svg>
    }
}

/// Build a SVG string with the 24px24 viewBox from a path and
/// an optional `fill` attribute
pub fn svg_with_path_opt_fill(path: &str, fill: Option<&str>) -> String {
    format!(
        concat!(
            "<svg role=\"img\" viewBox=\"0 0 24 24\"",
            " xmlns=\"http://www.w3.org/2000/svg\"><path{} d=\"{}\"/></svg>",
        ),
        match fill {
            Some(fill) => format!(" fill=\"#{}\"", fill),
            None => "".to_string(),
        },
        path,
    )
}

/// Build a SVG string with the 24px24 viewBox from a path, a title and
/// an optional `fill` attribute
pub fn svg_with_title_path_opt_fill(
    title: &str,
    path: &str,
    fill: Option<&str>,
) -> String {
    format!(
        concat!(
            "<svg role=\"img\" viewBox=\"0 0 24 24\"",
            " xmlns=\"http://www.w3.org/2000/svg\">",
            "<title>{}</title><path{} d=\"{}\"/></svg>",
        ),
        // TODO: Escape unicode characters like simple-icons does
        title
            .replace('&', "&amp;")
            .replace('\'', "&apos;")
            .replace('\"', "&quot;")
            .replace('<', "&lt;")
            .replace('>', "&gt;"),
        match fill {
            Some(fill) => format!(" fill=\"#{}\"", fill),
            None => "".to_string(),
        },
        path,
    )
}
