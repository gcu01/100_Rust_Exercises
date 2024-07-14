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

#[cfg(test)]
mod tests {
    use super::mytraits::{MyTrait};
    use crate::traits1::IsEven;

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
}