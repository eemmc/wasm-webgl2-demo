use std::f32::consts::PI;
use std::ops::Deref;
use super::Vec3;

pub struct Mat4 {
    data: [f32; 16],
}

impl Default for Mat4 {
    fn default() -> Self {
        Self {
            data: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ]
        }
    }
}

impl Clone for Mat4 {
    fn clone(&self) -> Self {
        Self { data: self.data.clone() }
    }
}

impl Deref for Mat4 {
    type Target = [f32];

    fn deref(&self) -> &Self::Target {
        &*self.data.as_slice()
    }
}

impl Mat4 {
    pub fn identity(&mut self) {
        self.data.fill(0.0);
        self.data[0] = 1.0;
        self.data[5] = 1.0;
        self.data[10] = 1.0;
        self.data[15] = 1.0;
    }

    pub fn scale(&mut self, vec: &Vec3) {
        for i in 0..4 {
            self.data[0 + i] *= vec.x;
            self.data[4 + i] *= vec.y;
            self.data[8 + i] *= vec.z;
        }
    }

    pub fn translate(&mut self, vec: &Vec3) {
        for i in 0..4 {
            self.data[12 + i] += self.data[i] * vec.x
                + self.data[4 + i] * vec.y
                + self.data[8 + i] * vec.z;
        }
    }

    pub fn rotate(&mut self, angle: f32, vec: &Vec3) {
        let rad = angle * PI / 180.0;
        let sin = rad.sin();
        let cos = rad.cos();
        let tmp = 1.0 - cos;
        let len = vec.length();
        if len > 0.0 {
            let x = vec.x / len;
            let y = vec.y / len;
            let z = vec.z / len;
            //
            let mut swap = Mat4::default();
            swap.data[0] = x * x * tmp + cos;
            swap.data[1] = y * x * tmp + sin * z;
            swap.data[2] = z * x * tmp - sin * y;
            swap.data[3] = 0.0;
            swap.data[4] = x * y * tmp - sin * z;
            swap.data[5] = y * y * tmp + cos;
            swap.data[6] = z * y * tmp + sin * x;
            swap.data[7] = 0.0;
            swap.data[8] = x * z * tmp + sin * y;
            swap.data[9] = y * z * tmp - sin * x;
            swap.data[10] = z * z * tmp + cos;
            swap.data[11] = 0.0;
            swap.data[12] = 0.0;
            swap.data[13] = 0.0;
            swap.data[14] = 0.0;
            swap.data[15] = 1.0;
            //
            swap.multiply(self);
            self.data.copy_from_slice(swap.data.as_slice());
        }
    }

    pub fn ortho(&mut self, l: f32, t: f32, r: f32, b: f32, n: f32, f: f32) {
        let (dx, dy, dz) = (r - l, t - b, f - n);
        if dz != 0.0 && dy != 0.0 && dz != 0.0 {
            let mut swap = Mat4::default();
            swap.data[0] = 2.0 / dx;
            swap.data[5] = 2.0 / dy;
            swap.data[10] = -2.0 / dz;
            swap.data[12] = -(l + r) / dx;
            swap.data[13] = -(t + b) / dy;
            swap.data[14] = -(n + f) / dz;
            //
            swap.multiply(self);
            self.data.copy_from_slice(swap.data.as_slice());
        }
    }

    pub fn frustum(&mut self, l: f32, t: f32, r: f32, b: f32, n: f32, f: f32) {
        let (dx, dy, dz) = (r - l, t - b, f - n);
        if n > 0.0 && f > 0.0 && dx > 0.0 && dy > 0.0 && dz > 0.0 {
            //
            let mut swap = Mat4::default();
            swap.data[0] = 2.0 * n / dx;
            swap.data[1] = 0.0;
            swap.data[2] = 0.0;
            swap.data[3] = 0.0;
            swap.data[4] = 0.0;
            swap.data[5] = 2.0 * n / dy;
            swap.data[6] = 0.0;
            swap.data[7] = 0.0;
            swap.data[8] = (l + r) / dx;
            swap.data[9] = (t + b) / dy;
            swap.data[10] = -(f + n) / dz;
            swap.data[11] = -1.0;
            swap.data[12] = 0.0;
            swap.data[13] = 0.0;
            swap.data[14] = -2.0 * n * f / dz;
            swap.data[15] = 0.0;
            //
            swap.multiply(self);
            self.data.copy_from_slice(swap.data.as_slice());
        }
    }

