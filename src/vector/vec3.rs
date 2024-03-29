use num::Num;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

/// A generic 3D Vector implementation.
/// Takes 3 generic numbers (all 3 must be same type).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Vec3<T>
where
    T: Num + Clone + Copy,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T>
where
    T: Num + Clone + Copy,
{
    /// Create a 3D Vector with the given XYZ values
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Return the dot product of two 3D Vectors
    ///
    /// `a.x * b.x + a.y * b.y + a.z * b.z`
    pub fn dot(&self, rhs: &Vec3<T>) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    // TODO: Explaination comment?
    /// Return the cross product of two 3D Vectors
    pub fn cross(&self, rhs: &Vec3<T>) -> Vec3<T> {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    // TODO: Swizzle?
}

impl Vec3<f32> {
    /// Create a 3D vector with all values initialised to 0.0
    pub fn zero() -> Vec3<f32> {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

/// Allows for the following syntax:
/// ```rust
/// # use lamar::vector::Vec3;
/// let a = Vec3::new(2, 4, 8);
/// let b = Vec3::new(16, 32, 64);
/// let c = a + b;
///
/// assert_eq!(c, Vec3::new(a.x + b.x, a.y + b.y, a.z + b.z));
/// ```
impl<T> Add for Vec3<T>
where
    T: Num + Clone + Copy,
{
    type Output = Vec3<T>;

    fn add(self, other: Vec3<T>) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

/// Allows for the following syntax:
/// ```rust
/// # use lamar::vector::Vec3;
/// let a = Vec3::new(4, 7, 8);
/// let b = 2;
/// let c = a + b;
///
/// assert_eq!(c, Vec3::new(4 + 2, 7 + 2, 8 + 2));
/// ```
impl<T> Add<T> for Vec3<T>
where
    T: Num + Clone + Copy,
{
    type Output = Vec3<T>;

    fn add(self, other: T) -> Self::Output {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

/// Allows for the following syntax:
/// ```rust
/// # use lamar::vector::Vec3;
/// let a = Vec3::new(2, 4, 8);
/// let b = Vec3::new(16, 32, 64);
/// let c = a - b;
///
/// assert_eq!(c, Vec3::new(a.x - b.x, a.y - b.y, a.z - b.z));
/// ```
impl<T> Sub for Vec3<T>
where
    T: Num + Clone + Copy,
{
    type Output = Vec3<T>;

    fn sub(self, other: Vec3<T>) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/// Allows for the following syntax:
/// ```rust
/// # use lamar::vector::Vec3;
/// let a = Vec3::new(4, 7, 8);
/// let b = 2;
/// let c = a - b;
///
/// assert_eq!(c, Vec3::new(4 - 2, 7 - 2, 8 - 2));
/// ```
impl<T> Sub<T> for Vec3<T>
where
    T: Num + Clone + Copy,
{
    type Output = Vec3<T>;

    fn sub(self, other: T) -> Self::Output {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

/// Allows for the following syntax:
/// ```rust
/// # use lamar::vector::Vec3;
/// let a = Vec3::new(2, 4, 8);
/// let b = Vec3::new(16, 32, 64);
/// let c = a * b;
///
/// assert_eq!(c, a.cross(&b));
/// ```
impl<T> Mul for Vec3<T>
where
    T: Num + Clone + Copy,
{
    type Output = Vec3<T>;

    fn mul(self, other: Vec3<T>) -> Self::Output {
        self.cross(&other)
    }
}

/// Allows for the following syntax:
/// ```rust
/// # use lamar::vector::Vec3;
/// let a = Vec3::new(4, 7, 8);
/// let b = 2;
/// let c = a * b;
///
/// assert_eq!(c, Vec3::new(4 * 2, 7 * 2, 8 * 2));
/// ```
impl<T> Mul<T> for Vec3<T>
where
    T: Num + Clone + Copy,
{
    type Output = Vec3<T>;

    fn mul(self, other: T) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

/// Allows for the following syntax:
/// ```rust
/// # use lamar::vector::Vec3;
/// let a = Vec3::new(4, 6, 12);
/// let b = 2;
/// let c = a / b;
///
/// assert_eq!(c, Vec3::new(4 / 2, 6 / 2, 12 / 2));
/// ```
impl<T> Div<T> for Vec3<T>
where
    T: Num + Clone + Copy,
{
    type Output = Vec3<T>;

    fn div(self, other: T) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl<T> Display for Vec3<T>
where
    T: Display + Num + Clone + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}\ny: {}\nz: {}", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod test {
    use crate::vector::Vec3;

    #[test]
    fn zero_vec3_test() {
        assert_eq!(Vec3::zero(), Vec3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn dot_product_test() {
        let lhs = Vec3::new(2, 4, 8);
        let rhs = Vec3::new(16, 32, 64);

        assert_eq!(lhs.dot(&rhs), 672);
    }

    #[test]
    fn cross_product_test() {
        let lhs = Vec3::new(5, 10, 15);
        let rhs = Vec3::new(3, 1, 7);

        assert_eq!(lhs.cross(&rhs), Vec3::new(55, 10, -25));
    }

    #[test]
    fn cross_product_mul_trait_test() {
        let lhs = Vec3::new(5, 10, 15);
        let rhs = Vec3::new(3, 1, 7);

        assert_eq!(lhs * rhs, Vec3::new(55, 10, -25));
    }

    #[test]
    fn add_test() {
        let lhs = Vec3::new(2, 4, 8);
        let rhs = Vec3::new(16, 32, 64);

        assert_eq!(lhs + rhs, Vec3::new(18, 36, 72));
    }

    #[test]
    fn sub_test() {
        let lhs = Vec3::new(2, 4, 8);
        let rhs = Vec3::new(16, 32, 64);

        assert_eq!(lhs - rhs, Vec3::new(-14, -28, -56));
    }

    #[test]
    fn add_scalar_test() {
        let lhs = Vec3::new(32, 64, 128);
        let rhs = 10;

        assert_eq!(lhs + rhs, Vec3::new(42, 74, 138));
    }

    #[test]
    fn sub_scalar_test() {
        let lhs = Vec3::new(32, 64, 128);
        let rhs = 10;

        assert_eq!(lhs - rhs, Vec3::new(22, 54, 118));
    }

    #[test]
    fn mul_scalar_test() {
        let lhs = Vec3::new(32, 64, 128);
        let rhs = 10;

        assert_eq!(lhs * rhs, Vec3::new(320, 640, 1280));
    }

    #[test]
    fn div_scalar_test() {
        let lhs = Vec3::new(32, 64, 128);
        let rhs = 4;

        assert_eq!(lhs / rhs, Vec3::new(8, 16, 32));
    }
}
