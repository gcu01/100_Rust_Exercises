//mod new_o {
    #[derive(Debug)]
    pub struct Order {
        product_name: String,
        quantity: u16,
        unit_price: u32
    }

    impl Order {
        pub fn new(product_name: String, quantity: u16, unit_price: u32) -> Self{
            if product_name.is_empty() {
                panic!("product name should not be empty");
            }
            if product_name.len() > 300 {
                panic!("product name should not be longer than 300 chars");
            }
            if quantity == 0 {
                panic!("quantity should not be zero");
            }
            if unit_price == 0 {
                panic!("unit price should not be zero");
            }

            Order {
                product_name,
                quantity,
                unit_price
            }
        }
        pub fn print_order(&self) {
            println!("Order = {:?}", self);
        }
        pub fn set_product_name(&mut self, product_name:String) {
            self.product_name = product_name;
        }
        
        pub fn set_quantity(&mut self, quantity: u16) {
            if quantity == 0 {
                panic!("the price should not be zero");
            }
            self.quantity = quantity;
        }
        
        pub fn set_unit_price(&mut self, unit_price: u32) {
            if unit_price == 0 {
                panic!("the unit price should not be zero");
            }
            self.unit_price = unit_price;
        }
        
        pub fn get_product_name(&self) -> String {
            (*self.product_name).to_string()
        }
        
        pub fn get_quantity(&self) -> u16 {
            self.quantity
        }

        fn get_unit_price(&self) -> u32 {
            self.unit_price
        }

        pub fn total(&self) ->u32 {
            self.quantity as u32 * self.unit_price
        }
    }
//}

#[cfg(test)]
mod tests {
    use super::{Order};

    #[test]
    #[should_panic(expected="product name should not be empty")]
    fn test_new_product_name() {
        Order::new("".to_string(), 2, 1);
    }
    #[test]
    #[should_panic(expected="quantity should not be zero")]
    fn test_new_quantity() {
        
        let ord1:Order = Order::new("one".to_string(), 10_u16, 10_u32);
        ord1.print_order();

        Order::new("one".to_string(), 0, 1);
    }

    #[test]
    fn test_set_product_name(){
        let mut o1:Order = Order::new("one".to_string(), 10, 2);
        //o1.print_order();
        o1.set_product_name("two".to_string());
        assert_eq!("two".to_string(), o1.get_product_name());       
    }

    #[test]
    fn test_total() {
        let ord2:Order = Order::new("tomate".to_string(), 2, 5);
        assert_eq!(10, Order::get_quantity(&ord2) as u32 * ord2.get_unit_price());
    }
}