//! DSA in Rust
/// Building algorithms in Rust to learn both DSA and Rust at the same time
mod algorithms;

use algorithms::anagram::{anagram_solution1, anagram_solution2};
use algorithms::dec_to_binary::{base_converter, dec_to_binary};
use algorithms::expression_converter::postfix_convert;
use algorithms::par_checker::par_checker;
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
    let res3 = par_checker(sa);
    let res4 = par_checker(sb);
    println!("{sa} balanced: {res3}, {sb} balanced:{res4}");

    let num = 100;
    let bin_str: String = dec_to_binary(num);
    println!("{num} = b{bin_str}");

    let num1 = 10;
    let num2 = 43;
    let bin_str: String = base_converter(num1, 2);
    let hex_str: String = base_converter(num2, 16);
    println!("{num1} = b{bin_str}, {num2} = x{hex_str}");

    // postfix converter
    let postfix_string = postfix_convert("A+B-C");
    match postfix_string {
        Some(val) => {
            println!("{val}");
        }
        None => {
            println!("not correct string");
        }
    }
}
