/*
For now, just define the structure of the project

the libraries to use::
1) rust equivalent to torch (for tensors)
2) rust equivalent to torch.nn (for neural networks)
3) cuda bindings for rust to use GPU
*/

use std::collections::HashMap;
use candle_core::{Device, Result, Tensor};
use std::fs;

fn main() {
    println!("GPT-2 rust style under construction!!");

    // Again just get the steps in.  We will organize later.

    // 1) Load the dataset
    let file_contents = fs::read_to_string("input.txt");
    //println!("input.txt content=\n{:?}", file_contents);

    // Get the unique characters in the dataset
    let text = file_contents.unwrap();
    let mut chars: Vec<char> = text.chars().collect();
    chars.sort();
    chars.dedup();

    println!("Unique characters in the dataset: {:?}", chars);

    // 2) Create the simple tokenizer
    let str_to_int: HashMap<char, usize> = chars.into_iter().enumerate().map(|(i, ch)| (ch, i)).collect();
    let int_to_str: HashMap<usize, char> = str_to_int.iter().map(|(ch, i)| (*i, *ch)).collect();

    // 3) Encode the dataset


    // 4) Test/train validation split


    // 5) Build simple BigramModel (to start and learn)
    // 5b) Multi-head attention (later)


    // 6) Optimizer (take gradients, update based on loss)


    // 7) Training loop

}

fn encode(str_to_int: &HashMap<char, usize> ) -> String {
  String::from("encoded")
}

fn decode(int_to_str: &HashMap<usize, char>) -> String {
  String::from("decoded")
}