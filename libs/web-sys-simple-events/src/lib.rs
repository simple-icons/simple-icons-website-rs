/// Dispatch an event on an input
pub fn dispatch_event_on_input(
    input: &web_sys::HtmlInputElement,
    event_name: &str,
) {
    let event_opts = web_sys::EventInit::new();
    event_opts.set_bubbles(true);
    let event =
        web_sys::Event::new_with_event_init_dict(event_name, &event_opts)
            .unwrap();
    input.dispatch_event(&event).unwrap();
}

/// Dispatch an input event on an input
pub fn dispatch_input_event_on_input(input: &web_sys::HtmlInputElement) {
    dispatch_event_on_input(input, "input");
}
