use numeric::{Float, One, Zero};
use std::ops::MulAssign;

/// A 3D vector, containing an `x`, `y` and a `z` floating point value.
///
/// A Vector -- specifically an Euclidean (or geometric) vector -- in
/// two or three dimensional space is a geometric entity that has some
/// magnitude (or length) and a direction.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Vector<F: Float> {
    pub x: F,
    pub y: F,
    pub z: F,
}

impl<F: Copy + Clone + Float> Vector<F> {
    /// Create a new vector.
    pub fn new(x: F, y: F, z: F) -> Self {
        Self { x, y, z }
    }

    pub fn from_vec<U: Into<U> + Copy + Clone + Float>(src: Vector<U>) -> Vector<U> {
        Vector {
            x: src.x.into(),
            y: src.y.into(),
            z: src.z.into(),
        }
    }

    pub fn into_vec<U: From<U> + From<F> + Float>(self) -> Vector<U> {
        Vector {
            x: self.x.into(),
            y: self.y.into(),
            z: self.z.into(),
        }
    }
}

impl<F: Float> Into<(F, F, F)> for Vector<F> {
    /// Performs the conversion.
    fn into(self) -> (F, F, F) {
        (self.x, self.y, self.z)
    }
}

impl<F: Float> From<(F, F, F)> for Vector<F> {
    /// Performs the conversion.
    fn from(src: (F, F, F)) -> Self {
        Self {
            x: src.0,
            y: src.1,
            z: src.2,
        }
    }
}

impl<F: Float> Into<[F; 3]> for Vector<F> {
    /// Performs the conversion.
    fn into(self) -> [F; 3] {
        [self.x, self.y, self.z]
    }
}

impl<F: Float> From<[F; 3]> for Vector<F> {
    /// Performs the conversion.
    fn from(src: [F; 3]) -> Self {
        Self {
            x: src[0],
            y: src[1],
            z: src[2],
        }
    }
}

impl<F: Default + Zero + One + Float> Vector<F> {
    /// Default construct a `Vector` with all components set to 0.
    ///
    /// # Example
    /// ```
    /// use math_vector::Vector;
    /// let v = Vector::default();
    /// assert_eq!(Vector::new(0, 0, 0), v);
    /// ```
    pub fn default() -> Self {
        Self {
            x: F::default(),
            y: F::default(),
            z: F::default(),
        }
    }

    /// Construct a `Vector` with all components set to 1.
    pub fn one() -> Self {
        Self {
            x: F::one(),
            y: F::one(),
            z: F::one(),
        }
    }

    /// Construct a `Vector` with all components set to 0.
    pub fn zero() -> Self {
        Self {
            x: F::zero(),
            y: F::zero(),
            z: F::zero(),
        }
    }

    /// Reset all components to 0.
    pub fn reset(&mut self) {
        self.x = F::zero();
        self.y = F::zero();
        self.z = F::zero();
    }

    /// Returns a vector with only the horizontal component of the current one
    ///
    /// # Example
    /// ```
    /// use math_vector::Vector;
    /// let v = Vector::new(10.0, 20.0, 30.0);
    /// assert_eq!(Vector::new(10.0, 0.0, 0.0), v.abscissa());
    /// ```
    pub fn abscissa(self) -> Self {
        Self {
            x: self.x,
            y: F::zero(),
            z: F::zero(),
        }
    }

    /// Returns a vector with only the vertical component of the current one
    ///
    /// # Example
    /// ```
    /// use math_vector::Vector;
    /// let v = Vector::new(10.0, 20.0, 30.0);
    /// assert_eq!(Vector::new(0.0, 20.0, 0.0), v.ordinate());
    /// ```
    pub fn ordinate(self) -> Self {
        Self {
            x: F::zero(),
            y: self.y,
            z: F::zero(),
        }
    }

    /// Returns a vector with only the depth component of the current one
    ///
    /// # Example
    /// ```
    /// use math_vector::Vector;
    /// let v = Vector::new(10.0, 20.0, 30.0);
    /// assert_eq!(Vector::new(0.0, 0.0, 30.0), v.applicate());
    /// ```
    pub fn applicate(self) -> Self {
        Self {
            x: F::zero(),
            y: F::zero(),
            z: self.z,
        }
    }

    /// Create a new unit vector from its polar coordinates.
    /// Use `from_angle` to create a unit vector from an angle in the 2d plane.
    pub fn from_polar(theta: F, phi: F) -> Self {
        Self {
            x: theta.cos() * phi.cos(),
            y: theta.sin() * phi.cos(),
            z: phi.sin(),
        }
    }

