use std::io;

fn main() {
    let mut user_input:String = String::new();
    println!("Please define a word separator ?!");
    io::stdin().read_line(&mut user_input).expect("Err reading your input");
    let word_separator = user_input.chars().take(1).last().unwrap();

    println!("Please write your input text ?!");
    let mut user_input:String = String::new();
    io::stdin().read_line(&mut user_input).expect("Err reading your input");
    
    let words = explode(word_separator, &user_input);
    for (i,word) in words.into_iter().enumerate()
    {
        println!("{} - {}",i,word);
    }
}

fn explode(word_separator:char, user_input: &String) -> Vec<&str>
{
    let text_under_processing = user_input.trim();
    let mut words:Vec<&str> = Vec::new();
    let mut start_index:usize = 0;
    for (end_index, letter) in text_under_processing.chars().enumerate()
    {
        if letter == word_separator 
        {
            words.push(&text_under_processing[start_index..end_index]);
            start_index = end_index+1;
        }
    }
    //push last word
    words.push(&text_under_processing[start_index..]);

    words
}