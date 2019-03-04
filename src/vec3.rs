use std::iter::Sum;
use std::ops::{Neg, Index, IndexMut,
               Add, AddAssign, Sub, SubAssign};
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn default() -> Vec3 {
        Vec3 {
            e: [0.0, 0.0, 0.0]
        }
    }

    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 {
            e: [e0, e1, e2]
        }
    }

    pub fn x(&self) -> f64 { self.e[0] }
    pub fn y(&self) -> f64 { self.e[1] }
    pub fn z(&self) -> f64 { self.e[2] }
    pub fn r(&self) -> f64 { self.e[0] }
    pub fn g(&self) -> f64 { self.e[1] }
    pub fn b(&self) -> f64 { self.e[2] }

    pub fn mul_s(v: &Vec3, s: f64) -> Vec3 {
        Vec3 {
            e: [v.e[0] * s, v.e[1] * s, v.e[2] * s]
        }
    }

    pub fn mul_v(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3 {
            e: [v1.e[0] * v2.e[0], v1.e[1] * v2.e[1], v1.e[2] * v2.e[2]]
        }
    }

    pub fn div_s(v: &Vec3, s: f64) -> Vec3 {
        Vec3 {
            e: [v.e[0] / s, v.e[1] / s, v.e[2] / s]
        }
    }

    pub fn div_v(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3 {
            e: [v1.e[0] / v2.e[0], v1.e[1] / v2.e[1], v1.e[2] / v2.e[2]]
        }
    }

    pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
        v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
    }

    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3 {
            e: [v1.e[1] * v2.e[2] - v1.e[2] * v2.e[1],
                v1.e[2] * v2.e[0] - v1.e[0] * v2.e[2],
                v1.e[0] * v2.e[1] - v1.e[1] * v2.e[0]]
        }
    }

    pub fn len(&self) -> f64 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn len_squared(&self) -> f64 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2])
    }

    pub fn normalize(&self) -> Vec3 {
        let k = self.len();
        Vec3 {
            e: [self.e[0] / k, self.e[1] / k, self.e[2] / k]
        }
    }

    pub fn normalize01(&self) -> Vec3 {
        let n = self.normalize();
        Vec3::mul_s(&Vec3::new(n.e[0] + 1.0, n.e[1] + 1.0, n.e[2] + 1.0), 0.5)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]]
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &f64 {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2]]
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2]]
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} {} {}]", self.e[0], self.e[1], self.e[2])
    }
}

impl Sum for Vec3 {
    fn sum<I>(it: I) -> Self
    where I: Iterator<Item = Vec3> {
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
        let v = Vec3::new(3., 4., 5.);
        let s = 3.;
        let v_res = Vec3::new(9., 12., 15.);
        assert_eq!(v_res, Vec3::mul_s(&v, s));
    }

    #[test]
    fn test_div_s() {
        let v = Vec3::new(9., 12., 15.);
        let s = 3.;
        let v_res = Vec3::new(3., 4., 5.);
        assert_eq!(v_res, Vec3::div_s(&v, s));
    }

    #[test]
    fn test_mul_v() {
        let v1 = Vec3::new(3., 4., 5.);
        let v2 = Vec3::new(2., 3., 4.);
        let v_res = Vec3::new(6., 12., 20.);
        assert_eq!(v_res, Vec3::mul_v(&v1, &v2));
    }

    #[test]
    fn test_div_v() {
        let v1 = Vec3::new(6., 12., 20.);
        let v2 = Vec3::new(2., 3., 4.);
        let v_res = Vec3::new(3., 4., 5.);
        assert_eq!(v_res, Vec3::div_v(&v1, &v2));
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(-7., 8., 9.);
        let s_res = 36.;
        assert_eq!(s_res, Vec3::dot(&v1, &v2));
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(1., 2., 3.);
        let v2 = Vec3::new(-7., 8., 9.);
        let v_res = Vec3::new(-6., -30., 22.);
        assert_eq!(v_res, Vec3::cross(&v1, &v2));
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