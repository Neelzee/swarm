use std::{cmp::Ordering, collections::{HashMap, BinaryHeap}, hash::{Hash, Hasher}};

#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    components: Vec<f32>
}


impl Vector {
    pub fn new(components: Vec<f32>) -> Self {
        Vector { components }
    }

    pub fn from_iter<I: Iterator<Item = f32>>(iter: I) -> Self {
        Vector { components: iter.collect::<Vec<f32>>() }
    }


    pub fn dist(&self, end: &Vector) -> f32 {
        f32::sqrt(self.into_iter().zip(end.into_iter()).map(|(a, b)| a - b).map(|c| f32::powi(c, 2)).sum())
    }

    pub fn lerp(start: Vector, end: Vec<f32>, t: f32) -> Self {
        Self::from_iter(start.into_iter().zip(end.into_iter()).map(|(a, b)| ((1f32 - t) * a + t * b)).collect::<Vec<f32>>().into_iter())
    }

    fn approx_eq(&self, other: &Vector, epsilon: f32) -> bool {
        f32::powi(self.dist(other), 2) < epsilon.powi(2)
    }
}

impl IntoIterator for Vector {
    type Item = f32;

    type IntoIter = std::vec::IntoIter<f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.components.into_iter()
    }
}

// Implement Hash for Vector
impl Hash for Vector {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for component in &self.components {
            // Convert the f32 component to its raw binary representation (u32) using to_bits()
            let bits: u32 = component.to_bits();
            bits.hash(state);
        }
    }
}

impl Eq for Vector {
    
}

// Define the Node struct for A* algorithm
struct Node {
    vector: Vector,
    g: f32,
    h: f32,
}

impl Node {
    fn new(vector: Vector, g: f32, h: f32) -> Self {
        Node { vector, g, h }
    }

    fn f(&self) -> f32 {
        self.g + self.h
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.vector.approx_eq(&other.vector, 0.05f32)
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare based on f value (f = g + h)
        self.f().partial_cmp(&other.f()).unwrap().reverse()
    }
}

fn reconstruct_path(came_from: HashMap<Vector, Vector>, mut current: Vector, start: Vector) -> Vec<Vector> {
    let mut path = vec![current.clone()];
    while current.clone() != start {
        current = *came_from.get(&(current.clone())).unwrap();
        path.push(current.clone());
    }
    path.reverse();
    path
}

fn a_star(start: Vector, goal: Vector) -> Option<Vec<Vector>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<Vector, Vector> = HashMap::new();
    let mut g_scores: HashMap<Vector, f32> = HashMap::new();

    g_scores.insert(start, 0.0);
    open_set.push(Node::new(start, 0.0, start.dist(&goal)));

    while !open_set.is_empty() {
        let current_node = open_set.pop().unwrap();
        let current_vector = current_node.vector;

        if current_vector == goal {
            return Some(reconstruct_path(came_from, current_vector, start));
        }

        for neighbor in &[/* Add neighboring vectors here */] {
            let tentative_g_score = g_scores.get(&current_vector).unwrap_or(&f32::MAX) + current_vector.dist(&neighbor);

            if tentative_g_score < *g_scores.get(&neighbor).unwrap_or(&f32::MAX) {
                came_from.insert(*neighbor, current_vector);
                g_scores.insert(*neighbor, tentative_g_score);
                open_set.push(Node::new(*neighbor, tentative_g_score, neighbor.dist(&goal)));
            }
        }
    }

    None
}
