use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlImageElement};
use web_sys::WebGlRenderingContext;
use web_sys::WebGl2RenderingContext;
use web_sys::{WebGlProgram, WebGlShader, WebGlTexture};


pub fn create_shader(
    gl: &WebGl2RenderingContext,
    type_: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl.create_shader(type_).ok_or(
        "Unable to create shader object".to_owned()
    )?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);
    let compiled = gl.get_shader_parameter(
        &shader, WebGl2RenderingContext::COMPILE_STATUS,
    );
    if !compiled.as_bool().unwrap_or(false) {
        let error = gl.get_shader_info_log(&shader).unwrap_or(
            "Unknown error creating shader.".to_owned()
        );
        gl.delete_shader(Some(&shader));
        Err(error)
    } else {
        Ok(shader)
    }
}

pub fn create_program(
    gl: &WebGl2RenderingContext,
    vs: &WebGlShader,
    fs: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = gl.create_program().ok_or(
        "Unable to create shader object".to_owned()
    )?;
    gl.attach_shader(&program, vs);
    gl.attach_shader(&program, fs);
    gl.link_program(&program);
    let linked = gl.get_program_parameter(
        &program, WebGl2RenderingContext::LINK_STATUS,
    );
    if !linked.as_bool().unwrap_or(false) {
        let error = gl.get_program_info_log(&program).unwrap_or(
            "Unknown error creating program object.".to_owned()
        );
        gl.delete_program(Some(&program));
        Err(error)
    } else {
        Ok(program)
    }
}

pub fn load_texture(
    gl: &WebGl2RenderingContext,
    url: &str,
) -> Option<WebGlTexture> {
    let texture = gl.create_texture();
    //
    let pixel: Vec<u8> = vec![0, 0, 255, 255];
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture.as_ref());
    gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        WebGl2RenderingContext::TEXTURE_2D, 0,
        WebGlRenderingContext::RGBA as i32,
        1, 1, 0,
        WebGlRenderingContext::RGBA,
        WebGl2RenderingContext::UNSIGNED_BYTE,
        Some(pixel.as_slice()),
    ).unwrap();
    //
    {
        let image = HtmlImageElement::new().unwrap();
        let gl_ = gl.clone();
        let texture_ = texture.clone();
        let closure = Closure::wrap(Box::new(
            move |event: Event| {
                let image = event.target().unwrap()
                    .dyn_into::<HtmlImageElement>()
                    .unwrap();
                gl_.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture_.as_ref());
                gl_.tex_image_2d_with_u32_and_u32_and_html_image_element(
                    WebGl2RenderingContext::TEXTURE_2D, 0,
                    WebGlRenderingContext::RGBA as i32,
                    WebGlRenderingContext::RGBA,
                    WebGl2RenderingContext::UNSIGNED_BYTE,
                    &image,
                ).unwrap();
                //
                gl_.generate_mipmap(
                    WebGl2RenderingContext::TEXTURE_2D,
                );
                gl_.tex_parameteri(
                    WebGl2RenderingContext::TEXTURE_2D,
                    WebGl2RenderingContext::TEXTURE_WRAP_S,
                    WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
                );
                gl_.tex_parameteri(
                    WebGl2RenderingContext::TEXTURE_2D,
                    WebGl2RenderingContext::TEXTURE_WRAP_T,
                    WebGl2RenderingContext::CLAMP_TO_EDGE as i32,
                );
                gl_.tex_parameteri(
                    WebGl2RenderingContext::TEXTURE_2D,
                    WebGl2RenderingContext::TEXTURE_MIN_FILTER,
                    WebGl2RenderingContext::LINEAR as i32,
                );
            }
        ) as Box<dyn FnMut(_)>);
        image.add_event_listener_with_callback(
            "load", closure.as_ref().unchecked_ref(),
        ).unwrap();
        image.set_src(url);
        closure.forget();
    }
    //
    texture
}