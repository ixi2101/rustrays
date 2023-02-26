#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// impl basic ops
impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

macro_rules! vecAdd {
    ($x:ty) => {
        impl std::ops::Add<$x> for Vec3 {
            type Output = Self;
            fn add(self, rhs: $x) -> Self {
                Self {
                    x: self.x + rhs as f32,
                    y: self.y + rhs as f32,
                    z: self.z + rhs as f32,
                }
            }
        }
    };
}

vecAdd!(i32);
vecAdd!(f32);

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
macro_rules! vecSub {
    ($x:ty) => {
        impl std::ops::Sub<$x> for Vec3 {
            type Output = Self;
            fn sub(self, rhs: $x) -> Self {
                Self {
                    x: self.x - rhs as f32,
                    y: self.y - rhs as f32,
                    z: self.z - rhs as f32,
                }
            }
        }
    };
}

vecSub!(i32);
vecSub!(f32);

impl std::ops::Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}
macro_rules! vecDiv {
    ($x:ty) => {
        impl std::ops::Div<$x> for Vec3 {
            type Output = Self;
            fn div(self, rhs: $x) -> Self {
                Self {
                    x: self.x / rhs as f32,
                    y: self.y / rhs as f32,
                    z: self.z / rhs as f32,
                }
            }
        }
    };
}

vecDiv!(i32);
vecDiv!(f32);

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

macro_rules! vecMult {
    ($x:ty) => {
        impl std::ops::Mul<$x> for Vec3 {
            type Output = Self;
            fn mul(self, rhs: $x) -> Self {
                Self {
                    x: self.x * rhs as f32,
                    y: self.y * rhs as f32,
                    z: self.z * rhs as f32,
                }
            }
        }
    };
}
vecMult!(i32);
vecMult!(f32);

impl Vec3 {
    pub fn length(&self) -> f32 {
        (self.length_squared() as f32).sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_len_sq() {
        let a = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        assert_eq!(a.length_squared(), 3_f32);
    }

    #[test]
    fn test_unit_len() {
        let a = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        assert_eq!(a.length(), 3_f32.sqrt());
    }

    #[test]
    fn test_len_sq() {
        let a = Vec3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        assert_eq!(a.length_squared(), 50.0);
    }

    #[test]
    fn test_len() {
        let a = Vec3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        assert_eq!(a.length(), 50_f32.sqrt());
    }

    #[test]
    fn test_scalar_addition() {
        let a = Vec3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let b = 10;
        let c = a + b;
        assert_eq!(c.x, 13.0);
        assert_eq!(c.y, 14.0);
        assert_eq!(c.z, 15.0);
    }

    #[test]
    fn test_vec_addition() {
        let a = Vec3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let b = Vec3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let c = a + b;
        assert_eq!(c.x, 6.0);
        assert_eq!(c.y, 8.0);
        assert_eq!(c.z, 10.0);
    }

    #[test]
    fn test_scalar_sub() {
        let a = Vec3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let b = 10;
        let c = a - b;
        assert_eq!(c.x, -7.0);
        assert_eq!(c.y, -6.0);
        assert_eq!(c.z, -5.0);
    }

    #[test]
    fn test_vec_sub() {
        let a = Vec3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let b = Vec3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let c = a - b;
        assert_eq!(c.x, 0.0);
        assert_eq!(c.y, 0.0);
        assert_eq!(c.z, 0.0);
    }

    #[test]
    fn test_scalar_mult() {
        let a = Vec3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let b = 10;
        let c = a * b;
        assert_eq!(c.x, 30.0);
        assert_eq!(c.y, 40.0);
        assert_eq!(c.z, 50.0);
    }

    #[test]
    fn test_vec_mult() {
        let a = Vec3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let b = Vec3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let c = a * b;
        assert_eq!(c.x, 9.0);
        assert_eq!(c.y, 16.0);
        assert_eq!(c.z, 25.0);
    }

    #[test]
    fn test_scalar_div() {
        let a = Vec3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let b = 10;
        let c = a / b;
        assert_eq!(c.x, 0.3);
        assert_eq!(c.y, 0.4);
        assert_eq!(c.z, 0.5);
    }

    #[test]
    fn test_vec_div() {
        let a = Vec3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let b = Vec3 {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };
        let c = a / b;
        assert_eq!(c.x, 1.0);
        assert_eq!(c.y, 1.0);
        assert_eq!(c.z, 1.0);
    }
}
