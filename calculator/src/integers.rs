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
    let distance = end - start;
    // Don't change the line below
    distance / time_elapsed
}

/// Return `true` if `n` is even, `false` otherwise.
pub fn is_even(n: u32) -> bool {
    if n%2==0 {return true;}
    false
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
    fn test_is_even1() {
        assert_eq!(true, is_even(2));
    }

    #[test]
    fn test_is_even2() {
        assert_ne!(true, is_even(1));
    }
    

}