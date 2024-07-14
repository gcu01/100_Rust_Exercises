    pub trait IsEven {
        fn is_even(&self) -> bool;
    }
    
mod mytraits { 
    use crate::traits1::IsEven;

    pub struct MyTrait {
        value: u32
    }

    impl MyTrait {
        pub fn new( value: u32) -> MyTrait {
            MyTrait{value:value}
        }
    }
    impl IsEven for MyTrait{
        fn is_even(&self) -> bool {
            if self.value%2==0 {
                return true;
            }
        false
        }
    }
}

mod unsigned_no {
    use crate::traits1::IsEven;
    impl IsEven for u32 {
        fn is_even(&self) -> bool {
            self%2 == 0
        }
    }
}

mod integer_no {
    use crate::traits1::IsEven;
    impl IsEven for i32 {
        fn is_even(&self) -> bool {
            self %2 == 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::mytraits::{MyTrait};
    use crate::traits1::IsEven;
    //use crate::traits1::unsigned_no;

    #[test]
    fn test1_is_even(){
        let v:MyTrait = MyTrait::new(2);
        assert_eq!(true, v.is_even());
    }
    #[test]
    fn test2_is_even() {
        let v:MyTrait = MyTrait::new(5);
        assert_eq!(false, v.is_even());
    }

    #[test]
    fn test3_is_even() {
        assert_eq!(true, 4_u32.is_even());
    }
    #[test]
    fn test4_is_even() {
        assert!(!5_i32.is_even());
    }
}