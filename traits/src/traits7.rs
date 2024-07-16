pub trait Power<Exponent=Self>{
    type Output;

    fn power(&self, n: Exponent) -> Self::Output;
}

impl Power<u16> for u32{
    type Output=u32;
    fn power(&self, n:u16) -> Self::Output {
        let mut i=0_u16;
        let mut res = 1_u32;
        while i<n {
            res *= self;
            i+=1;
        }
        res
    }
}

impl Power<u32> for u32{
    type Output=u32;
    fn power(&self, n:u32) -> Self::Output {
        let mut i=0_u32;
        let mut res = 1_u32;
        while i<n {
            res *= self;
            i+=1;
        }
        res
    }
}

impl Power<&u32> for u32 {
    type Output = u32;
    fn power(&self, n:&u32) -> Self::Output {
        let mut i = 0_u32;
        let mut res = 1;
        while i < *n {
            res *= self;
            i += 1;
        }
    res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow_u16() {
        assert_eq!(8_u32, 2_u32.power(3_u16));
    }

    #[test]
    fn test_pow_u32() {
        assert_eq!(8_u32, 2_u32.power(3_u32));
    }

    #[test]
    fn test_pow_address_u32() {
        assert_eq!(8_u32, 2_u32.power(&3_u32));
    }
}