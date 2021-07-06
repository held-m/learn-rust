mod exec_time;

use std::iter::Zip;
use std::ops::Range;
use std::vec::IntoIter;

use std::fs;
use std::io::Read;

/// In this kata you are required to, given a string, replace every letter with its position in the alphabet.
///
/// If anything in the text isn't a letter, ignore it and don't return it.
///
/// "a" = 1, "b" = 2, etc.
/// Example
///
/// alphabet_position("The sunset sets at twelve o' clock.")
///
/// Should return "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11" (as a string)

fn main() {
    let text = &get_text()[..];

    exec_time::exec_time::time_exec_fn(alphabet_position_1, text, "alphabet_position_1");
    exec_time::exec_time::time_exec_fn(alphabet_position_2, text, "alphabet_position_2");
    exec_time::exec_time::time_exec_fn(alphabet_position_3, text, "alphabet_position_3");
}

fn alphabet() -> Vec<char> {
    vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ]
}

fn alphabet_hash() -> Zip<IntoIter<char>, Range<i32>> {
    alphabet().into_iter().zip(1..27)
}

fn alphabet_position_1(text: &str) -> String {
    let check = text
        .to_lowercase()
        .chars()
        .map(|ch| {
            alphabet_hash().fold(String::new(), |mut acc, (letter, index)| {
                if ch == letter {
                    acc += index.to_string().as_str();
                    acc += " ";
                }

                acc
            })
        })
        .collect::<String>();

    check.trim().to_string()
}

fn alphabet_position_2(text: &str) -> String {
    let mut result: String = String::new();

    for char in text.to_lowercase().chars() {
        for (index, letter) in alphabet().iter().enumerate() {
            if letter == &char {
                result.push_str((index + 1).to_string().as_str());
                result.push(' ');
            }
        }
    }

    result.trim().to_string()
}

// Best solution
fn alphabet_position_3(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|c| c >= &'a' && c <= &'z')
        .map(|c| (c as u32 - 96).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

fn get_text() -> String {
    let file = fs::File::open("text.txt");

    let mut file = match file {
        Ok(file) => file,
        Err(e) => panic!("Error open file: {}", e),
    };

    let mut text = String::new();

    match file.read_to_string(&mut text) {
        Ok(_) => &text,
        Err(err) => panic!("Error read file: {}", err),
    };

    text
}

#[test]
fn returns_expected() {
    assert_eq!(
        alphabet_position_1("The sunset sets at twelve o' clock."),
        "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11".to_string()
    );
    assert_eq!(
        alphabet_position_1("The narwhal bacons at midnight."),
        "20 8 5 14 1 18 23 8 1 12 2 1 3 15 14 19 1 20 13 9 4 14 9 7 8 20".to_string()
    );

    assert_eq!(
        alphabet_position_2("The sunset sets at twelve o' clock."),
        "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11".to_string()
    );
    assert_eq!(
        alphabet_position_2("The narwhal bacons at midnight."),
        "20 8 5 14 1 18 23 8 1 12 2 1 3 15 14 19 1 20 13 9 4 14 9 7 8 20".to_string()
    );

    assert_eq!(
        alphabet_position_3("The sunset sets at twelve o' clock."),
        "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11".to_string()
    );
    assert_eq!(
        alphabet_position_3("The narwhal bacons at midnight."),
        "20 8 5 14 1 18 23 8 1 12 2 1 3 15 14 19 1 20 13 9 4 14 9 7 8 20".to_string()
    );
}
