pub enum Shape {
     Circle,
     Square,
     Rectangle,
     Triangle,
     Pentagon,
}

mod my_shape {
    use super::Shape;
    impl Shape {
        pub fn n_sides(&self) -> u8 {
            match self {
                Shape::Circle => 0,
                Shape::Square => 4,
                Shape::Rectangle => 4,
                Shape::Triangle => 3,
                Shape::Pentagon => 5,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Shape};

    #[test]
    fn test_shape_circle() {
        assert_eq!(0, Shape::Circle.n_sides());
    }

    #[test]
    fn test_shape_pentagon() {
        assert_eq!(5, Shape::Pentagon.n_sides());
    }
}

