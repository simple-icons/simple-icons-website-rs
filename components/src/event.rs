/// Dispatch an input event on an input
pub fn dispatch_input_event_on_input(input: &web_sys::HtmlInputElement) {
    let event_opts = web_sys::EventInit::new();
    event_opts.set_bubbles(true);
    let event =
        web_sys::Event::new_with_event_init_dict("input", &event_opts).unwrap();
    input.dispatch_event(&event).unwrap();
}
