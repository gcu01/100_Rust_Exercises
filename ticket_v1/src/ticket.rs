pub struct Order {
    pub price: u32,
    pub quantity: u32
}

impl Order {
    fn is_available(self) -> bool {
        self.quantity > 0
    }
    pub fn my_fct(a:u32) -> bool {
        a>0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_available() {
        let o:Order = Order{ price: 10, quantity:2};
        assert_eq!(true, o.is_available());
    }

    #[test]
    fn test_in_not_available() {
        let o:Order = Order{ price:2, quantity:0};
        assert!(!o.is_available());
    }
}