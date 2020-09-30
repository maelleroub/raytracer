use std::ops;

pub struct Vec3(pub f64, pub f64, pub f64);

impl ops::Add<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn add(self, other: &Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, other: &Vec3) {
        *self = &*self + other
    }
}

impl ops::Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl ops::SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, other: &Vec3) {
        *self = &*self - other
    }
}

impl ops::Mul<f64> for &Vec3 {
    type Output = Vec3;
    fn mul(self, n: f64) -> Vec3 {
        Vec3(self.0 * n, self.1 * n, self.2 * n)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, n: f64) {
        *self = &*self * n
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, n: f64) -> Vec3 {
        Vec3(self.0 / n, self.1 / n, self.2 / n)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, n: f64) {
        *self = &*self / n
    }
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }
    pub fn print(&self) {
        print!("{{ {}, {}, {} }}", self.0, self.1, self.2);
    }
}