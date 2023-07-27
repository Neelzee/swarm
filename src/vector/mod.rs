use std::{ops::Sub, ops::Mul};

#[derive(Debug, Clone)]
pub struct Vector<T> {
    components: Vec<T>
}


impl<T: Sub<Output = T> + Mul<Output = T> + std::iter::Sum + Clone> Vector<T> {
    pub fn from_iter<I: Iterator<Item = T>>(iter: I) -> Vector<T> {
        Vector { components: iter.collect::<Vec<T>>() }
    }


    pub fn dist(start: Vector<T>, end: Vector<T>) -> T {
        start.into_iter().zip(end.into_iter()).map(|(a, b)| a - b).map(|c| c.clone() * c).sum()
    }

    pub fn lerp(start: Vector<f32>, end: Vec<f32>, t: f32) -> Vector<f32> {
        Self::from_iter::<Iterator<Item = f32>>(start.into_iter().zip(end.into_iter()).map(|(a, b)| ((1f32 - t) * a + t * b)).collect::<Vec<f32>>().into_iter())
    }
}

impl<T> IntoIterator for Vector<T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.components.into_iter()
    }
}
