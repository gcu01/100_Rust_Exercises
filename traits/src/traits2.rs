pub trait PartialEq {
    fn eq(&self, b:&Self) ->bool;
}
impl PartialEq for u32 {
    fn eq(&self, b:&Self) -> bool {
        *self == *b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eq() {
        assert!(5_u32.eq(5_u32));
    }
}