use js_sys::Float32Array;
use wasm_bindgen::JsValue;
use web_sys::{WebGl2RenderingContext as WebGl, WebGlBuffer};
use web_sys::WebGlVertexArrayObject;
use web_sys::WebGlProgram;
use web_sys::WebGlTexture;
use web_sys::WebGlUniformLocation;
use crate::engine::Context;
use crate::utils;

const FLOAT_SIZE: usize = std::mem::size_of::<f32>();

pub struct Quad {
    gl: WebGl,
    pro: Option<WebGlProgram>,
    buf: Option<WebGlBuffer>,
    vao: Option<WebGlVertexArrayObject>,
    upm: Option<WebGlUniformLocation>,
    uvm: Option<WebGlUniformLocation>,
}

impl Drop for Quad {
    fn drop(&mut self) {
        self.gl.delete_program(self.pro.as_ref());
        self.gl.delete_vertex_array(self.vao.as_ref());
        self.gl.delete_buffer(self.buf.as_ref());
    }
}

impl Quad {
    pub fn create(context: &dyn Context) -> Result<Self, JsValue> {
        let gl = context.gl().clone();
        //
        let vao = gl.create_vertex_array();
        gl.bind_vertex_array(vao.as_ref());
        //
        let vertices: Vec<f32> = vec![
            1.0, 1.0, 1.0, 0.0,
            1.0, -1.0, 1.0, 1.0,
            -1.0, -1.0, 0.0, 1.0,
            -1.0, -1.0, 0.0, 1.0,
            -1.0, 1.0, 0.0, 0.0,
            1.0, 1.0, 1.0, 0.0,
        ];
        let buffer = gl.create_buffer();
        gl.bind_buffer(WebGl::ARRAY_BUFFER, buffer.as_ref());
        let array_buffer = unsafe { Float32Array::view(vertices.as_slice()) };
        gl.buffer_data_with_array_buffer_view(
            WebGl::ARRAY_BUFFER,
            &array_buffer,
            WebGl::STATIC_DRAW,
        );
        //
        let v_shader = Some(utils::create_shader(
            gl.as_ref(),
            WebGl::VERTEX_SHADER,
            r###"#version 300 es
            precision highp float;
            layout(location = 0) in vec2 position;
            layout(location = 1) in vec2 texcoord;
            uniform mat4 uvm;
            uniform mat4 upm;
            out vec2 vTexCoord;
            void main() {
                gl_Position = upm * uvm * vec4(position, 0.0, 1.0);
                vTexCoord = texcoord;
            }
            "###,
        )?);
        let f_shader = Some(utils::create_shader(
            gl.as_ref(),
            WebGl::FRAGMENT_SHADER,
            r###"#version 300 es
            precision highp float;
            in vec2 vTexCoord;
            uniform sampler2D uSampler;
            out vec4 outColor;
            void main() {
                outColor = texture(uSampler, vTexCoord);
            }
            "###,
        )?);
        let pro = Some(utils::create_program(
            gl.as_ref(),
            v_shader.as_ref().unwrap(),
            f_shader.as_ref().unwrap(),
        )?);
        //
        gl.use_program(pro.as_ref());
        gl.bind_attrib_location(pro.as_ref().unwrap(), 0, "position");
        gl.bind_attrib_location(pro.as_ref().unwrap(), 1, "texcoord");
        let upm = gl.get_uniform_location(pro.as_ref().unwrap(), "upm");
        let uvm = gl.get_uniform_location(pro.as_ref().unwrap(), "uvm");
        let tex = gl.get_uniform_location(pro.as_ref().unwrap(), "uSampler");
        //
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_with_i32(
            0, 2, WebGl::FLOAT, false,
            4 * FLOAT_SIZE as i32,
            0,
        );
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_with_i32(
            1, 2, WebGl::FLOAT, false,
            4 * FLOAT_SIZE as i32,
            2 * FLOAT_SIZE as i32,
        );
        gl.active_texture(WebGl::TEXTURE0);
        gl.uniform1i(tex.as_ref(), 0);
        //
        gl.bind_vertex_array(None);

        //
        Ok(Self { gl, pro, buf: buffer, vao, upm, uvm })
    }

    pub fn draw(&self, context: &dyn Context, texture: Option<&WebGlTexture>) {
        let gl = context.gl().clone();
        //
        gl.use_program(self.pro.as_ref());
        gl.bind_vertex_array(self.vao.as_ref());
        //
        gl.uniform_matrix4fv_with_f32_array(
            self.upm.as_ref(), false, context.pro_matrix(),
        );
        gl.uniform_matrix4fv_with_f32_array(
            self.uvm.as_ref(), false, context.mod_matrix(),
        );
        //
        gl.bind_texture(WebGl::TEXTURE_2D, texture);
        gl.draw_arrays(WebGl::TRIANGLES, 0, 6);
        //
        gl.bind_vertex_array(None);
    }
}