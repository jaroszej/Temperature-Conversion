use std::io;
use std::io::prelude::*;

fn farenheit_to_celsius(farenheit: f64) {
    let farenheit = &farenheit;
    let conversion = (farenheit - 32.0)/2.0;
    println!("{} C°", conversion);
}

fn celsius_to_farenheit(celsius: f64 ) {
    let celsius = &celsius;
    let conversion = (celsius * 1.8) + 32.0;
    println!("{} F°", conversion);
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // allows cursor to stay at the end of the line, manual flush
    write!(stdout, "Press 'Enter' to exit. . .").unwrap();
    stdout.flush().unwrap();

    // read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() {
    println!("Enter a temperature. . .");

    let mut input = String::new();

    io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
    
    let temp = &input.trim().parse::<f64>().unwrap();
    
    let mut i = 0;

    while i < 1 {
        
        println!("Enter (1) to convert F° to C°.\nEnter (2) to convert C° to F°.");

        let mut choice = String::new();
    
        io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read input.");

        if choice.trim() == "1" {
            i = 1;
            farenheit_to_celsius(*temp);
        } else if choice.trim() == "2" {
            i = 1;
            celsius_to_farenheit(*temp);
        } else {
            println!("Invalid input. . .");
        }
    }

    pause();    
}  