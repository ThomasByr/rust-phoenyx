use crate::vector::Vector;
use numeric::Float;
use std::ops::{Mul, MulAssign};

// --- Mul ---

// vector

impl<F: Float> Mul<Vector<F>> for Vector<F>
where
    F: Mul<F, Output = F> + Copy + Clone,
{
    type Output = Vector<F>;

    fn mul(self, rhs: Vector<F>) -> Self::Output {
        Vector {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<F: Float> Mul<F> for Vector<F>
where
    F: Mul<F, Output = F> + Copy + Clone,
{
    type Output = Vector<F>;

    fn mul(self, rhs: F) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

// &vector

impl<F: Float> Mul<&Vector<F>> for &Vector<F>
where
    F: Mul<F, Output = F> + Copy + Clone,
{
    type Output = Vector<F>;

    fn mul(self, rhs: &Vector<F>) -> Self::Output {
        Vector {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<F: Float> Mul<F> for &Vector<F>
where
    F: Mul<F, Output = F> + Copy + Clone,
{
    type Output = Vector<F>;

    fn mul(self, rhs: F) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

// &mut vector

impl<F: Float> Mul<&mut Vector<F>> for &mut Vector<F>
where
    F: Mul<F, Output = F> + Copy + Clone,
{
    type Output = Vector<F>;

    fn mul(self, rhs: &mut Vector<F>) -> Self::Output {
        Vector {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<F: Float> Mul<F> for &mut Vector<F>
where
    F: Mul<F, Output = F> + Copy + Clone,
{
    type Output = Vector<F>;

    fn mul(self, rhs: F) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

// --- MulAssign ---

// vector

impl<F: Float> MulAssign<Vector<F>> for Vector<F>
where
    F: MulAssign<F> + Copy + Clone,
{
    fn mul_assign(&mut self, rhs: Vector<F>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl<F: Float> MulAssign<F> for Vector<F>
where
    F: MulAssign<F> + Copy + Clone,
{
    fn mul_assign(&mut self, rhs: F) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

// &vector

impl<F: Float> MulAssign<&Vector<F>> for Vector<F>
where
    F: MulAssign<F> + Copy + Clone,
{
    fn mul_assign(&mut self, rhs: &Vector<F>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

// &mut vector

impl<F: Float> MulAssign<&mut Vector<F>> for Vector<F>
where
    F: MulAssign<F> + Copy + Clone,
{
    fn mul_assign(&mut self, rhs: &mut Vector<F>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}
