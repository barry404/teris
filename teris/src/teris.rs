use std::collections::HashSet;
use crate::shape::Shape;


#[derive(Debug)]
pub struct Teris {
    width: usize,
    height: usize,
    current_shape: Shape,
    fixed_shaep: Vec<Shape>,
}


impl Teris {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            current_shape: Shape::new_random(),
            fixed_shaep: vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::teris::Teris;

    #[test]
    fn test() {
        println!("{:#?}", Teris::new(5, 10));
    }
}


