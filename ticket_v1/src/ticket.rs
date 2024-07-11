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

pub mod ticket_t1{

    pub struct Ticket {
        title: String,
        description: String,
        status: String
    }

    impl Ticket {
        pub fn new (title: String, description: String, status: String) -> Self {

            if status != "To-Do" && status != "In Progress" && status != "Done" {
                panic!("wrong status");
            }

            if title.is_empty() || title.len() > 50 {
                panic!("wrong title");
            }

            if description.is_empty() || description.len()>500 {
                panic!("wrong description");
            }

            Ticket {
                title: title,
                description: description,
                status: status
            }
        } 
        pub fn title(&self) -> &String {
            &self.title
        }
        pub fn description(&self) -> &String {
            &self.description
        }
        pub fn status(&self) -> &String {
            &self.status
        }  
        pub fn set_title(&mut self, new_title:String) {
            if new_title.is_empty() {
                panic!("the new title is empty");
            } 
            if new_title.len() > 50 {
                panic!("the new title is too long (over 50 chars)");
            }
            self.title = new_title;
        }
        pub fn set_description(&mut self, new_description: String) {
            if new_description.is_empty() {
                panic!("the new description is empty");
            }
            if new_description.len() > 500 {
                panic!("the new description is too long (over 500 chars)");
            }
            self.description = new_description;
        }
        pub fn set_status(&mut self, new_status:String) {
            if new_status.is_empty() {
                panic!("the new status is empty");
            }
            if new_status != "To-Do" && new_status != "In Progress" && new_status != "Done" {
                panic!("wrong status");
            }
            self.status = new_status;
        }
    }
}

mod helpers {
    use crate::ticket::ticket_t1::Ticket;
    pub fn create_todo_ticket(title:String, description: String) -> Ticket {
        Ticket::new(title, description, "To-Do".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::ticket::ticket_t1::Ticket;
    use super::ticket_t1::*;
    use std::mem::size_of;

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

    #[test]
    #[should_panic(expected="wrong status")]
    fn test_new_status() {
        Ticket::new("a".to_string(), "b".to_string(), "Not Done".to_string());
    }

    #[test]
    #[should_panic("wrong title")]
    fn test_new_title() {
        Ticket::new("".to_string(), "a".to_string(), "c".to_string());
    }

    #[test]
    fn test_mod() {
        let test_ticket:Ticket = Ticket::new("title".to_string(), "description".to_string(), "Done".to_string());
        assert_eq! (test_ticket.title(), helpers::create_todo_ticket("title".to_string(), "description".to_string()).title());
    }

    #[test]
    #[should_panic("the new title is empty")]
    fn test_set_title_empty () {
        let mut t:Ticket = Ticket::new("title".to_string(), "description".to_string(), "status".to_string());
        t.set_title("".to_string());
    }

    #[test]
    fn u16_size() {
        assert_eq!(2, size_of::<u16>());
    }
    #[test]
    fn i32_size() {
        assert_eq!(4, size_of::<i32>());
    }
    #[test]
    fn bool_size() {
        assert_eq!(1, size_of::<bool>());
    }
}