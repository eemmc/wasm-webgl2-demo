mod cube;
mod engine;
mod math;
mod utils;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGl2RenderingContext as WebGL;
use crate::engine::Engine;

fn request_animation_frame(
    closure: &Closure<dyn FnMut()>
) -> Result<i32, JsValue> {
    web_sys::window().unwrap()
        .request_animation_frame(
            closure.as_ref().unchecked_ref()
        )
}


#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(480);
    canvas.set_height(360);
    let gl = canvas.get_context("webgl2")?.unwrap()
        .dyn_into::<WebGL>()?;

    let engine = Rc::new(RefCell::new(
        Engine::create(gl),
    ));

    //
    {
        engine.borrow_mut().setup();
    }
    //
    {
        let callback = Rc::new(RefCell::new(None));
        //
        let engine = engine.clone();
        let callback_ = callback.clone();
        *callback.borrow_mut() = Some(Closure::wrap(Box::new(
            move || {
                engine.borrow_mut().update();
                request_animation_frame(
                    RefCell::borrow(&callback_).as_ref().unwrap()
                ).unwrap();
            }
        ) as Box<dyn FnMut()>));
        request_animation_frame(
            RefCell::borrow(&callback).as_ref().unwrap()
        )?;
    }

    Ok(())
}
