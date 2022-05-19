use js_sys::{Float32Array, Uint16Array};
use wasm_bindgen::JsValue;
use web_sys::WebGl2RenderingContext as WebGl;
use web_sys::WebGlVertexArrayObject;
use web_sys::WebGlProgram;
use web_sys::WebGlTexture;
use web_sys::WebGlUniformLocation;
use crate::engine::Context;
use crate::utils;

pub struct Cube {
    gl: WebGl,
    vao: Option<WebGlVertexArrayObject>,
    program: Option<WebGlProgram>,
    pro_mat: Option<WebGlUniformLocation>,
    mod_mat: Option<WebGlUniformLocation>,
    sampler: Option<WebGlUniformLocation>,
}

impl Drop for Cube {
    fn drop(&mut self) {
        self.gl.delete_program(self.program.as_ref());
        self.gl.delete_vertex_array(self.vao.as_ref());
    }
}

impl Cube {
    pub fn create(context: &dyn Context) -> Result<Self, JsValue> {
        let gl = context.gl().clone();
        //
        let vao = gl.create_vertex_array();
        gl.bind_vertex_array(vao.as_ref());
        //
        let vertices: Vec<f32> = include!("cube.pos.inc");
        let v_buffer = gl.create_buffer();
        gl.bind_buffer(WebGl::ARRAY_BUFFER, v_buffer.as_ref());
        let array_buffer = unsafe {
            Float32Array::view(vertices.as_slice())
        };
        gl.buffer_data_with_array_buffer_view(
            WebGl::ARRAY_BUFFER,
            &array_buffer,
            WebGl::STATIC_DRAW,
        );
        //
        let indices: Vec<u16> = include!("cube.idx.inc");
        let i_buffer = gl.create_buffer();
        gl.bind_buffer(WebGl::ELEMENT_ARRAY_BUFFER, i_buffer.as_ref());
        let array_buffer = unsafe {
            Uint16Array::view(indices.as_slice())
        };
        gl.buffer_data_with_array_buffer_view(
            WebGl::ELEMENT_ARRAY_BUFFER,
            &array_buffer,
            WebGl::STATIC_DRAW,
        );
        //
        let v_shader = Some(utils::create_shader(
            gl.as_ref(),
            WebGl::VERTEX_SHADER,
            r###"#version 300 es
            precision highp float;
            in vec3 position;
            in vec2 texcoord;
            in vec3 normal;
            uniform mat4 umm;
            uniform mat4 upm;
            out vec2 vTexCoord;
            out vec3 vLighting;
            void main() {
                gl_Position = upm * umm * vec4(position,1.0);
                vTexCoord = texcoord;
                //
                vec3 ambientColor = vec3(0.2, 0.2, 0.2);
                vec3 directionColor = vec3(1.0, 1.0, 1.0);
                vec3 directionLight = normalize(vec3(0.85, 0.8, 0.75));
                vec4 transformNormal = transpose(inverse(umm)) * vec4(normal, 1.0);
                float directional = max(dot(transformNormal.xyz, directionLight), 0.0);
                //
                vLighting = ambientColor + (directionColor * directional);
            }
            "###,
        )?);
        let f_shader = Some(utils::create_shader(
            gl.as_ref(),
            WebGl::FRAGMENT_SHADER,
            r###"#version 300 es
            precision highp float;
            in vec2 vTexCoord;
            in vec3 vLighting;
            uniform sampler2D uSampler;
            out vec4 outColor;
            void main() {
                vec4 texColor = texture(uSampler, vTexCoord);
                outColor = vec4(texColor.rgb * vLighting, texColor.a);
            }
            "###,
        )?);
        let program = Some(utils::create_program(
            gl.as_ref(),
            v_shader.as_ref().unwrap(),
            f_shader.as_ref().unwrap(),
        )?);
        gl.use_program(program.as_ref());
        let position = gl.get_attrib_location(
            program.as_ref().unwrap(), "position",
        ) as u32;
        let texcoord = gl.get_attrib_location(
            program.as_ref().unwrap(), "texcoord",
        ) as u32;
        let normal = gl.get_attrib_location(
            program.as_ref().unwrap(), "normal",
        ) as u32;
        let pro_mat = gl.get_uniform_location(
            program.as_ref().unwrap(), "upm",
        );
        let mod_mat = gl.get_uniform_location(
            program.as_ref().unwrap(), "umm",
        );
        let sampler = gl.get_uniform_location(
            program.as_ref().unwrap(), "uSampler",
        );
        //
        gl.enable_vertex_attrib_array(position);
        gl.vertex_attrib_pointer_with_i32(
            position, 3, WebGl::FLOAT, false,
            8 * std::mem::size_of::<f32>() as i32,
            0,
        );
        gl.enable_vertex_attrib_array(texcoord);
        gl.vertex_attrib_pointer_with_i32(
            texcoord, 2, WebGl::FLOAT, false,
            8 * std::mem::size_of::<f32>() as i32,
            3 * std::mem::size_of::<f32>() as i32,
        );
        gl.enable_vertex_attrib_array(normal);
        gl.vertex_attrib_pointer_with_i32(
            normal, 3, WebGl::FLOAT, false,
            8 * std::mem::size_of::<f32>() as i32,
            5 * std::mem::size_of::<f32>() as i32,
        );

        gl.active_texture(WebGl::TEXTURE0);
        gl.uniform1i(sampler.as_ref(), 0);

        gl.bind_vertex_array(None);
        //
        Ok(Self {
            gl,
            vao,
            program,
            pro_mat,
            mod_mat,
            sampler,
        })
    }

    pub fn draw(&self, context: &dyn Context, texture: Option<&WebGlTexture>) {
        let gl = context.gl().clone();
        //
        gl.use_program(self.program.as_ref());
        gl.bind_vertex_array(self.vao.as_ref());
        gl.uniform_matrix4fv_with_f32_array(
            self.mod_mat.as_ref(), false, context.mod_matrix(),
        );
        gl.uniform_matrix4fv_with_f32_array(
            self.pro_mat.as_ref(), false, context.pro_matrix(),
        );
        //
        gl.bind_texture(WebGl::TEXTURE_2D, texture);
        gl.draw_elements_with_i32(
            WebGl::TRIANGLES,
            36,
            WebGl::UNSIGNED_SHORT,
            0,
        );
    }
}