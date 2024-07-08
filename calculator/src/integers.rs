pub fn print_max_min_i32() {
    println!("max i32 = {}, min i32={}", i32::MAX, i32::MIN);
}

pub fn compute(a: u32, b: u32) -> u32 {
    // TODO: change the line below to fix the compiler error and make the tests pass.
    a + b * 4u32
}

pub fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: define a variable named `distance` with the right value to get tests to pass
    //  Do you need to annotate the type of `distance`? Why or why not?
    if time_elapsed == 0 { panic!("The journey took no time at all, that's impossible!");}
        (end - start) / time_elapsed
}

/// Return `true` if `n` is even, `false` otherwise.
pub fn is_even(n: u32) -> bool {
    if n%2==0 {return true;}
    false
}

pub fn factorial(n:u32) -> u32 {
    if n == 0 {return 1;}
    else {return n*factorial(n-1);}
}

#[cfg(test)]
mod tests {
    use crate::integers::*;


    #[test]
    fn test_compute() {
        assert_eq!(17, compute(1, 4));
    }
    #[test]
    fn case_speed1() {
        assert_eq!(speed(0, 10, 10), 1);
    } 
    #[test]
    fn case_speed2() {
        assert_eq!(speed(10, 30, 10), 2);
    }
    #[test]
    fn case_speed3() {
        assert_eq!(speed(10, 31, 10), 2);    }

    #[test]
    #[should_panic(expected = "The journey took no time at all, that's impossible!")]
    fn case_spee4(){
         assert_eq!(1, speed(5, 2, 0));
    }

    #[test]
    fn test_is_even1() {
        assert_eq!(true, is_even(2));
    }

    #[test]
    fn test_is_even2() {
        assert_ne!(true, is_even(1));
    }

    #[test]
    fn test_factorial1() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn test_factorial2() {
        assert_eq!(1, factorial(1));
    }

    #[test]
    fn test_factorial3() {
        assert_eq!(120, factorial(5));
    }

}