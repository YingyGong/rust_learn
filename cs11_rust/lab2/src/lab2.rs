use std::convert::TryInto;

type U64Filter = std::iter::Filter<std::ops::RangeFrom<u64>, fn(&u64) -> bool>;


// ---------------------------------------------------------------------- 
// Part 1.
// ---------------------------------------------------------------------- 

pub fn divide_exact(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    }
    else if a % b == 0 {
        Some(a / b)
    }
    else {
        None
    }
}

pub fn checked_factorial(n: u64) -> Option<u64> {
    if n == 0 {
        Some(1)
    }
    else {
        let last = checked_factorial(n-1)?;
        n.checked_mul(last)
    }
}

#[derive(Debug, PartialEq)]
pub enum GCDError {
    NotPositive,
    OutOfRange,
    NotAnInteger,
}

// I am unsure whether my code is efficient
pub fn float_gcd(a: f32, b: f32) -> Result<f32, GCDError> {
   if (a < 0.0) | (b < 0.0) {
    Err(GCDError::NotPositive)
   }
   else if (a > 16777215.0) | (b > 16777215.0) {
    Err(GCDError::OutOfRange)
   }
   else if (a.fract() != 0.0) | (b.fract() != 0.0) {
    Err(GCDError::NotAnInteger)
   }
   else if (a == 0.0) {
    Ok(b)
    }
    else if (b == 0.0) {
     Ok(a)
    }
    else if a > b{
       let ans = float_gcd(b, a % b)?;
       Ok(ans)
    }
    else {
        let ans = float_gcd(a, b % a)?;
        Ok(ans)
    }
}

pub fn divide_gcd(a: u16, b: u16) -> (u16, u16) {
    let gcd = float_gcd(a.into(), b.into());
    let gcd = gcd.unwrap();
    (divide_exact(a.into(), gcd as i32).unwrap() as u16, 
    divide_exact(b.into(), gcd as i32).unwrap() as u16)
}

#[cfg(test)]
mod test1 {
    use super::*;

    #[test]
    fn test_divide_exact() {
        assert_eq!(divide_exact(1, 0), None);
        assert_eq!(divide_exact(1, 10), None);
        assert_eq!(divide_exact(100, 10), Some(10));
        assert_eq!(divide_exact(-100, 10), Some(-10));
        assert_eq!(divide_exact(44217, 153), Some(289));
    }

    #[test]
    fn test_checked_factorial() {
        assert_eq!(checked_factorial(0), Some(1));
        assert_eq!(checked_factorial(1), Some(1));
        assert_eq!(checked_factorial(5), Some(120));
        assert_eq!(checked_factorial(10), Some(3628800));
        assert_eq!(checked_factorial(21), None);
    }

    #[test]
    fn test_float_gcd() {
        assert_eq!(float_gcd(-1.0, 0.0), Err(GCDError::NotPositive));
        assert_eq!(float_gcd(0.0, -1.0), Err(GCDError::NotPositive));
        assert_eq!(float_gcd(2f32.powf(24.0), 1.0), Err(GCDError::OutOfRange));
        assert_eq!(float_gcd(1.0, 2f32.powf(24.0)), Err(GCDError::OutOfRange));
        assert_eq!(float_gcd(1.5, 2.0), Err(GCDError::NotAnInteger));
        assert_eq!(float_gcd(2.0, 1.5), Err(GCDError::NotAnInteger));
        assert_eq!(float_gcd(12.0, 32.0), Ok(4.0));
        assert_eq!(float_gcd(1479.0, 319.0), Ok(29.0));
    }

    #[test]
    fn test_divide_gcd() {
        assert_eq!(divide_gcd(5, 5), (1, 1));
        assert_eq!(divide_gcd(10, 5), (2, 1));
        assert_eq!(divide_gcd(31, 23), (31, 23));
        assert_eq!(divide_gcd(81, 9), (9, 1));
    }
}

// // ---------------------------------------------------------------------- 
// // Part 2.
// // ---------------------------------------------------------------------- 

// pub fn create_array_1() -> [i32; 10] {
//     // TODO
// }

// pub fn create_array_2(a: i32, b: i32, c: i32, d: i32) -> [i32; 4] {
//     // TODO
// }

// pub fn create_vec_empty() -> Vec<i32> {
//     // TODO
// }

// pub fn create_vec_macro_1() -> Vec<i32> {
//     // TODO
// }

// pub fn create_vec_macro_2(a: i32, b: i32, c: i32, d: i32) -> Vec<i32> {
//     // TODO
// }

// pub fn add_two(x: &mut Vec<i32>) {
//     // TODO
// }

// pub fn set_to_one(x: &mut [i32]) {
//     // TODO
// }

// #[cfg(test)]
// mod test2 {
//     use super::*;

//     #[test]
//     fn test_array_functions() {
//         let mut arr1 = create_array_1();
//         set_to_one(&mut arr1[..5]);
//         assert_eq!(arr1, [1, 1, 1, 1, 1, 0, 0, 0, 0, 0]);

//         let arr2 = create_array_2(6, 7, 8, 9);
//         assert_eq!(arr2, [6, 7, 8, 9]);
//     }

//     #[test]
//     fn test_vec_functions() {
//         let mut vec1 = create_vec_empty();
//         add_two(&mut vec1);
//         add_two(&mut vec1);
//         add_two(&mut vec1);
//         set_to_one(&mut vec1[..2]);
//         assert_eq!(vec1, vec![1, 1, 2]);

//         let mut vec2 = create_vec_macro_1();
//         set_to_one(&mut vec2[5..]);
//         assert_eq!(vec2, vec![0, 0, 0, 0, 0, 1, 1, 1, 1, 1]);

//         let vec3 = create_vec_macro_2(6, 7, 8, 9);
//         assert_eq!(vec3, vec![6, 7, 8, 9]);
//     }
// }

// // ---------------------------------------------------------------------- 
// // Part 3.
// // ---------------------------------------------------------------------- 

// pub fn vec_mean(data: &[f32]) -> f32 {
//     // TODO
// }

// pub fn vec_variance(data: &[f32]) -> f32 {
//     // TODO
// }

// pub fn iterator_factorial(n: u64) -> u64 {
//     // TODO
// }

// pub fn primes_iterator() -> U64Filter {
//     // TODO
// }

// pub fn nth_prime(n: usize) -> u64 {
//     // TODO
// }

// pub fn n_primes(n: usize) -> Vec<u64> {
//     // TODO
// }

// #[cfg(test)]
// mod test3 {
//     use super::*;

//     #[test]
//     fn test_iterator_factorial() {
//         assert_eq!(iterator_factorial(0), 1);
//         assert_eq!(iterator_factorial(1), 1);
//         assert_eq!(iterator_factorial(3), 6);
//         assert_eq!(iterator_factorial(5), 120);
//         assert_eq!(iterator_factorial(10), 3628800);
//     }

//     #[test]
//     fn test_n_primes() {
//         assert_eq!(n_primes(1), vec![2]);
//         assert_eq!(n_primes(2), vec![2, 3]);
//         assert_eq!(n_primes(5), vec![2, 3, 5, 7, 11]);
//         assert_eq!(n_primes(10), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
//     }

//     #[test]
//     fn test_nth_prime() {
//         assert_eq!(nth_prime(0), 2);
//         assert_eq!(nth_prime(1), 3);
//         assert_eq!(nth_prime(5), 13);
//         assert_eq!(nth_prime(10), 31);
//     }
// }
