use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::{BufWriter, Write};
use std::io::prelude::*;

fn main() {
    let reached_score = loop {
    println!("Enter reached score:");
    let mut reached_score = String::new();
    io::stdin()
        .read_line(&mut reached_score)
        .expect("not valid input");

    let _reached_score: i32 = match reached_score.trim().parse(){
        Ok(num) => break num,
        Err(_) => continue,
        };
    };
    // to do : extract vector data from existing file.
    let mut scores: Vec<i32> = vec![1,2];
    scores.push(reached_score);
    score_statistics(&mut scores);
    improved(&mut scores);
    println!("The average score is {}", score_avg(&mut scores));
    scores_data_read();
    let reached_score_as_string: String = reached_score.to_string();
    scores_data_write(&reached_score_as_string);
}

fn score_statistics(vector: &mut Vec<i32> ){
    score_avg(vector);
}

fn score_avg(vector: &mut Vec<i32>) -> f64{
    let sum: i32 = vector.iter().sum();
    let average = sum as f64 / vector.len() as f64; 
    return average;
}

fn improved(vector: &mut Vec<i32>) -> () {
    let last_element = vector.last().unwrap();
    let last_element = *last_element as f64;
    let improvement = last_element - score_avg(vector);
    if last_element > score_avg(vector){
        println!("You improved by {}", improvement);
    } else {
        println!("You did not improve");
    }
}

fn scores_data_read() {
    let mut data = String::new();
    let file = File::open("src/vector_data.txt")
        .expect("Unale to open file");
    let mut reader = BufReader::new(file);
    reader.read_to_string(&mut data).expect("could not read data");
    let vector_data = data.parse::<i32>().unwrap();

    
    println!("{}", vector_data);
    //todo: save txt data in vector
}

//needs to be checked 
fn scores_data_write(string: &str) -> i32{
    let data = String::new();
    let file = File::open("src/vector_data.txt")
        .expect("Unale to open file");
    let mut file = BufWriter::new(file);
    file.write_all(data.as_bytes()).expect("unable to write");
    let input_score = data.parse::<i32>().unwrap();
    return input_score;
}