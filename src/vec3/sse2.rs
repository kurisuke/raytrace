use std::fmt;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[derive(Debug, Clone, Copy)]
pub struct Vec3(__m128);

impl Vec3 {
    #[inline]
    pub fn default() -> Vec3 {
        unsafe { Vec3(_mm_set1_ps(0.0)) }
    }

    #[inline]
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        unsafe { Vec3(_mm_set_ps(e2, e2, e1, e0)) }
    }

    #[inline]
    pub fn x(&self) -> f32 {
        unsafe { _mm_cvtss_f32(self.0) }
    }

    #[inline]
    pub fn y(&self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, 0b01_01_01_01)) }
    }

    #[inline]
    pub fn z(&self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.0, self.0, 0b10_10_10_10)) }
    }

    #[inline]
    pub fn set_x(&mut self, x: f32) {
        unsafe {
            self.0 = _mm_move_ss(self.0, _mm_set_ss(x));
        }
    }

    #[inline]
    pub fn set_y(&mut self, y: f32) {
        unsafe {
            let mut t = _mm_move_ss(self.0, _mm_set_ss(y));
            t = _mm_shuffle_ps(t, t, 0b11_10_00_00);
            self.0 = _mm_move_ss(t, self.0);
        }
    }

    #[inline]
    pub fn set_z(&mut self, z: f32) {
        unsafe {
            let mut t = _mm_move_ss(self.0, _mm_set_ss(z));
            t = _mm_shuffle_ps(t, t, 0b11_00_01_00);
            self.0 = _mm_move_ss(t, self.0);
        }
    }

    #[inline]
    fn yzx(self) -> Vec3 {
        unsafe { Vec3(_mm_shuffle_ps(self.0, self.0, 0b00_00_10_01)) }
    }

    #[inline]
    fn zxy(self) -> Vec3 {
        unsafe { Vec3(_mm_shuffle_ps(self.0, self.0, 0b01_01_00_10)) }
    }

    #[inline]
    fn sum(self) -> f32 {
        self.x() + self.y() + self.z()
    }

    #[inline]
    pub fn r(&self) -> f32 {
        self.x()
    }

    #[inline]
    pub fn g(&self) -> f32 {
        self.y()
    }

    #[inline]
    pub fn b(&self) -> f32 {
        self.z()
    }

    #[inline]
    pub fn i(&self, i: usize) -> f32 {
        match i {
            0 => self.x(),
            1 => self.y(),
            2 => self.z(),
            _ => panic!("Vec3::i: wrong index"),
        }
    }

    #[inline]
    pub fn set_i(&mut self, i: usize, s: f32) {
        match i {
            0 => self.set_x(s),
            1 => self.set_y(s),
            2 => self.set_z(s),
            _ => panic!("Vec3::set_i: wrong index"),
        }
    }

    #[inline]
    pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
        (v1 * v2).sum()
    }

    #[inline]
    pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        (v1.zxy() * v2 - v1 * v2.zxy()).zxy()
    }

    #[inline]
    pub fn len(self) -> f32 {
        Vec3::dot(self, self).sqrt()
    }

    #[inline]
    pub fn len_squared(self) -> f32 {
        Vec3::dot(self, self)
    }

    #[inline]
    pub fn normalize(self) -> Vec3 {
        let inv_length = 1.0 / Vec3::dot(self, self).sqrt();
        self * inv_length
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    #[inline]
    fn neg(self) -> Vec3 {
        unsafe { Vec3(_mm_sub_ps(_mm_set1_ps(0.0), self.0)) }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: Vec3) -> Vec3 {
        unsafe { Vec3(_mm_add_ps(self.0, rhs.0)) }
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Vec3) {
        unsafe { self.0 = _mm_add_ps(self.0, rhs.0) }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: Vec3) -> Vec3 {
        unsafe { Vec3(_mm_sub_ps(self.0, rhs.0)) }
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Vec3) {
        unsafe { self.0 = _mm_sub_ps(self.0, rhs.0) }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        unsafe { Vec3(_mm_mul_ps(self.0, rhs.0)) }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: f32) -> Vec3 {
        unsafe { Vec3(_mm_mul_ps(self.0, _mm_set1_ps(rhs))) }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        unsafe { Vec3(_mm_mul_ps(_mm_set1_ps(self), rhs.0)) }
    }
}

