use std::io::prelude::*;
use std::io;

fn main() {
    println!("Soal 1 : Concat String");
    print!("param1 = ");
    let mut par1 = String::new();
    std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut par1).unwrap();
  
    print!("param2 = ");
    let mut par2 = String::new();
    std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut par2).unwrap();

    let result = format!("{} {}", par1.trim(), par2.trim());
    println!(" ");
    println!("output = {}", result);
    println!(" ");

    println!("Soal 2 : The Remainder");
    print!("param1 = ");
    let mut par1 = String::new();
    std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut par1).unwrap();
  
    print!("param2 = ");
    let mut par2 = String::new();
    std::io::stdout().flush().unwrap();
    io::stdin().read_line(&mut par2).unwrap();

    let par1_num = par1.trim().parse::<i32>().unwrap();
    let par2_num = par2.trim().parse::<i32>().unwrap();

    let result = par1_num % par2_num;
    println!(" ");
    println!("output = {}", result);
}
