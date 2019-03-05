use std::fmt;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    #[inline]
    pub fn default() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    #[inline]
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.e[0]
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.e[1]
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.e[2]
    }

    #[inline]
    pub fn set_x(&mut self, s: f32) {
        self.e[0] = s;
    }

    #[inline]
    pub fn set_y(&mut self, s: f32) {
        self.e[1] = s;
    }

    #[inline]
    pub fn set_z(&mut self, s: f32) {
        self.e[2] = s;
    }

    #[inline]
    pub fn r(&self) -> f32 {
        self.e[0]
    }

    #[inline]
    pub fn g(&self) -> f32 {
        self.e[1]
    }

    #[inline]
    pub fn b(&self) -> f32 {
        self.e[2]
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
        v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
    }

    #[inline]
    pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3 {
            e: [
                v1.e[1] * v2.e[2] - v1.e[2] * v2.e[1],
                v1.e[2] * v2.e[0] - v1.e[0] * v2.e[2],
                v1.e[0] * v2.e[1] - v1.e[1] * v2.e[0],
            ],
        }
    }

    #[inline]
    pub fn len(self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    #[inline]
    pub fn len_squared(self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2])
    }

    #[inline]
    pub fn normalize(self) -> Vec3 {
        let k = self.len();
        Vec3 {
            e: [self.e[0] / k, self.e[1] / k, self.e[2] / k],
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl SubAssign for Vec3 {
    #[inline]
    fn sub_assign(&mut self, other: Vec3) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            e: [self.e[0] * other, self.e[1] * other, self.e[2] * other],
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self * other.e[0], self * other.e[1], self * other.e[2]],
        }
    }
}

impl MulAssign<Vec3> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: Vec3) {
        self.e[0] *= other.e[0];
        self.e[1] *= other.e[1];
        self.e[2] *= other.e[2];
    }
}

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        self.e[0] *= other;
        self.e[1] *= other;
        self.e[2] *= other;
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] / other.e[0],
                self.e[1] / other.e[1],
                self.e[2] / other.e[2],
            ],
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, other: f32) -> Vec3 {
        Vec3 {
            e: [self.e[0] / other, self.e[1] / other, self.e[2] / other],
        }
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self / other.e[0], self / other.e[1], self / other.e[2]],
        }
    }
}

impl DivAssign<Vec3> for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: Vec3) {
        self.e[0] /= other.e[0];
        self.e[1] /= other.e[1];
        self.e[2] /= other.e[2];
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        self.e[0] /= other;
        self.e[1] /= other;
        self.e[2] /= other;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} {} {}]", self.e[0], self.e[1], self.e[2])
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
