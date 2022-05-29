mod engine;
mod glm;
mod obj;
mod utils;



use std::cell::{Cell, RefCell};
use std::intrinsics::fabsf32;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, WebGl2RenderingContext as WebGl};
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
    canvas.set_width(360);
    canvas.set_height(480);
    let gl = canvas.get_context("webgl2")?.unwrap()
        .dyn_into::<WebGl>()?;

    //
    let pressed = Rc::new(Cell::new(false));
    let engine = Rc::new(RefCell::new(
        Engine::create(gl),
    ));
    //
    engine.borrow_mut().setup();
    //
    {
        let engine = engine.clone();
        let closure = Closure::wrap(Box::new(
            move |event: web_sys::MouseEvent| {
                let rect = event.target()
                    .dyn_into::<Element>().unwrap()
                    .get_bounding_client_rect();
                let x = event.client_x() - rect.x();
                let y = event.client_y() - rect.y();
                //
                pressed.set(true);
                engine.borrow_mut().input(
                    x as f32,
                    y as f32,
                    pressed.get(),
                );
            }
        ) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback(
            "mousedown", closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }
    //
    {
        let engine = engine.clone();
        let closure = Closure::wrap(Box::new(
            move |event: web_sys::MouseEvent| {
                let rect = event.target()
                    .dyn_into::<Element>().unwrap()
                    .get_bounding_client_rect();
                let x = (event.client_x() - rect.x()) as f32;
                let y = (event.client_y() - rect.y()) as f32;
                engine.borrow_mut().input(
                    x.max(0.0),
                    y.max(0.0),
                    pressed.get(),
                );
            }
        ) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback(
            "mousemove", closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }
    //
    {
        let engine = engine.clone();
        let closure = Closure::wrap(Box::new(
            move |event: web_sys::MouseEvent| {
                pressed.set(false);
                engine.borrow_mut().input(
                    0.0,
                    0.0,
                    pressed.get(),
                );
            }
        ) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback(
            "mouseup", closure.as_ref().unchecked_ref(),
        )?;
        closure.forget();
    }

    //
    {
        let engine = engine.clone();
        let callback = Rc::new(RefCell::new(None));
        let callback_ = callback.clone();
        *callback.borrow_mut() = Some(Closure::wrap(Box::new(
            move || {
                engine.borrow_mut().update();
                request_animation_frame(
                    callback_.borrow().as_ref().unwrap()
                ).unwrap();
            }
        ) as Box<dyn FnMut()>));
        request_animation_frame(
            callback.borrow().as_ref().unwrap()
        ).unwrap();
    }

    Ok(())
}