    pub fn perspective(&mut self, fovy: f32, aspect: f32, near: f32, far: f32) {
        let vertical = (fovy * PI / 360.0).tan() * near;
        let horizontal = vertical * aspect;
        //
        self.frustum(
            -horizontal, vertical,
            horizontal, -vertical,
            near, far,
        );
    }

    pub fn lookat(&mut self, eye: &Vec3, center: &Vec3, up: &Vec3) {
        let mut tmt = [0f32; 9];
        //
        tmt[6] = eye.x - center.x;
        tmt[7] = eye.y - center.y;
        tmt[8] = eye.z - center.z;
        let len = (tmt[6] * tmt[6] + tmt[7] * tmt[7] + tmt[8] * tmt[8]).sqrt();
        if len != 0.0 {
            tmt[6] /= len;
            tmt[7] /= len;
            tmt[8] /= len;
        }
        //
        tmt[0] = up.y * tmt[8] - up.z * tmt[7];
        tmt[1] = up.z * tmt[6] - up.x * tmt[8];
        tmt[2] = up.x * tmt[7] - up.y * tmt[6];
        let len = (tmt[0] * tmt[0] + tmt[1] * tmt[1] + tmt[2] * tmt[2]).sqrt();
        if len != 0.0 {
            tmt[0] /= len;
            tmt[1] /= len;
            tmt[2] /= len;
        }
        //
        tmt[3] = tmt[7] * tmt[2] - tmt[8] * tmt[1];
        tmt[4] = tmt[8] * tmt[0] - tmt[6] * tmt[2];
        tmt[5] = tmt[6] * tmt[1] - tmt[7] * tmt[0];
        let len = (tmt[3] * tmt[3] + tmt[4] * tmt[4] + tmt[5] * tmt[5]).sqrt();
        if len != 0.0 {
            tmt[3] /= len;
            tmt[4] /= len;
            tmt[5] /= len;
        }
        //
        self.data[0] = tmt[0];
        self.data[1] = tmt[3];
        self.data[2] = tmt[6];
        self.data[3] = 0.0;
        self.data[4] = tmt[1];
        self.data[5] = tmt[4];
        self.data[6] = tmt[7];
        self.data[7] = 0.0;
        self.data[8] = tmt[2];
        self.data[9] = tmt[5];
        self.data[10] = tmt[8];
        self.data[11] = 0.0;
        self.data[12] = -(tmt[0] * eye.x + tmt[1] * eye.y + tmt[2] * eye.z);
        self.data[13] = -(tmt[3] * eye.x + tmt[4] * eye.y + tmt[5] * eye.z);
        self.data[14] = -(tmt[6] * eye.x + tmt[7] * eye.y + tmt[8] * eye.z);
        self.data[15] = 1.0;
    }

    pub fn multiply(&mut self, rhs: &Self) {
        for i in 0..4 {
            let m: usize = i * 4;
            //
            let x = self.data[m + 0] * rhs.data[0] + self.data[m + 1] * rhs.data[4]
                + self.data[m + 2] * rhs.data[8] + self.data[m + 3] * rhs.data[12];
            let y = self.data[m + 0] * rhs.data[1] + self.data[m + 1] * rhs.data[5]
                + self.data[m + 2] * rhs.data[9] + self.data[m + 3] * rhs.data[13];
            let z = self.data[m + 0] * rhs.data[2] + self.data[m + 1] * rhs.data[6]
                + self.data[m + 2] * rhs.data[10] + self.data[m + 3] * rhs.data[14];
            let w = self.data[m + 0] * rhs.data[3] + self.data[m + 1] * rhs.data[7]
                + self.data[m + 2] * rhs.data[11] + self.data[m + 3] * rhs.data[15];
            //
            self.data[m + 0] = x;
            self.data[m + 1] = y;
            self.data[m + 2] = z;
            self.data[m + 3] = w;
        }
    }
}