use std::vec;

fn main() {
    let test: i32 = 121;
    //let result = Solution::is_palindrome(test);
    println!("{}", test.isqrt());
    if test.isqrt().pow(2) == test {
        println!("Is perfect square");
    }
}

struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let vec_x: Vec<char> = x.clone().to_string().chars().collect();
        let mut new_x: Vec<char> = vec![];
        for i in 0..x.to_string().len() {
            new_x.push(vec_x[x.to_string().len() - 1 - i]);
        }
        if vec_x == new_x {
            return true;
        }
        return false;
    }
}

struct Sol2;

impl Sol2 {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        return true;
    }
}
