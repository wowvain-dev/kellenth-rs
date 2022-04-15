#[allow(unused, dead_code)]
use std::ops;

/// Three-dimensional vector used to describe position, movement, direction, etc. in space.
#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    /// Length along the x coordinate
    pub x: f64,

    /// Length along the y coordinate
    pub y: f64,

    /// Length along the z coordinate
    pub z: f64
}

/// Implement the `Display` trait for Vector3.
/// A vector will be described by the length across each axis, its magnitude and its direction.
impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "value: [x = {}, y = {}, z = {}]; magnitude = {}; direction = [x = {}, y = {}, z = {}]",
            self.x,
            self.y,
            self.z,
            self.magnitude(),
            self.get_normalized().x,
            self.get_normalized().y,
            self.get_normalized().z
        )
    }
}

/// Operator overloads for scalar operations
impl ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, _rhs: f64) -> Vector3 {
        Vector3 {
            x: self.x * &_rhs,
            y: self.y * &_rhs,
            z: self.z * &_rhs,
        }
    }
}
impl ops::Mul<i64> for Vector3 {
    type Output = Vector3;

    fn mul(self, _rhs: i64) -> Vector3 {
        Vector3 {
            x: self.x * &(_rhs as f64),
            y: self.y * &(_rhs as f64),
            z: self.z * &(_rhs as f64),
        }
    }
}
impl ops::MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= &rhs;
        self.y *= &rhs;
        self.z *= &rhs;
    }
}
impl ops::MulAssign<i64> for Vector3 {
    fn mul_assign(&mut self, rhs: i64) {
        self.x *= &(rhs as f64);
        self.y *= &(rhs as f64);
        self.z *= &(rhs as f64);
    }
}

/// Operator overloads for vector operations
/// Operator `%` will be used for the cross-product
impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + &rhs.x,
            y: self.y + &rhs.y,
            z: self.z + &rhs.z,
        }
    }
}
impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - &rhs.x,
            y: self.y - &rhs.y,
            z: self.z - &rhs.z,
        }
    }
}
impl ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self.x += &rhs.x;
        self.y += &rhs.y;
        self.z += &rhs.z;
    }
}
impl ops::SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        self.x -= &rhs.x;
        self.y -= &rhs.y;
        self.z -= &rhs.z;
    }
}
impl ops::Mul<Vector3> for Vector3 {
    type Output = f64;

    fn mul(self, rhs: Vector3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
impl ops::Rem<Vector3> for Vector3 {
    type Output = Vector3;

    fn rem(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.y*rhs.z - self.z*rhs.y,
            y: self.z*rhs.x - self.x*rhs.z,
            z: self.x*rhs.y - self.y*rhs.x,
        }
    }
}
impl ops::RemAssign<Vector3> for Vector3 {
    fn rem_assign(&mut self, rhs: Vector3) {
        self.x = self.y * rhs.z - self.z * rhs.y;
        self.y = self.z * rhs.x - self.x - rhs.z;
        self.z = self.x * rhs.y - self.y * rhs.x;
    }
}

impl Vector3 {
    /// Adds a vector scaled by a scalar to the current vector.
    pub fn add_scaled_vector(&mut self, vector: Vector3, scalar: f64) {
        self.x += vector.x * &scalar;
        self.y += vector.y * &scalar;
        self.z += vector.z * &scalar;
    }

    /// Returns the component product of this vector and a given one.
    pub fn component_product(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    /// Returns the vector product of this vector and the given one.
    /// ## Equivalent to the `%` operator when used between two vectors.
    pub fn vector_product(self, vector: Vector3) -> Vector3 {
        Vector3 {x: self.y * vector.z - self.z * vector.y,
                 y: self.z * vector.x - self.x * vector.z,
                 z: self.x * vector.y - self.y * vector.x}
    }

    /// Returns the scalar product of this vector and the given one.
    /// ## Equivalent to the `*` operator when used between two vectors.
    pub fn scalar_product(self, vector: Vector3) -> f64 {
        self.x * vector.x + self.y * vector.y + self.z * vector.z
    }

    /// Inverts the vector along each axis.
    pub fn invert(&mut self) {
        self.x *= -1 as f64;
        self.y *= -1 as f64;
        self.z *= -1 as f64;
    }

    /// Returns the magnitude of the vector.
    pub fn magnitude(&self) -> f64 {
        f64::sqrt(&self.x * &self.x + &self.y * &self.y + &self.z * &self.z)
    }

    /// Normalizes the vector, making it a unit-length vector.
    pub fn normalize(&mut self) {
        let l = self.magnitude();
        if l > 0 as f64 {
            *self *= 1. / l;
        }
    }

    /// Returns the normalized version of the vector. (returns the distance).
    pub fn get_normalized(self) -> Vector3 {
        let l = self.magnitude();
        let mut dist = self;
        if l > 0 as f64 {
            dist *= 1. / l;
        }
        dist
    }
}
