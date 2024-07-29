// TODO: Define a function named `lowercase` that converts all characters in a string to lowercase,
//  modifying the input in place.
//  Does it need to take a `&mut String`? Does a `&mut str` work? Why or why not?


pub fn lwrcase(a: &mut str) {
    
    a.make_ascii_lowercase();
    println!("and the output is: {}", a);
}


#[cfg(test)]
mod tests {
    use super::lwrcase;

    #[test]
    fn test1() {
        let mut test:String = String::from("aAAA");
        println!("test = {}", test.to_lowercase());

        let mut aa:String = String::from("aSDFghjKL");
        lwrcase(&mut aa);
        assert_eq!("asdfghjkl".to_string(), aa);
    }
}





















/* 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut s = String::from("");
        lowercase(&mut s);
        assert_eq!(s, "");
    }

    #[test]
    fn one_char() {
        let mut s = String::from("A");
        lowercase(&mut s);
        assert_eq!(s, "a");
    }

    #[test]
    fn multiple_chars() {
        let mut s = String::from("Hello, World!");
        lowercase(&mut s);
        assert_eq!(s, "hello, world!");
    }

    #[test]
    fn mut_slice() {
        let mut s = "Hello, World!".to_string();
        lowercase(s.as_mut_str());
        assert_eq!(s, "hello, world!");
    }
}
*/