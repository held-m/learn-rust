use std::iter::FromIterator;
use std::str::{SplitWhitespace, Chars};

pub fn task2() -> () {
    let text: &str = "some text apple orange table book";
    let words = text.split_whitespace();
    let pig_text = get_pig_text(words);
    println!("pig_text: {}", pig_text);
}

fn get_pig_text(words: SplitWhitespace) -> String {

    let mut new_tex: String = String::new();

    for word  in words {
        // new_tex.push_str(change_word_to_pig(word));
        new_tex += &change_word_to_pig(word);
        // let new_word = word + "ay";
    }
    new_tex
}

fn change_word_to_pig(word: &str) -> String {

    let mut chars_wort: Vec<char> = get_vec_chars(word.chars());
    let pig_word = get_pig_word(chars_wort);

    pig_word
}

fn get_vec_chars(chars: Chars) -> Vec<char> {

    let mut vec_char:Vec<char> = Vec::new();

    for char in chars {
        vec_char.push(char);
    }

    vec_char
}

fn is_consonant(letter: char) -> bool {
    let consonants :Vec<char> = vec!['b','d','f','g','h','j','k','l','m','n','p','q','r','s','t','v','w','x','z','B','C','D','F','G','H','J','K','L','M','N','P','Q','R','S','T','V','W','X','Z'];
    for consonant in consonants {
        if consonant == letter {
            return true;
        }
    }
    false
}

fn get_pig_word(mut word: Vec<char>) -> String {
    let first_letter: char = word[0];
    if is_consonant(first_letter) {
        word.remove(0);
        return String::from_iter(word) + &get_pig_sound_consonant(first_letter);
    }
    return String::from_iter(word) + &"-hay ".to_string();
}

fn get_pig_sound_consonant(consonant: char) -> String {
    let mut pig_sound = String::new();
    pig_sound.push('-');
    pig_sound.push(consonant);
    pig_sound.push_str("ay ");
    return pig_sound;
}