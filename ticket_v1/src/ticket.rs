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

pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String
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
}

mod helpers {
    use super::Ticket;
    pub fn create_todo_ticket(title:String, description: String) -> Ticket {
        Ticket::new(title, description, "To-Do".to_string())
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
        let test_ticket:Ticket = Ticket{title:"title".to_string(), description:"description".to_string(), status:"Done".to_string()};
        assert_eq! (test_ticket.title, helpers::create_todo_ticket("title".to_string(), "description".to_string()).title);

    }
}