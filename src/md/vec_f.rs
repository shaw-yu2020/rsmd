use super::Fmd;
use rand::Rng;
#[derive(Copy, Clone, Debug)]
pub struct VecF {
    x: Fmd,
    y: Fmd,
}
impl VecF {
    pub fn new(x: Fmd, y: Fmd) -> Self {
        Self { x, y }
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
    pub fn rand() -> Self {
        let rng: f64 = rand::thread_rng().gen();
        let v: f64 = 2.0 * std::f64::consts::PI * rng;
        Self::new(v.cos(), v.sin())
    }
    pub fn set(&mut self, x: Fmd, y: Fmd) {
        self.x = x;
        self.y = y;
    }
    pub fn x(&self) -> Fmd {
        self.x
    }
    pub fn y(&self) -> Fmd {
        self.y
    }
    pub fn sum(&self) -> Fmd {
        self.x + self.y
    }
    pub fn prod(&self) -> Fmd {
        self.x * self.y
    }
    pub fn len_sq(&self) -> Fmd {
        self.x.powi(2) + self.y.powi(2)
    }
    pub fn scale(&self, t: Fmd) -> Self {
        Self {
            x: self.x * t,
            y: self.y * t,
        }
    }
    pub fn unscale(&self, t: Fmd) -> Self {
        Self {
            x: self.x / t,
            y: self.y / t,
        }
    }
}
impl VecF {
    pub fn wrap_all(&mut self, region: &VecF) {
        if self.x >= 0.5 * region.x {
            self.x -= region.x
        } else if self.x < -0.5 * region.x {
            self.x += region.x
        }
        if self.y >= 0.5 * region.y {
            self.y -= region.y
        } else if self.y < -0.5 * region.y {
            self.y += region.y
        }
    }
}
impl std::ops::Add for VecF {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl std::ops::Sub for VecF {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl std::ops::Mul for VecF {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}
impl std::ops::Div for VecF {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}
impl std::ops::AddAssign for VecF {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl std::ops::SubAssign for VecF {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
impl std::ops::MulAssign for VecF {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
    }
}
impl std::ops::DivAssign for VecF {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
    }
}
