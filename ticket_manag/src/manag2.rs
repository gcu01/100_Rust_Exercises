mod f {
    pub fn fibonacci(n: u32) -> u32{
        let mut v:Vec<u32> = Vec::new();
        let mut idx = 2;

        if n == 0 { return 0_u32; }
        else if n == 1 { return 1_u32; }
        else {
            v.push(0);
            v.push(1);

            while idx <= n {
                println!("{}",(idx-2)+(idx-1));
                v.push(v[(idx as usize)-2]+v[(idx as usize)-1]);
                idx += 1;
            }
            
            /* for val in v.iter() {
                result += val;
            }*/
        }
        println!("{:?}", v);
        v[n as usize]
    }
}

#[cfg(test)]
mod tests{
    use super::f::fibonacci;

    #[test]
    fn test_fibonacci1(){
        assert_eq!(0, fibonacci(0));
    }

    
    #[test]
    fn test_fibonacci2(){
        assert_eq!(1, fibonacci(1));
    }

    
    #[test]
    fn test_fibonacci3(){
        assert_eq!(1, fibonacci(2));
    }

    #[test]
    fn test_fibonacci4(){
        assert_eq!(2, fibonacci(3));
    }

    #[test]
    fn test_fibonacci5(){
        assert_eq!(55, fibonacci(10));
    }

    #[test]
    fn test_fibonacci6(){
        assert_eq!(fibonacci(30), 832040);
    }
}