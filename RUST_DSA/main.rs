//! Math mod <--- doc comment, describe mode
//!
//! # Add <--- doc comment, describe function, test case
/// This function sums the inputs
///
/// ## Example <--- test code, use case
/// use math::add;
/// assert_eq!(3, add(1,2));
fn add(x: i32, y: i32) -> i32 {
    //sum <-- regular comment
    x + y
}

// rust recommends UpperCaseCamel for class-level content and snake_case for value-level

const AGE: i32 = 22;
// AGE = 23; error, mutation not allowed

const NUM: f64 = 233.0;
// const NUM: f64 = 211.0; error, defined

let x: f64 = 3.14; // let defines x which can be reasigned but not mutated
// x = 6.28; error, x is unmutable
let x: f64 = 2.71 // reassignable 

let mut y = 985 // let mut defines y which can be reassigned and mutated
y = 996; // y is mutable 
let y = 2019; // reassign y 

static NAME: &str = "shiebar"; // static variable which can be used as a constant
// Name = "kew", error, Name is umutable
static mut NUM: i32 = 100; // static mutable variable
// static variables are referenced, constants are replaced.
// static variables defined with static mut are wrapped in 'unsafe'
// to indicate that they're not safe. 
// It recommended to use constants and variables only
unsafe {
    NUM += 1; // NUM is mutable
    println!("NUM:{}", NUM);
}

// unicode scalar value
let c = 's';

// dynamic arrays
let c_str = "s";

let tup1: (i8, f32, i64) = (-1, 2.33, 8000_000);
let (x, y, z) = tup1;

let tup2 = (0, 100, 2.4);
let zero = tup2.0; // use symbol . get value
let one_hundred = tup2.1;

let genders = ["Female", "Male", "None-Above"];
let gender_f = genders[0]; //indice element

// [type; num] define an array
let digits: [i32; 5] = [0,1,2,3,4];
let zeros = [0; 10;]; // define array which holds then '0'

#![allow(overflowing_literals)] // ignore overflow warnings for type conversion.

fn main(){
    let decimal = 61.3214_f32; 
    // let integer: u8 = decimal; // error f32 can't be converted to u8
    let integer = decimal as u8;
    let character = integer as char;
    println!("1000 as a u16: {}", 1000 as u16);
    println!("1000 as a u8: {}", 1000 as u8);
}

pub trait From<T> {
    fn from<T> -> Self;
}

pub trait Into<T> {
    fn into<T> -> T;
}

#[derive(Debug)]
struct Complex {
    real: i32, // real quantity;
    imag: i32, // imaginary quantity
}

// implement a conversion from i32 to a complex number,
// where the i32 is converted to the real part and the imaginary
// part is set 0

impl From<i32> for Complex {
    fn from(real: i32) -> Self {
        Self { real, imag: 0 }
    }
}

fn main() {
    // implemented Into by default
    let c2: Complex - 2_i32.into();
    println!("c1: {:?}, c2: {:?}", c1, c2);
}

// Declaration statement for function
fn sum_of_nums(nums: &[i32]) -> i32{
    nums.iter().sum::<i32>()
}

let x = 5; // the entire line is a statement, x = 5 is an
// expression that calculates the value of x
// the entire line is an expression
// the entire line is a statement, y = x + 1
// is an expression

println!("{y}");
let z = [1,2,3];
println!("sum is {:?}", sum_of_nums(&z));

let some_value = Some(100);
if let Some(value) = some_value {
    print!("value: {value}");
} else {
    println!("no value");
}

let a = 10;
match a {
    0 => println!("0 == a"),
    1..=9 => println!("1 <= a < = 9"),
    _ => println!("10 <= a"),
}


// loop that has an break and continue as flow management.
let mut val = 10;
let res = loop {
    if val < 0 {
        break val;
    }

    val -= 1;
    if 0 == val % 2 {
        continue;
    }

    println!("val = {val}");
};

let num = 10;
while num > 10 {
    println!("{}", num);
    num -= 1;
}

let nums = [1,2,3,4,5,6,7,8];
let mut index = 0;
while index < 8 {
    println!("val: {}", nums[index]);
    index+=1;
}

let nums = [1,2,3,4,5,6,7,8];
for nums in nums {
    println!("val: {num}");
}

for num in nums.iter().rev() {
    print!("val:{num}");
}

let mut v = vec![1,2,3,4,5,6,7];
while let Some(x) = v.pop() {
    println!("{x}")
}

// Some() is part of the Option<T> enum:
#[derive(Debug)]
enum Option<T> {
    Some(T),
    None, 
}

let maybe_number = Some(5);
let no_number: Option<i32> = None;
