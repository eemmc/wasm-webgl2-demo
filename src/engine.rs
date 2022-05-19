use std::rc::Rc;
use web_sys::WebGl2RenderingContext as WebGl;
use web_sys::WebGlTexture;
use crate::cube::Cube;
use crate::math::{Mat4, Vec3};
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
    cube: Option<Cube>,
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
            cube: None,
        }
    }
}

impl Engine {
    pub fn setup(&mut self) {
        self.pro_mat.perspective(45.0, 480.0 / 360.0, 0.1, 100.0);
        self.pro_mat.translate(&Vec3::wrap(0.0, 0.0, -6.0));

        let gl = self.gl().clone();
        gl.viewport(0, 0, 480, 360);
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear_depth(1.0);
        gl.enable(WebGl::DEPTH_TEST);
        gl.depth_func(WebGl::LEQUAL);

        self.cube = Some(Cube::create(self).unwrap());
        self.texture = utils::load_texture(
            gl.as_ref(), "cubetexture.png",
        );
    }

    pub fn input(&mut self, _x: f32, _y: f32, _pressed: bool) {}

    pub fn update(&mut self) {
        let now = js_sys::Date::now();
        let rz = (90.0 * (now - self.stamp) / 1000.0) as f32;
        self.stamp = now;
        self.mod_mat.rotate(rz, &Vec3::wrap(0.0, 1.0, 1.0));
        //
        self.gl().clear(
            WebGl::COLOR_BUFFER_BIT | WebGl::DEPTH_BUFFER_BIT
        );
        self.cube.as_ref().unwrap().draw(
            self, self.texture.as_ref(),
        );
    }
}