//! DSA in Rust
/// Building algorithms in Rust to learn both DSA and Rust at the same time
mod algorithms;

use algorithms::anagram::{anagram_solution1, anagram_solution2};
use algorithms::par_checker::{par_checker1, par_checker2, par_checker3};
use algorithms::stack::Stack;

fn main() {
    // Example usage of anagram algorithms
    let s1 = "listen";
    let s2 = "silent";

    println!("Are '{}' and '{}' anagrams?", s1, s2);
    println!("Solution 1: {}", anagram_solution1(s1, s2));
    println!("Solution 2: {}", anagram_solution2(s1, s2));

    // Example with strings
    let mut string_stack = Stack::new();
    string_stack.push(String::from("Hello"));
    string_stack.push(String::from("World"));

    // Using into_iter
    for s in string_stack {
        println!("{}", s);
    }

    let mut s = Stack::new();
    let sum1 = s.iter().sum::<i32>();
    let mut addend = 0;
    for item in s.iter_mut() {
        *item += 1;
        addend += 1;
    }

    println!("Sum: {}", sum1);
    println!("Addend: {}", addend);

    // parenthesis checker with stack
    let sa = "(2+3){fun}[abc]";
    let sb = "()((()";
    // let res1 = par_checker1(sa);
    let res1 = par_checker2(sa);
    let res2 = par_checker2(sb);
    println!("{sa} balanced: {res1}, {sb} balanced:{res2}");
    let res3 = par_checker3(sa);
    let res4 = par_checker3(sb);
    println!("{sa} balanced: {res3}, {sb} balanced:{res4}");
}
