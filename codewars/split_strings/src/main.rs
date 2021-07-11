mod my_data;
mod utils;

use my_data::some_str;
use utils::exec_time::time_exec_fn;

fn main() {
    let str = some_str::get_some_str();

    time_exec_fn(solution1, str, "solution1");
    time_exec_fn(solution2, str, "solution2");
    time_exec_fn(solution3, str, "solution3");
}
/// Complete the solution so that it splits the string into pairs of two characters.
/// If the string contains an odd number of characters then it should replace the missing second
/// character of the final pair with an underscore ('_').
///
/// Exanples:
///
/// solution("abcdef") // should return ["ab", "cd", "ef"]
/// solution("abcdefg") // should return ["ab", "cd", "ef", "g_"]
///
///
/// solution1 it's the best. Solution2 I was make it before i found a way make step_by(2).
/// Solution3 I took from codewars.
fn solution1(s: &str) -> Vec<String> {
    let mut str: Vec<char> = s.chars().collect();
    if str.len() % 2 != 0 {
        str.push('_');
    }

    str.iter()
        .enumerate()
        .step_by(2)
        .fold(Vec::new(), |mut acc, (i, _ch)| {
            acc.push(format!("{}{}", str[i], str[i + 1]));
            acc
        })
}

fn solution2(s: &str) -> Vec<String> {
    let res = s.char_indices().fold(Vec::new(), |mut acc, (i, ch)| {
        if i == s.len() - 1 && i % 2 == 0 {
            acc.push(String::from(ch.to_string() + "_"));
        } else if i % 2 == 0 {
            acc.push(s[i..i + 2].to_string());
        }
        acc
    });

    res
}

fn solution3(s: &str) -> Vec<String> {
    let odds: Vec<char> = s.chars().step_by(2).collect();
    let mut evens: Vec<char> = s.chars().skip(1).step_by(2).collect();
    if odds.len() != evens.len() {
        evens.push('_');
    }
    odds.iter()
        .zip(evens.iter())
        .map(|(c1, c2)| format!("{}{}", c1, c2))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solution1("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(solution1("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(solution1(""), [] as [&str; 0]);

        assert_eq!(solution2("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(solution2("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(solution2(""), [] as [&str; 0]);

        assert_eq!(solution3("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(solution3("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(solution3(""), [] as [&str; 0]);
    }
}
