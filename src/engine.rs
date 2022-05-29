use std::collections::LinkedList;
use std::rc::Rc;
use web_sys::WebGl2RenderingContext as WebGl;
use web_sys::WebGlTexture;
use crate::glm::{Mat4, Vec3};
use crate::obj::Quad;
use crate::utils;

pub trait Context {
    fn gl(&self) -> &WebGl;
    fn pro_matrix(&self) -> &[f32];
    fn mod_matrix(&self) -> &[f32];
}

pub struct Engine {
    gl: Rc<WebGl>,
    mod_mat: Mat4,
    pro_mat: Mat4,
    stamp: f64,
    texture: Option<WebGlTexture>,
    quad: Option<Quad>,
}

impl Context for Engine {
    fn gl(&self) -> &WebGl {
        self.gl.as_ref()
    }

    fn pro_matrix(&self) -> &[f32] {
        &*self.pro_mat
    }

    fn mod_matrix(&self) -> &[f32] {
        &*self.mod_mat
    }
}

impl Engine {
    pub fn create(gl: WebGl) -> Self {
        Self {
            gl: Rc::new(gl),
            pro_mat: Mat4::default(),
            mod_mat: Mat4::default(),
            stamp: 0.0,
            texture: None,
            quad: None,
        }
    }
}

impl Engine {
    pub fn setup(&mut self) {
        self.pro_mat.perspective(45.0, 360.0 / 480.0, 0.1, 100.0);
        self.pro_mat.translate(&Vec3::wrap(0.0, 0.0, -6.0));

        let gl = self.gl().clone();
        self.gl.viewport(0, 0, 360, 480);
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        //gl.enable(WebGl::BLEND);
        //gl.blend_func(WebGl::SRC_ALPHA, WebGl::ONE);
        gl.enable(WebGl::DEPTH_TEST);
        gl.depth_func(WebGl::LEQUAL);

        self.quad = Some(Quad::create(self).unwrap());
        self.texture = utils::load_texture(
            gl.as_ref(), "cubetexture.png",
        );
    }

    pub fn input(&mut self, _x: f32, _y: f32, _pressed: bool) {}

    pub fn update(&mut self) {
        {
            self.gl.clear(WebGl::COLOR_BUFFER_BIT | WebGl::DEPTH_BUFFER_BIT);
            self.quad.as_ref().unwrap().draw(self, self.texture.as_ref());
        }
    }
}