// CS 11 Rust track: Assignment 1

// ---------------------------------------------------------------------- 
// Part 1.
// ---------------------------------------------------------------------- 


pub fn factorial_recursive(n: u64) -> u64 {
    if n == 0 {
        1
    }
    else {
        n * factorial_recursive(n-1)
    }
}

pub fn factorial_while(mut n: u64) -> u64 {
    let mut ans = 1;
    while n > 0 {
        ans *= n;
        n -= 1;
    }
    ans
}

pub fn factorial_for(n: u64) -> u64 {
    let mut ans = 1;
    for i in 1..(n + 1) {
        ans *= i;
    }
    ans
}

/// This is where the tests are located. You can ignore the syntax used to
/// declare tests for now, because we'll go over it later on in the class.
#[cfg(test)]
pub mod test1 {
    use super::*;

    #[test]
    pub fn test_factorial_recursive() {
        assert_eq!(factorial_recursive(0), 1);
        assert_eq!(factorial_recursive(1), 1);
        assert_eq!(factorial_recursive(3), 6);
        assert_eq!(factorial_recursive(5), 120);
        assert_eq!(factorial_recursive(10), 3628800);
    }

    #[test]
    pub fn test_factorial_while() {
        assert_eq!(factorial_while(0), 1);
        assert_eq!(factorial_while(1), 1);
        assert_eq!(factorial_while(3), 6);
        assert_eq!(factorial_while(5), 120);
        assert_eq!(factorial_while(10), 3628800);
    }

    #[test]
    pub fn test_factorial_for() {
        assert_eq!(factorial_for(0), 1);
        assert_eq!(factorial_for(1), 1);
        assert_eq!(factorial_for(3), 6);
        assert_eq!(factorial_for(5), 120);
        assert_eq!(factorial_for(10), 3628800);
    }
}

// ---------------------------------------------------------------------- 
// Part 2.
// ---------------------------------------------------------------------- 

pub fn takes_reference(s: &String) {
    println!("ref - {}", s);
}

pub fn takes_mut_reference(s: &mut String) {
    println!("mut - {}", s);
}

pub fn takes_ownership(s: String) -> String {
    println!("own - {}", s);
    s
}

pub fn rewrite_exercise_1() {
    let mut s = String::from("Hello world!");
    
    takes_reference(&s);

    let mr = &mut s;

    takes_mut_reference(mr);
}

pub fn rewrite_exercise_2() {
    let mut s = String::from("Hello world!");
    let mr1 = &mut s;
    takes_mut_reference(mr1);
    let mr2 = &mut s;
    takes_mut_reference(mr2);
}

pub fn rewrite_exercise_3() {
    let s = String::from("Hello world!");
    takes_reference(&s);
    let _s2 = takes_ownership(s);

}

pub fn double_ref(_a: &String, _b: &mut String) {}

// This doesn't compile; why?
// Write your answer here:
// 
/* "At any given time, you can have either one mutable reference 
or any number of immutable references."
so we cannot have one mutable reference and one immutable reference
at the same time.

pub fn explain_exercise_1() {
    let mut s = String::from("Hello world!");
    
    double_ref(&s, &mut s)
}
*/

pub fn explain_exercise_2() {
    let s1 = String::from("Hello world!"); // start of variable s1
    let mut s2 = String::from("Goodbye!"); // start of variable s2
    
    let r = &s1; // start of reference r

    takes_reference(r); // r is passed into the function, and r is invalidated
    
    takes_reference(&s2); // &s2 is passed into the function, 
                          // which doesn't influence s2 the variable
    takes_mut_reference(&mut s2);

    takes_reference(r);
    takes_mut_reference(&mut s2);

    // s2 is invalidated; s3 owns the string, start of variable s3
    let s3 = takes_ownership(s2); 

    // s1 is invalidated; 
    // s2 starts its life again and owns the string hello world
    s2 = takes_ownership(s1); 

    takes_reference(&s2);
    takes_reference(&s3);
} // s2, s3 die as well because they are out of scope


// ---------------------------------------------------------------------- 
// Part 3.
// ---------------------------------------------------------------------- 

#[derive(Debug, PartialEq, Eq)]
pub enum FizzBuzz {
    Fizz,
    Buzz,
    FizzBuzz,
    Neither (u64),
}

impl FizzBuzz {
    pub fn new(n: u64) -> Self {
        if n % 15 == 0 {
            FizzBuzz::FizzBuzz
        }
        else if n % 3 == 0 {
            FizzBuzz::Fizz
        }
        else if n % 5 == 0 {
            FizzBuzz::Buzz
        }
        else {
            FizzBuzz::Neither(n)
        }
    }
}

#[derive(Debug)]
pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2 {x: x, y: y}
    }

    pub fn hypot(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn dot(&self, other: &Vector2) -> f32 {
       self.x * other.x + self.y * other.y
    }

    pub fn add(&mut self, other: &Vector2) {
        self.x += other.x;
        self.y += other.y;
    }
}

#[cfg(test)]
pub mod test3 {
    use super::*;

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(FizzBuzz::new(2), FizzBuzz::Neither(2));
        assert_eq!(FizzBuzz::new(3), FizzBuzz::Fizz);
        assert_eq!(FizzBuzz::new(5), FizzBuzz::Buzz);
        assert_eq!(FizzBuzz::new(15), FizzBuzz::FizzBuzz);
    }

    #[test]
    fn test_vector() {
        let mut v_34 = Vector2::new(3.0, 4.0);
        let unit_x = Vector2::new(1.0, 0.0);
        let unit_y = Vector2::new(0.0, 1.0);

        assert_eq!(v_34.hypot(), 5.0);
        assert_eq!(unit_x.hypot(), 1.0);
        assert_eq!(unit_y.hypot(), 1.0);

        assert_eq!(v_34.dot(&unit_x), 3.0);
        assert_eq!(v_34.dot(&unit_y), 4.0);
        assert_eq!(unit_x.dot(&unit_y), 0.0);

        v_34.add(&unit_x);
        v_34.add(&unit_y);
        
        assert_eq!(v_34.dot(&unit_x), 4.0);
        assert_eq!(v_34.dot(&unit_y), 5.0);
    }
}

