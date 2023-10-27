use leptos::*;

/// A simple spinner with tree animated dots
#[component]
pub(crate) fn TripleDotsSpinner<H>(
    /// Radius of the circles in pixels (5 as default)
    #[prop(optional)]
    radius: Option<u32>,
    /// Spacing between dots in pixels (3 as default)
    #[prop(optional)]
    space_x: Option<u32>,
    /// Duration of the animation loop in milliseconds (1000ms as default)
    #[prop(optional)]
    duration: Option<u32>,
    /// Height of the spinner in pixels (36 as default)
    #[prop(optional)]
    height: Option<u32>,
    /// Number of frames with dots not visible (3 as default)
    #[prop(optional)]
    hidden_frames: Option<u32>,
    /// Indicates if the spinner is currently hidden
    hidden: H,
) -> impl IntoView
where
    H: Fn() -> bool + 'static,
{
    let r = radius.unwrap_or(5);
    let dur = format!("{}ms", duration.unwrap_or(1000));
    // Spacing between dots
    let space_x = space_x.unwrap_or(3);
    let height = height.unwrap_or(36);
    let width = r * 6 + space_x * 2;
    let cy = height / 2;
    let hidden_frames_values = "0;".repeat(hidden_frames.unwrap_or(3) as usize);

    view! {
        <Show when=move || !hidden()>
            <svg
                width=format!("{}px", width)
                height=format!("{}px", height)
                viewBox=format!("0 0 {} {}", width, height)
            >
                <circle cx=r cy=cy r=r>
                    <animate
                        attributeName="r"
                        values=format!("0;{};0;0;{}", r, hidden_frames_values)
                        dur=&dur
                        repeatCount="indefinite"
                    ></animate>
                </circle>
                <circle cx=r * 3 + space_x cy=cy r=r>
                    <animate
                        attributeName="r"
                        values=format!("0;0;{};0;{}", r, hidden_frames_values)
                        dur=&dur
                        repeatCount="indefinite"
                    ></animate>
                </circle>
                <circle cx=r * 5 + space_x * 2 cy=cy r=r>
                    <animate
                        attributeName="r"
                        values=format!("0;0;0;{};{}", r, hidden_frames_values)
                        dur=&dur
                        repeatCount="indefinite"
                    ></animate>
                </circle>
            </svg>
        </Show>
    }
}
