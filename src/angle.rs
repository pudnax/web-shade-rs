#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Rad(pub f32);

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Deg(pub f32);

impl From<Rad> for Deg {
    fn from(x: Rad) -> Self {
        Self(180.0 * x.0 / std::f32::consts::PI)
    }
}

impl From<Deg> for Rad {
    fn from(x: Deg) -> Self {
        Self(std::f32::consts::PI * x.0 / 180.0)
    }
}

impl From<f32> for Rad {
    fn from(x: f32) -> Self {
        Self(x)
    }
}

impl From<f32> for Deg {
    fn from(x: f32) -> Self {
        Self(x)
    }
}

impl From<Rad> for f32 {
    fn from(x: Rad) -> Self {
        x.0
    }
}

impl From<Deg> for f32 {
    fn from(x: Deg) -> Self {
        x.0
    }
}

impl std::ops::Neg for Rad {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl std::ops::Neg for Deg {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl std::ops::Mul<f32> for Rad {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl std::ops::Mul<f32> for Deg {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl std::ops::Add<f32> for Rad {
    type Output = Self;
    fn add(self, rhs: f32) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl std::ops::Add<f32> for Deg {
    type Output = Self;
    fn add(self, rhs: f32) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl std::ops::Sub<f32> for Rad {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl std::ops::Sub<f32> for Deg {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl std::ops::AddAssign<Rad> for Rad {
    fn add_assign(&mut self, rhs: Rad) {
        self.0 = self.0 + rhs.0;
    }
}