impl MulAssign<Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Vec3) {
        unsafe {
            self.0 = _mm_mul_ps(self.0, rhs.0);
        }
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
        unsafe { self.0 = _mm_mul_ps(self.0, _mm_set1_ps(rhs)) }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: Vec3) -> Vec3 {
        unsafe { Vec3(_mm_div_ps(self.0, rhs.0)) }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, rhs: f32) -> Vec3 {
        unsafe { Vec3(_mm_div_ps(self.0, _mm_set1_ps(rhs))) }
    }
}

impl DivAssign<Vec3> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: Vec3) {
        unsafe {
            self.0 = _mm_div_ps(self.0, rhs.0);
        }
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        unsafe { self.0 = _mm_div_ps(self.0, _mm_set1_ps(rhs)) }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} {} {}]", self.x(), self.y(), self.z())
    }
}

impl Sum for Vec3 {
    #[inline]
    fn sum<I>(it: I) -> Self
    where
        I: Iterator<Item = Vec3>,
    {
        it.fold(Vec3::default(), |acc, x| acc + x)
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.x() == other.x() && self.y() == other.y() && self.z() == other.z()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_xyz() {
        let v = Vec3::new(3., 4., 5.);
        assert_eq!(v.x(), 3.);
        assert_eq!(v.y(), 4.);
        assert_eq!(v.z(), 5.);
        assert_eq!(v.r(), 3.);
        assert_eq!(v.g(), 4.);
        assert_eq!(v.b(), 5.);
    }

    #[test]
    fn test_neg() {
        let v_input = Vec3::new(3., 4., 5.);
        let v_neg = Vec3::new(-3., -4., -5.);
        assert_eq!(-v_input, v_neg);
    }

    #[test]
    fn test_add() {
        let v1 = Vec3::new(3., 4., 5.);
        let v2 = Vec3::new(3., 4., 5.);
        let v_sum = Vec3::new(6., 8., 10.);
        assert_eq!(v1 + v2, v_sum);

        let mut v3 = v1;
        v3 += v2;
        assert_eq!(v3, v_sum);
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3::new(3., 4., 5.);
        let v2 = Vec3::new(6., 8., 10.);
        let v_diff = Vec3::new(-3., -4., -5.);
        assert_eq!(v1 - v2, v_diff);

        let mut v3 = v1;
        v3 -= v2;
        assert_eq!(v3, v_diff);
    }

    #[test]
    fn test_mul_s() {
        let v_res = Vec3::new(9., 12., 15.);
        let s = 3.;

        let v1 = Vec3::new(3., 4., 5.);
        assert_eq!(v_res, v1 * s);

        let mut v2 = v1;
        v2 *= s;
        assert_eq!(v_res, v2);
    }

    #[test]
    fn test_div_s() {
        let v_res = Vec3::new(3., 4., 5.);
        let s = 3.;

        let v1 = Vec3::new(9., 12., 15.);
        assert_eq!(v_res, v1 / s);

        let mut v2 = v1;
        v2 /= s;
        assert_eq!(v_res, v2);
    }

    #[test]
    fn test_mul_v() {
        let v_res = Vec3::new(6., 12., 20.);
        let v = Vec3::new(2., 3., 4.);

        let v1 = Vec3::new(3., 4., 5.);
        assert_eq!(v_res, v1 * v);

        let mut v2 = v1;
        v2 *= v;
        assert_eq!(v_res, v2);
    }

    #[test]
    fn test_div_v() {
        let v_res = Vec3::new(3., 4., 5.);
        let v = Vec3::new(2., 3., 4.);

        let v1 = Vec3::new(6., 12., 20.);
        assert_eq!(v_res, v1 / v);

        let mut v2 = v1;
        v2 /= v;
        assert_eq!(v_res, v2);
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(-7., 8., 9.);
        let s_res = 36.;
        assert_eq!(s_res, Vec3::dot(v1, v2));
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(-7., 8., 9.);
        let v_res = Vec3::new(-6., -30., 22.);
        assert_eq!(v_res, Vec3::cross(v1, v2));
    }

    #[test]
    fn test_length() {
        let v = Vec3::new(3., 4., 5.);
        let res = 50.;
        assert_eq!(res, v.len_squared());
        assert_eq!(res.sqrt(), v.len());
    }

    #[test]
    fn test_normalize() {
        let epsilon = 1e-4;
        let v = Vec3::new(3., 4., 5.);
        let v_res = Vec3::new(0.42426, 0.56569, 0.70711);
        let v_diff = v_res - v.normalize();

        assert!(v_diff.x().abs() < epsilon);
        assert!(v_diff.y().abs() < epsilon);
        assert!(v_diff.z().abs() < epsilon);
    }
}
