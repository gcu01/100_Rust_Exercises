// TODO: Define a function named `sum` that takes a reference to a slice of `u32` and returns the sum of all
//  elements in the slice.

pub fn sum(a: &[u32]) -> u32 {
    a.iter().sum()
}



#[cfg(test)]
mod tests {
    use super::sum;

    #[test]
    fn test1() {
        let v1:Vec<u32> = vec![1,2,3,4,5,6,7,8];

        let v2:&[u32] = v1.as_slice();
        assert_eq!(36, sum(v2));
    }

    #[test]
    fn test_empty() {
        let v1:Vec<u32> = vec![];

        let v2:&[u32] = v1.as_slice();
        assert_eq!(0, sum(v2));
    }
}


/* 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let v = vec![];
        assert_eq!(sum(&v), 0);
    }

    #[test]
    fn one_element() {
        let v = vec![1];
        assert_eq!(sum(&v), 1);
    }

    #[test]
    fn multiple_elements() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(sum(&v), 15);
    }

    #[test]
    fn array_slice() {
        let v = [1, 2, 3, 4, 5];
        assert_eq!(sum(&v), 15);
    }
}
*/