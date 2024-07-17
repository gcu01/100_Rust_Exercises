pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    /*
    let ticket2:Ticket = Ticket{title:ticket.title.clone(), 
                                description:ticket.description.clone(),
                                status:ticket.status.clone()};
    (ticket2, ticket.summary())
    */

    (ticket.clone(), ticket.summary())
}

#[derive(Clone)]
pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String
}

impl Ticket {
    pub fn summary(self) -> Summary {
        Summary {
            title: self.title,
            status: self.status
        }
    }
}

pub struct Summary {
    pub title: String,
    pub status:String
}

#[cfg(test)]
mod tests {
    use super::{Ticket, Summary, summary};

    #[test]
    fn test_clone() {
        let t1:Ticket = Ticket{title:"one".to_string(), description:"two".to_string(), status:"three".to_string()};
        assert!(true);
    }
}