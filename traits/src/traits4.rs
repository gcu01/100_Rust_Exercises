pub struct Ticket {
    title: String,
    description: String,
    status: String
}

mod ticket_methods {
    use super::Ticket;
    impl Ticket {
        pub fn new(title: String, description: String, status: String) -> Ticket{
            if title == "" {
                panic!("title is empthy");
            }
            if title.len() > 50 {
                panic!("to long title");
            }
            if description == "" {
                panic!("description is empthy");
            }
            if description.len() > 300 {
                panic!("too long description");
            }
            if status != "Done" && status != "To-Do" && status != "In Progress" {
                panic!("wrong status");
            }
            Ticket {title, description, status}
        }
        pub fn get_title(&self) -> &str {
            &self.title
        }
        pub fn get_description(&self) -> &str {
            &self.description
        }
        pub fn get_status(&self) -> &str {
            &self.status
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Ticket};

    #[test]
    fn test_get_title() {
        let t1:Ticket = Ticket::new("title".to_string(), "descr".to_string(), "Done".to_string());
        assert_eq!("title".to_string(), t1.get_title());
    }
}