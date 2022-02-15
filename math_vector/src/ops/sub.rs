use crate::vector::Vector;
use numeric::Float;
use std::ops::{Sub, SubAssign};

// --- Sub ---

// vector

impl<F: Float> Sub<Vector<F>> for Vector<F>
where
    F: Sub<F, Output = F> + Copy + Clone,
{
    type Output = Vector<F>;

    fn sub(self, rhs: Vector<F>) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<F: Float> Sub<F> for Vector<F>
where
    F: Sub<F, Output = F> + Copy + Clone,
{
    type Output = Vector<F>;

    fn sub(self, rhs: F) -> Self::Output {
        Vector {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

// &vector

impl<F: Float> Sub<&Vector<F>> for &Vector<F>
where
    F: Sub<F, Output = F> + Copy + Clone,
{
    type Output = Vector<F>;

    fn sub(self, rhs: &Vector<F>) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<F: Float> Sub<F> for &Vector<F>
where
    F: Sub<F, Output = F> + Copy + Clone,
{
    type Output = Vector<F>;

    fn sub(self, rhs: F) -> Self::Output {
        Vector {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

// &mut vector

impl<F: Float> Sub<&mut Vector<F>> for &mut Vector<F>
where
    F: Sub<F, Output = F> + Copy + Clone,
{
    type Output = Vector<F>;

    fn sub(self, rhs: &mut Vector<F>) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<F: Float> Sub<F> for &mut Vector<F>
where
    F: Sub<F, Output = F> + Copy + Clone,
{
    type Output = Vector<F>;

    fn sub(self, rhs: F) -> Self::Output {
        Vector {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

// --- SubAssign ---

// vector

impl<F: Float> SubAssign<Vector<F>> for Vector<F>
where
    F: SubAssign<F> + Copy + Clone,
{
    fn sub_assign(&mut self, rhs: Vector<F>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<F: Float> SubAssign<F> for Vector<F>
where
    F: SubAssign<F> + Copy + Clone,
{
    fn sub_assign(&mut self, rhs: F) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

// &vector

impl<F: Float> SubAssign<&Vector<F>> for Vector<F>
where
    F: SubAssign<F> + Copy + Clone,
{
    fn sub_assign(&mut self, rhs: &Vector<F>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

// &mut vector

impl<F: Float> SubAssign<&mut Vector<F>> for Vector<F>
where
    F: SubAssign<F> + Copy + Clone,
{
    fn sub_assign(&mut self, rhs: &mut Vector<F>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
