pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String
}

impl Ticket {
    pub fn title(&self) -> &str {
        self.title.trim()
    }
    pub fn description(&self) -> &str {
        self.description.trim()        
    }
}

#[cfg(test)]
mod tests {
    use super::Ticket;

    #[test]
    fn test_trim() {
        let t: Ticket = Ticket{title:"  title".to_string(), description: "  description".to_string(), status: "Done".to_string()};
        assert_eq!("title".to_string(), t.title());
    }
}