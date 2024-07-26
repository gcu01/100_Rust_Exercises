pub struct WeekTemperature{
    t: [Option<i32>; 7],
}

pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn weekday2index(day:&Weekday) -> usize {
        match day {
            Weekday::Monday => 1,
            Weekday::Tuesday => 2,
            Weekday::Wednesday => 3,
            Weekday::Thursday => 4,
            Weekday::Friday => 5,
            Weekday::Saturday => 6,
            Weekday::Sunday => 7,
        }
    }

impl WeekTemperature {
    pub fn new() -> Self {
        WeekTemperature{
            t: [None, None, None, None, None, None, None],
            //t:[None, 7]
        }
    }

    pub fn get_temperature(&self, day: Weekday) -> Option<i32>{
        let index:usize = weekday2index(&day);
        self.t[index]
    }

    pub fn set_temperature(&mut self, day: Weekday, temperature: i32) {
        let index = weekday2index(&day);
        self.t[index] = Some(temperature);
    }
}

#[cfg(test)]
mod tests {
    use super::{WeekTemperature, Weekday};

    #[test]
    fn test_new(){
        let mut t1 = WeekTemperature::new();

        assert_eq!(t1.get_temperature(Weekday::Monday),None);
    }

    #[test]
    fn test_get(){
        let mut t1 = WeekTemperature::new();
        t1.set_temperature(Weekday::Monday, 34);
        assert_eq!(Some(34), t1.get_temperature(Weekday::Monday));
    }
}