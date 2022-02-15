use crate::vector::Vector;
use numeric::Float;
use std::ops::Neg;

impl<F: Float> Neg for Vector<F>
where
    F: Neg + Copy + Clone,
{
    type Output = Vector<F>;

    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<F: Float> Neg for &Vector<F>
where
    F: Neg + Copy + Clone,
{
    type Output = Vector<F>;

    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<F: Float> Neg for &mut Vector<F>
where
    F: Neg + Copy + Clone,
{
    type Output = Vector<F>;

    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
