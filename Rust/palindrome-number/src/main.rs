use std::vec;

fn main() {
    let test = -121;
    let result = Solution::is_palindrome(test);
    println!("{}", result);
}

struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let vec_x: Vec<char> = x.clone().to_string().chars().collect();
        let mut new_x: Vec<char> = vec![];
        println!("{:?}", vec_x);
        for i in 0..x.to_string().len() {
            new_x.push(vec_x[x.to_string().len() - 1 - i]);
            println!("{}", vec_x[i]);
            //new_x.push('1');
        }
        println!("{:?}", new_x);
        if vec_x == new_x {
            return true;
        }
        return false;
    }
}
