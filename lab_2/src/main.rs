#![allow(non_snake_case)]
use std::io::{
    self,
    stdin,
    stdout,
    Write
};

const C_BLUE: &str = "\x1B[34m";
const C_RESET: &str ="\x1B[0m";

fn run(name: &str, func: fn() -> io::Result<()>) -> io::Result<()> {
    println!("{C_BLUE}START {name}{C_RESET}");
    (func)()?;
    println!("{C_BLUE}END {name}{C_RESET}");
    Ok(())
}


fn L2_1() -> io::Result<()> {
    let mut ch_a: String = String::new();
    let mut ch_b: String = String::new();

    print!("Enter a character: "); stdout().flush()?;
    stdin().read_line(&mut ch_a)?;
    print!("Enter another character: "); stdout().flush()?;
    stdin().read_line(&mut ch_b)?;
    ch_a = ch_a.replace("\n\r", "").replace("\n", "");
    ch_b = ch_b.replace("\n\r", "").replace("\n", "");
    println!("'{ch_a}' == '{ch_b}' = {}", ch_a == ch_b);
    Ok(())
}

fn L2_2() -> io::Result<()> {
    // In rust the variables are not mutable by default, so you have to make them mutable manualy to change the value
    let mut ch_a: String = String::new();
    let mut ch_b: String = String::new();
    
    // Print with no newline
    print!("x: ");
    // Flush the stdin buffer so it outputs the text to console
    stdout().flush()?;
    // Read the input to the mutable ch_a variable 
    stdin().read_line(&mut ch_a)?;

    print!("y: ");
    // The ? operator will convert and push the error back to the caller function to be dealt with
    stdout().flush()?;
    stdin().read_line(&mut ch_b)?;

    // Strip the newlines and parse the String to a number, if it fails to parse it will default to 0
    let mut num_a = ch_a.replace("\n\r", "").replace("\n", "").parse::<i32>().unwrap_or(0);
    let mut num_b = ch_b.replace("\n\r", "").replace("\n", "").parse::<i32>().unwrap_or(0);

    // swap the numbers
    (num_a, num_b) = (num_b, num_a);


    println!("x = {}", num_a);
    println!("y = {}", num_b);

    Ok(())
}

fn L2_3() -> io::Result<()> {
    let mut ch_a: String = String::new();
    
    print!("x: ");
    stdout().flush()?;
    stdin().read_line(&mut ch_a)?;

    // Strip the newlines and parse the String to a number, if it fails to parse it will default to 0
    let num_a = ch_a.replace("\n\r", "").replace("\n", "").parse::<i32>().unwrap_or(0);
    
    //i32::pow(2, 10);
    println!("{}", num_a);
    println!("{} {}", i32::pow(num_a, 3), i32::pow(num_a, 6));
    println!("{} {} {}", i32::pow(num_a, 6), i32::pow(num_a, 3), num_a);


    Ok(())
}

fn L2_4() -> io::Result<()> {
    let mut ch_a: String = String::new();
    let mut ch_b: String = String::new();
    let mut ch_c: String = String::new();
    
    print!("a: ");
    stdout().flush()?;
    stdin().read_line(&mut ch_a)?;

    print!("b: ");
    stdout().flush()?;
    stdin().read_line(&mut ch_b)?;
    
    print!("c: ");
    stdout().flush()?;
    stdin().read_line(&mut ch_c)?;

    let mut num_a = ch_a.replace("\n\r", "").replace("\n", "").parse::<i32>().unwrap_or(0);
    let mut num_b = ch_b.replace("\n\r", "").replace("\n", "").parse::<i32>().unwrap_or(0);
    let mut num_c = ch_c.replace("\n\r", "").replace("\n", "").parse::<i32>().unwrap_or(0);

    // swap the numbers
    (num_a, num_b, num_c) = (num_c, num_b, num_a);


    println!("a = {}", num_a);
    println!("b = {}", num_b);
    println!("c = {}", num_c);

    Ok(())
}

