use std::ops;
use std::fmt;
use super::rt;

#[derive(Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

//Addition (Vec3 + Vec3)
impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = *self + rhs
    }
}

//Substraction (Vec3 - Vec3)
impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = *self - rhs
    }
}

//Multiplication (Vec3 * Vec3)
impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = *self * rhs
    }
}

//Multiplication (Vec3 * f64)
impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, n: f64) -> Vec3 {
        Vec3(self.0 * n, self.1 * n, self.2 * n)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, n: f64) {
        *self = *self * n
    }
}

//Multiplication (f64 * Vec3)
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

//Division (Vec3 / f64)
impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, n: f64) -> Vec3 {
        Vec3(self.0 / n, self.1 / n, self.2 / n)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, n: f64) {
        *self = *self / n
    }
}

//Negation (-Vec3)
impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}, {}, {}}}", self.0, self.1, self.2)
    }
}

impl Vec3 {
    #[inline]
    pub fn new() -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }

    #[inline]
    pub fn norm_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    #[inline]
    pub fn norm(&self) -> f64 {
        f64::sqrt(self.norm_squared())
    }

    #[inline]
    pub fn normalize(&self) -> Vec3 {
        let norm = self.norm();
        Vec3(self.0 / norm, self.1 / norm, self.2 / norm)
    }

    #[inline]
    pub fn dot(lhs: Vec3, rhs: Vec3) -> f64 {
        lhs.0 * rhs.0 + lhs.1 * rhs.1 + lhs.2 * rhs.2
    }

    #[inline]
    pub fn cross(lhs: Vec3, rhs: Vec3) -> Vec3 {
        Vec3(lhs.1 * rhs.2 - lhs.2 * rhs.1,
             lhs.2 * rhs.0 - lhs.0 * rhs.2,
             lhs.0 * rhs.1 - lhs.1 * rhs.0)
    }

    #[inline]
    pub fn random() -> Vec3 {
        Vec3(rt::random_double(), rt::random_double(), rt::random_double())
    }

    #[inline]
    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3(
            rt::random_double_range(min, max),
            rt::random_double_range(min, max),
            rt::random_double_range(min, max)
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.norm_squared() >= 1.0 { continue; }
            return p;
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        let a = rt::random_double_range(0.0, 2.0 * rt::PI);
        let z = rt::random_double_range(-1.0, 1.0);
        let r = f64::sqrt(1.0 - z * z);
        Vec3(r * f64::cos(a), r * f64::sin(a), z)
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3(rt::random_double_range(-1.0, 1.0),
                        rt::random_double_range(-1.0, 1.0),
                        0.0);
            if p.norm_squared() >= 1.0 { continue; }
            return p;
        }
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * Vec3::dot(v, n) * n
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = Vec3::dot(-uv, n);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel =
            -(f64::abs(1.0 - r_out_perp.norm_squared())).sqrt() * n;
        return r_out_perp + r_out_parallel;
    }
}
