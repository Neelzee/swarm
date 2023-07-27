use std::{ops::Sub, ops::Mul};

#[derive(Debug, Clone)]
pub struct Vector {
    components: Vec<f32>
}


impl Vector {
    pub fn from_iter<I: Iterator<Item = f32>>(iter: I) -> Vector {
        Vector { components: iter.collect::<Vec<f32>>() }
    }


    pub fn dist(start: Vector, end: Vector) -> f32 {
        start.into_iter().zip(end.into_iter()).map(|(a, b)| a - b).map(|c| c.clone() * c).sum()
    }

    pub fn lerp(start: Vector, end: Vec<f32>, t: f32) -> Vector {
        Self::from_iter(start.into_iter().zip(end.into_iter()).map(|(a, b)| ((1f32 - t) * a + t * b)).collect::<Vec<f32>>().into_iter())
    }
}

impl IntoIterator for Vector {
    type Item = f32;

    type IntoIter = std::vec::IntoIter<f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.components.into_iter()
    }
}