fn L2_5() -> io::Result<()> {
    let mut ch_a: String = String::new();
    let mut ch_b: String = String::new();
    let mut ch_c: String = String::new();
    let mut ch_d: String = String::new();
    
    print!("a: ");
    stdout().flush()?;
    stdin().read_line(&mut ch_a)?;

    print!("b: ");
    stdout().flush()?;
    stdin().read_line(&mut ch_b)?;
    
    print!("c: ");
    stdout().flush()?;
    stdin().read_line(&mut ch_c)?;

    print!("d: ");
    stdout().flush()?;
    stdin().read_line(&mut ch_d)?;

    let mut num_a = ch_a.replace("\n\r", "").replace("\n", "").parse::<i32>().unwrap_or(0);
    let mut num_b = ch_b.replace("\n\r", "").replace("\n", "").parse::<i32>().unwrap_or(0);
    let mut num_c = ch_c.replace("\n\r", "").replace("\n", "").parse::<i32>().unwrap_or(0);
    let mut num_d = ch_d.replace("\n\r", "").replace("\n", "").parse::<i32>().unwrap_or(0);

    // swap the numbers
    (num_a, num_b, num_c, num_d) = (num_d, num_c, num_b, num_a);


    println!("a = {}", num_a);
    println!("b = {}", num_b);
    println!("c = {}", num_c);
    println!("d = {}", num_d);

    Ok(())
}

fn L2_6() -> io::Result<()> {
    let mut ch_a: String = String::new();
    let mut ch_b: String = String::new();
    let mut ch_c: String = String::new();
    
    print!("m1: ");
    stdout().flush()?;
    stdin().read_line(&mut ch_a)?;

    print!("m2: ");
    stdout().flush()?;
    stdin().read_line(&mut ch_b)?;
    
    print!("R: ");
    stdout().flush()?;
    stdin().read_line(&mut ch_c)?;

    let m1 = ch_a.replace("\n\r", "").replace("\n", "").parse::<i32>().unwrap_or(0);
    let m2 = ch_b.replace("\n\r", "").replace("\n", "").parse::<i32>().unwrap_or(0);
    let r = ch_c.replace("\n\r", "").replace("\n", "").parse::<i32>().unwrap_or(1);

    const G: f64 = 6.673e-8;


    println!("F = {}", G * ((m1 * m2)/i32::pow(r, 2)) as f64);

    Ok(())
}

fn L2_7() -> io::Result<()> {
    let mut ch_a: String = String::new();
    let mut ch_b: String = String::new();
    let mut ch_c: String = String::new();
    
    print!("H: ");
    stdout().flush()?;
    stdin().read_line(&mut ch_a)?;

    print!("M: ");
    stdout().flush()?;
    stdin().read_line(&mut ch_b)?;
    
    print!("S: ");
    stdout().flush()?;
    stdin().read_line(&mut ch_c)?;

    let H = ch_a.replace("\n\r", "").replace("\n", "").parse::<i32>().unwrap_or(0);
    let M = ch_b.replace("\n\r", "").replace("\n", "").parse::<i32>().unwrap_or(0);
    let S = ch_c.replace("\n\r", "").replace("\n", "").parse::<i32>().unwrap_or(0);

    if H > 23 {println!("ERROR: hours are formated badly")};
    if M > 59 {println!("ERROR: minutes are formated badly")};
    if S > 59 {println!("ERROR: seconds are formated badly")};

    Ok(())
}

fn L2_8() -> io::Result<()> {
    let mut ch_a: String = String::new();
    let mut ch_b: String = String::new();

    
    print!("X: ");
    stdout().flush()?;
    stdin().read_line(&mut ch_a)?;

    print!("Y: ");
    stdout().flush()?;
    stdin().read_line(&mut ch_b)?;

    let x = ch_a.replace("\n\r", "").replace("\n", "").parse::<f32>().unwrap_or(0.0);
    let y = ch_b.replace("\n\r", "").replace("\n", "").parse::<f32>().unwrap_or(0.0);
    println!("is inside: {}", (y >= x && y >= -x && y <= 1.0));
    Ok(())
}

fn L2_9() -> io::Result<()> {
    let mut ch_a: String = String::new();
    let mut ch_b: String = String::new();

    
    print!("X: ");
    stdout().flush()?;
    stdin().read_line(&mut ch_a)?;

    print!("Y: ");
    stdout().flush()?;
    stdin().read_line(&mut ch_b)?;

    let x = ch_a.replace("\n\r", "").replace("\n", "").parse::<f32>().unwrap_or(0.0);
    let y = ch_b.replace("\n\r", "").replace("\n", "").parse::<f32>().unwrap_or(0.0);
    println!("x = {x}, y = {y}");

    println!("is inside: {}", (y >= -1.0 && y <= 1.0) && (x >= -1.0 && x <= 1.0));
    Ok(())
}

fn main()  -> io::Result<()> {
    run("L2.1", L2_1)?;
    run("L2.2", L2_2)?;
    run("L2.3", L2_3)?;
    run("L2.4", L2_4)?;
    run("L2.5", L2_5)?;
    run("L2.6", L2_6)?;
    run("L2.7", L2_7)?;
    run("L2.8", L2_8)?;
    run("L2.9", L2_9)?;
    
    Ok(())
}
