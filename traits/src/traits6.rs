pub struct WrappingU32 {
    pub value: u32
}

impl From<u32> for WrappingU32 {
    fn from(value: u32)-> Self {
        WrappingU32{value}
    }
}

fn example() {
    let wrapping: WrappingU32 = 32.into();
    let wrapping = WrappingU32::from(42);
}

#[cfg(test)]
mod tests {
    use super::WrappingU32;

    #[test]
    fn test_trait(){
        let w1:WrappingU32 = WrappingU32 {value:13_u32};
        let w2 = WrappingU32::from(13_u32);
        assert_eq!(w1.value, w2.value);
    }
}