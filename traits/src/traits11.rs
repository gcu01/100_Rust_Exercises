// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

/*pub trait From<T> {
    type Output;
    pub fn from(&self, outsider: T)-> Output;
} */


pub mod my_type {
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct SaturatingU16 {
        value:u16,
    }

    impl From<u16> for SaturatingU16 {
        fn from(outsider: u16) -> Self {
            Self {
                value: outsider
            }
        }
    }

    impl From<u8> for SaturatingU16 {
        fn from(outsider: u8) -> Self {
            Self {
                value: outsider as u16
            }
        }
    }

    impl From<&u16> for SaturatingU16 {
        fn from(outsider: &u16) -> Self {
            Self {
                value: *outsider
            }
        }
    }

    impl From<&u8> for SaturatingU16 {
        fn from(outsider: &u8) -> Self {
            Self {
                value: *outsider as u16
            }
        }
    }

    impl std::ops::Add for SaturatingU16 {
        type Output=SaturatingU16;
        fn add(self, rhs: SaturatingU16) -> Self::Output {
            Self {
                value: self.value + rhs.value
            }
        }
    }

    impl std::ops::Add<&SaturatingU16> for SaturatingU16 {
        type Output=SaturatingU16;
        fn add(self, rhs:&SaturatingU16) -> Self::Output {
            Self {
                value:self.value + rhs.value
            }
        }
    }

    impl std::ops::Add<u16> for SaturatingU16 {
        type Output=SaturatingU16;
        fn add(self, rhs:u16) -> Self::Output {
            Self {
                value:self.value+rhs
            }
        }
    }

    impl std::ops::Add<&u16> for SaturatingU16 {
        type Output=SaturatingU16;
        fn add(self, rhs:&u16) -> Self::Output {
            Self {
                value:self.value+*rhs
            }
        }
    }

    impl PartialEq<u16> for SaturatingU16 {
        //type Output= SaturatingU16;
        fn eq(&self, rhs:&u16) -> bool {
            self.value == *rhs
        }
    }

    impl SaturatingU16 {
        pub fn get_value_u16(&self) -> u16 {
            self.value
        }
        pub fn get_value_u8(&self) -> u8 {
            self.value as u8
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::traits11::my_type::SaturatingU16;

    #[test]
    fn test_conversion_u16() {
        let tst:SaturatingU16 = SaturatingU16::from(1_u16);
        assert_eq!(1_u16, tst.get_value_u16());
    }

    #[test]
    fn test_conversion_u8() {
        let tst:SaturatingU16 = SaturatingU16::from(1_u8);
        assert_eq!(1_u8, tst.get_value_u8());
    }

    #[test]
    fn test_conversion_address_u16() {
        let tst:SaturatingU16 = SaturatingU16::from(&1_u16);
        assert_eq!(1_u16, tst.get_value_u16());
    }

    #[test]
    fn test_conversion_address_u8() {
        let tst:SaturatingU16 = SaturatingU16::from(&1_u8);
        assert_eq!(1_u8, tst.get_value_u8());
    }

    #[test]
    fn test_add_saturating_u16() {
    let a:SaturatingU16 = SaturatingU16::from(2_u16);
    let b:SaturatingU16 = SaturatingU16::from(3_u16);

    assert_eq!(5, (a+b).get_value_u16());
    }
    #[test]
    fn test_add_address_saturating_u16() {
    let a:SaturatingU16 = SaturatingU16::from(2_u16);
    let b:SaturatingU16 = SaturatingU16::from(3_u16);

    assert_eq!(5, (a+&b).get_value_u16());
    }

    #[test]
    fn test_add_u16() {
    let a:SaturatingU16 = SaturatingU16::from(2_u16);

    assert_eq!(5, (a+3).get_value_u16());
    }

    #[test]
    fn test_add_address_u16() {
    let a:SaturatingU16 = SaturatingU16::from(2_u16);

    assert_eq!(5, (a+&3).get_value_u16());
    }

    #[test]
    fn test_eq() {
        assert_eq!(5_u16, (SaturatingU16::from(5_u16)).get_value_u16());
    }
}