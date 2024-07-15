pub trait PartialEq {
    fn eq(&self, b:&Self) -> bool;
}

mod ticket_mod {
    pub struct Ticket{
        pub title: String,
        pub description: String,
        pub status: String
    }

    impl PartialEq for Ticket {
        fn eq (self: &Self, b: &Self) -> bool {
            if self.title != b.title {
                println!("titles are diff");
                return false;
            } else if self.description != b.description {
                println!("descriptions are diff");
                return false;
            } else if self.status != b.status {
                println!("statuses are diff");
                return false;
            }
            true
            //self.title == b.title && self.description == b.description && self.status == b.status
        }
    }
}

mod trait_bounds {
    pub fn min<T: Ord>(left:T, right:T) -> T 
    // same as having ^ <T: Ord>
    //where T: Ord  
    {        
        if left <= right {
            left
        } else {
            right
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ticket_mod::Ticket;
    use super::trait_bounds::min;

    #[test]
    fn test_eq_description() {
        let ticket1: Ticket = Ticket{title:"one".to_string(), description:"descr".to_string(), status:"Done".to_string()};
        let ticket2: Ticket = Ticket{title:"one".to_string(), description:"descr".to_string(), status:"Done".to_string()};
        assert!(ticket1==ticket2);
    }

    #[test]
    fn test_neq_title() {
        let ticket1: Ticket = Ticket{title:"one".to_string(), description:"descr".to_string(), status:"Done".to_string()};
        let ticket2: Ticket = Ticket{title:"two".to_string(), description:"descr".to_string(), status:"Done".to_string()};
        assert!(! (ticket1==ticket2));
    }

    #[test]
    fn test_min() {
        assert_eq!(5_u8, min(5_u8, 8_u8));
    }
}