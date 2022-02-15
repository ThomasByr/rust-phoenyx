use crate::vector::Vector;
use numeric::Float;
use std::ops::{Add, AddAssign};

// --- Add ---

// vector

impl<F: Float> Add<Vector<F>> for Vector<F>
where
    F: Add<F, Output = F> + Copy + Clone,
{
    type Output = Vector<F>;

    fn add(self, rhs: Vector<F>) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<F: Float> Add<F> for Vector<F>
where
    F: Add<F, Output = F> + Copy + Clone,
{
    type Output = Vector<F>;

    fn add(self, rhs: F) -> Self::Output {
        Vector {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

// &vector

impl<F: Float> Add<&Vector<F>> for &Vector<F>
where
    F: Add<F, Output = F> + Copy + Clone,
{
    type Output = Vector<F>;

    fn add(self, rhs: &Vector<F>) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<F: Float> Add<F> for &Vector<F>
where
    F: Add<F, Output = F> + Copy + Clone,
{
    type Output = Vector<F>;

    fn add(self, rhs: F) -> Self::Output {
        Vector {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

// &mut vector

impl<F: Float> Add<&mut Vector<F>> for &mut Vector<F>
where
    F: Add<F, Output = F> + Copy + Clone,
{
    type Output = Vector<F>;

    fn add(self, rhs: &mut Vector<F>) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<F: Float> Add<F> for &mut Vector<F>
where
    F: Add<F, Output = F> + Copy + Clone,
{
    type Output = Vector<F>;

    fn add(self, rhs: F) -> Self::Output {
        Vector {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

// --- AddAssign ---

// vector

impl<F: Float> AddAssign<Vector<F>> for Vector<F>
where
    F: AddAssign<F> + Copy + Clone,
{
    fn add_assign(&mut self, rhs: Vector<F>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<F: Float> AddAssign<F> for Vector<F>
where
    F: AddAssign<F> + Copy + Clone,
{
    fn add_assign(&mut self, rhs: F) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

// &vector

impl<F: Float> AddAssign<&Vector<F>> for Vector<F>
where
    F: AddAssign<F> + Copy + Clone,
{
    fn add_assign(&mut self, rhs: &Vector<F>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

// &mut vector

impl<F: Float> AddAssign<&mut Vector<F>> for Vector<F>
where
    F: AddAssign<F> + Copy + Clone,
{
    fn add_assign(&mut self, rhs: &mut Vector<F>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
