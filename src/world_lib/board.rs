use crate::vector::Vector;

pub struct Board<T> {
    dim: Vector,
    board: Vec<T>
}

pub struct Tile {

}


impl<T> Board<T> {
    pub fn new(dim: Vector, size: Vector) -> Board<T> {
        Board { dim, board: Self::create_board(size) }
    }

    fn create_board(size: Vector) -> Option<Vec<T>> {
        let mut comp = size.get_components();


        let mut vec = Vec::new();

        match comp.pop() {
            Some(el) => {
                for _ in 0..el {
                    match Self::create_board(Vector::from_iter(vec.into_iter())) {
                        Some(el) => vec.push(el),
                        None => todo!(),
                    }
                }
            }
            None => return None
        }

        return vec;

    }
}