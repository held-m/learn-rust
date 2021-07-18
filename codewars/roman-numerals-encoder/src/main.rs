use std::str::Chars;

fn main() {
    num_as_roman(1000);
}

/// Create a function taking a positive integer as its parameter and returning a string containing the Roman Numeral representation of that integer.
///
/// Modern Roman numerals are written by expressing each digit separately starting with the left most digit and skipping any digit with a value of zero. In Roman numerals 1990 is rendered: 1000=M, 900=CM, 90=XC; resulting in MCMXC. 2008 is written as 2000=MM, 8=VIII; or MMVIII. 1666 uses each Roman symbol in descending order: MDCLXVI.
///
/// Example:
///
/// solution(1000); // should return 'M'
///
/// Help:
/// Symbol    Value
/// I          1
/// V          5
/// X          10
/// L          50
/// C          100
/// D          500
/// M          1,000

fn num_as_roman(num: i32) -> String {
   
    let num: Vec<char> = num.to_string()[..].chars().rev().collect();
    
    println!("Num: {:?}", num);


    "X".to_string() 
}




#[test]
fn returns_expected() {
  assert_eq!(num_as_roman(182), "CLXXXII");
  assert_eq!(num_as_roman(1990), "MCMXC");
  assert_eq!(num_as_roman(1875), "MDCCCLXXV");
}
