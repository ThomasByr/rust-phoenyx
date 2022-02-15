use crate::vector::Vector;
use numeric::Float;
use std::ops::{Div, DivAssign};

// --- Div ---

// vector

impl<F: Float> Div<Vector<F>> for Vector<F>
where
    F: Div<F, Output = F> + Copy + Clone,
{
    type Output = Vector<F>;

    fn div(self, rhs: Vector<F>) -> Self::Output {
        Vector {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<F: Float> Div<F> for Vector<F>
where
    F: Div<F, Output = F> + Copy + Clone,
{
    type Output = Vector<F>;

    fn div(self, rhs: F) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

// &vector

impl<F: Float> Div<&Vector<F>> for &Vector<F>
where
    F: Div<F, Output = F> + Copy + Clone,
{
    type Output = Vector<F>;

    fn div(self, rhs: &Vector<F>) -> Self::Output {
        Vector {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<F: Float> Div<F> for &Vector<F>
where
    F: Div<F, Output = F> + Copy + Clone,
{
    type Output = Vector<F>;

    fn div(self, rhs: F) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

// &mut vector

impl<F: Float> Div<&mut Vector<F>> for &mut Vector<F>
where
    F: Div<F, Output = F> + Copy + Clone,
{
    type Output = Vector<F>;

    fn div(self, rhs: &mut Vector<F>) -> Self::Output {
        Vector {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<F: Float> Div<F> for &mut Vector<F>
where
    F: Div<F, Output = F> + Copy + Clone,
{
    type Output = Vector<F>;

    fn div(self, rhs: F) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

// --- DivAssign ---

// vector

impl<F: Float> DivAssign<Vector<F>> for Vector<F>
where
    F: DivAssign<F> + Copy + Clone,
{
    fn div_assign(&mut self, rhs: Vector<F>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl<F: Float> DivAssign<F> for Vector<F>
where
    F: DivAssign<F> + Copy + Clone,
{
    fn div_assign(&mut self, rhs: F) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

// &vector

impl<F: Float> DivAssign<&Vector<F>> for Vector<F>
where
    F: DivAssign<F> + Copy + Clone,
{
    fn div_assign(&mut self, rhs: &Vector<F>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

// &mut vector

impl<F: Float> DivAssign<&mut Vector<F>> for Vector<F>
where
    F: DivAssign<F> + Copy + Clone,
{
    fn div_assign(&mut self, rhs: &mut Vector<F>) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}
