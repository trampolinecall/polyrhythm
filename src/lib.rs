use wasm_bindgen::{closure::Closure, prelude::wasm_bindgen, JsCast};
use web_sys::{HtmlCanvasElement, HtmlTextAreaElement};

mod drawing;
mod parse;
mod polyrhythm;
mod rhythm;
mod time;
mod units;

#[wasm_bindgen(start)]
pub async fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let window = web_sys::window().expect("global window does not exists");
    let document = window.document().expect("expecting a document on window");

    let codebox = document.get_element_by_id("code").expect("could not find code box").dyn_into::<HtmlTextAreaElement>().expect("code box should be a textarea");
    let canvas = document.get_element_by_id("canvas").expect("could not find canvas").dyn_into::<HtmlCanvasElement>().expect("canvas should be canvas");
    let errors = document.get_element_by_id("errors").expect("could not find errors box");

    let font = drawing::Font::load_bravura(&window).await;

    codebox
        .add_event_listener_with_callback(
            "input",
            Closure::<dyn Fn()>::new({
                let codebox = codebox.clone();
                move || {
                    let code = codebox.value();
                    let parsed = parse::parse(&code);
                    match parsed {
                        Ok(polyrhythm) => {
                            drawing::draw(&canvas, &font, &polyrhythm);
                            errors.replace_children_with_node(&web_sys::js_sys::Array::new());
                        }
                        Err(err) => {
                            errors.replace_children_with_node_1(&parse::parse_error_to_div(&document, err).into());
                        }
                    }
                }
            })
            .into_js_value()
            .dyn_ref()
            .expect("closure should be function"),
        )
        .expect("could not add event listener on code box input");
}