    /// Create a new unit vector from its angle in the 2d plane.
    /// Use `from_polar` to create a unit vector from its polar coordinates.
    pub fn from_angle(theta: F) -> Self {
        Self {
            x: theta.cos(),
            y: theta.sin(),
            z: F::zero(),
        }
    }
}

impl<F: Copy + Clone + Float> Vector<F> {
    /// Linearly interpolate between two vectors.
    ///
    /// # Example
    /// ```
    /// use math_vector::Vector;
    /// let v1 = Vector::new(1.0, 2.0, 3.0);
    /// let v2 = Vector::new(2.0, 3.0, 4.0);
    /// assert_eq!(Vector::new(1.5, 2.5, 3.5), v1.lerp(v2, 0.5));
    /// ```
    pub fn lerp(self, other: Self, t: F) -> Self {
        self + (other - self) * t
    }
}

impl<F: Float> Vector<F> {
    /// Return the dot product of two vectors.
    ///
    /// # Example
    /// ```
    /// use math_vector::Vector;
    /// let x = Vector::new(1.0, 0.0, 0.0);
    /// let y = Vector::new(0.0, 1.0, 0.0);
    /// assert_eq!(x.dot(y), 1.0);
    /// ```
    pub fn dot(self, other: Vector<F>) -> F {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Return the cross product of two vectors.
    ///
    /// # Example
    /// ```
    /// use math_vector::Vector;
    /// let x = Vector::new(1.0, 0.0, 0.0);
    /// let y = Vector::new(0.0, 1.0, 0.0);
    /// let z = Vector::new(0.0, 0.0, 1.0);
    /// assert_eq!(x.cross(y), z);
    /// ```
    pub fn cross(self, other: Vector<F>) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl<F: Float> Vector<F> {
    /// Return the squared length of the vector.
    /// This is faster than `length` because it does not need to compute the square root.
    pub fn length_squared(self) -> F {
        self.dot(self)
    }

    /// Return the length of the vector.
    /// Prefer `length_squared` if you need to avoid computing the square root.
    pub fn length(self) -> F {
        self.length_squared().sqrt()
    }

    /// Return the norm 1 of the difference between two vectors.
    pub fn distance_squared(self, other: Self) -> F {
        (self - other).length_squared()
    }

    /// Return the euclidean distance between two vectors.
    pub fn distance(self, other: Self) -> F {
        (self - other).length()
    }

    /// Return a new vector which is the reflection of the current one
    /// on a surface with the given local normal.
    pub fn reflect(self, normal: Self) -> Self {
        self - normal * self.dot(normal) * F::from(2.0).unwrap()
    }

    /// Compare two vectors for equality.
    /// The comparison is done with the relative error `epsilon`
    /// so that the error is relative to the magnitude of both vectors.
    ///
    /// # Example
    /// ```
    /// use math_vector::Vector;
    /// let x = Vector::new(1.0, 0.0, 0.0);
    /// let y = Vector::new(1.0000000001, 0.0, 0.0);
    /// assert_eq!(y.is_close(x), true);
    ///
    /// let x = Vector::new(1000000.0, 0.0, 0.0);
    /// let y = Vector::new(1000000000.1, 0.0, 0.0);
    /// assert_eq!(y.is_close(x), true);
    ///
    /// let x = Vector::new(1.0, 0.0, 0.0);
    /// let y = Vector::new(1.0001, 0.0, 0.0);
    /// assert_eq!(y.is_close(x), false);
    /// ```
    pub fn is_close(self, reference: Vector<F>) -> bool {
        let num = (self - reference).length_squared();
        let den = reference.length_squared();

        num <= den * Float::epsilon()
    }
}

impl<F: Float + MulAssign + From<f64> + Into<f64>> Vector<F> {
    /// Set the length of the vector.
    /// The scaling factor is computed with the help of the fast inverse square root.
    pub fn set_length(&mut self, length: F) {
        let len_sq = self.length_squared();
        if len_sq != F::zero() {
            let factor = length * len_sq.sqrt().recip();
            self.x *= factor;
            self.y *= factor;
            self.z *= factor;
        }
    }

    /// Set the squared length of the vector.
    pub fn set_length_squared(&mut self, length_squared: F) {
        let len_sq = self.length_squared();
        if len_sq != F::zero() {
            let factor = length_squared / len_sq;
            self.x *= factor;
            self.y *= factor;
            self.z *= factor;
        }
    }

    /// Return a new vector whose length is set to `length`.
    /// The scaling factor is computed with the help of the fast inverse square root.
    pub fn with_length(self, length: F) -> Self {
        let mut v = self;
        v.set_length(length);
        v
    }

    /// Return a new vector whose squared length is set to `length_squared`.
    pub fn with_length_squared(self, length_squared: F) -> Self {
        let mut v = self;
        v.set_length_squared(length_squared);
        v
    }

    /// Limit the length of the vector to a maximum length.
    pub fn limit(&mut self, max_length: F) {
        let len_sq = self.length_squared();
        if len_sq > max_length * max_length {
            let factor = max_length * len_sq.sqrt().recip();
            self.x *= factor;
            self.y *= factor;
            self.z *= factor;
        }
    }

    /// Return a limited version of the vector.
    pub fn limited(self, max_length: F) -> Self {
        let mut v = self;
        v.limit(max_length);
        v
    }

    /// Normalize the vector.
    /// Does nothing if the vector is of length zero.
    pub fn normalize(&mut self) {
        self.set_length(F::one());
    }

    /// Return a normalized copy of the vector.
    /// Does nothing if the vector is of length zero.
    pub fn normalized(self) -> Self {
        let mut v = self;
        v.normalize();
        v
    }
}

impl<F: Float> Vector<F> {
    /// Return the angle in the 2d plane between the positive x axis and the vector.
    pub fn heading2d(self) -> F {
        self.y.atan2(self.x)
    }

    /// Return a couple of angles in the 3d space.
    pub fn heading3d(self) -> (F, F) {
        let theta = self.y.atan2(self.x);
        let phi = (self.z / self.length()).asin();
        (theta, phi)
    }

    /// Return the angle in radians between two vectors.
    pub fn angle_between(self, other: Vector<F>) -> F {
        let dot = self.dot(other);
        let det = self.cross(other).length();
        F::atan2(det, dot)
    }

    /// Return a new vector rotated around some axis by a given angle in radians.
    /// Please note that the axis is assumed to be normalized.
    ///
    /// # Example
    /// ```
    /// use math_vector::Vector;
    /// let v = Vector::new(1.0, 0.0, 0.0);
    /// let axis = Vector::new(0.0, 0.0, 1.0);
    /// let angle = std::f64::consts::PI / 2.0;
    /// let rotated = v.rotated(axis, angle);
    /// assert_eq!(rotated.is_close(Vector::new(0.0, 1.0, 0.0)), true);
    /// ```
    pub fn rotated(self, angle: F, axis: Vector<F>) -> Vector<F> {
        let (sin, cos) = angle.sin_cos();
        self * cos + axis.cross(self) * sin + axis * axis.dot(self) * (F::one() - cos)
    }

    /// Rotate the vector around some axis by a given angle in radians.
    /// Please note that the axis is assumed to be normalized.
    ///
    /// # Example
    /// ```
    /// use math_vector::Vector;
    /// let mut v = Vector::new(1.0, 0.0, 0.0);
    /// let axis = Vector::new(0.0, 0.0, 1.0);
    /// let angle = std::f64::consts::PI / 2.0;
    /// v.rotate(axis, angle);
    /// assert_eq!(v.is_close(Vector::new(0.0, 1.0, 0.0)), true);
    /// ```
    pub fn rotate(&mut self, angle: F, axis: Vector<F>) {
        let tmp = self.rotated(angle, axis);
        *self = tmp;
    }
}

impl<F: Float + Default + Zero + One> Vector<F> {
    /// Rotate the vector around the x axis by a given angle in radians.
    /// This is equivalent to `rotate(angle, Vector::new(1.0, 0.0, 0.0))`.
    pub fn rotate_x(&mut self, angle: F) {
        self.rotate(angle, Vector::one().abscissa());
    }

    /// Rotate the vector around the y axis by a given angle in radians.
    /// This is equivalent to `rotate(angle, Vector::new(0.0, 1.0, 0.0))`.
    pub fn rotate_y(&mut self, angle: F) {
        self.rotate(angle, Vector::one().ordinate());
    }

    /// Rotate the vector around the z axis by a given angle in radians.
    /// This is equivalent to `rotate(angle, Vector::new(0.0, 0.0, 1.0))`.
    pub fn rotate_z(&mut self, angle: F) {
        self.rotate(angle, Vector::one().applicate());
    }
}
