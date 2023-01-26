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

fn get_str(prompt: &str) -> io::Result<String> {
    // In rust the variables are not mutable by default, so you have to make them mutable manualy to change the value
    let mut inp: String = String::new();
    // Print with no newline
    print!(prompt);
    // Flush the stdout buffer so it outputs the text to console
    stdout().flush()?; // The ? operator will convert and push the error back to the caller function to be dealt with
    // Read the input to the mutable inp variable which we mark as mutable and we borrow it (&)
    stdin().read_line(&mut inp)?;
    Ok(inp)
}

fn get_int(prompt: &str) -> io::Result<usize> {
    /**
     * we return the Ok type with the result usize. This is how rust deals with errors
     */
    Ok(
        // Strip the newlines and parse the String to a number, if it fails to parse it will default to 0
        get_str(prompt)?
                    .replace("\n\r", "") 
                    .replace("\n", "")
                    .parse::<i32>()
                    .unwrap_or(0)
    )
}

fn L2_1() -> io::Result<()> {
    ch_a = get_str("Enter character: ")?;
    ch_b = get_str("Enter another character: ")?;
    println!("'{ch_a}' == '{ch_b}' = {}", ch_a == ch_b);
    Ok(())
}

fn L2_2() -> io::Result<()> {
    let mut num_a = get_int("x: ")?;
    let mut num_a = get_int("y: ")?;

    (num_a, num_b) = (num_b, num_a);


    println!("x = {}", num_a);
    println!("y = {}", num_b);

    Ok(())
}

fn L2_3() -> io::Result<()> {
    let mut num_a = get_int("x: ")?;
    
    println!("{}", num_a);
    println!("{} {}", i32::pow(num_a, 3), i32::pow(num_a, 6));
    println!("{} {} {}", i32::pow(num_a, 6), i32::pow(num_a, 3), num_a);


    Ok(())
}

fn L2_4() -> io::Result<()> {
    let mut num_a = get_int("a: ")?;
    let mut num_b = get_int("b: ")?;
    let mut num_c = get_int("c: ")?;
    // swap the numbers
    (num_a, num_b, num_c) = (num_c, num_b, num_a);


    println!("a = {}", num_a);
    println!("b = {}", num_b);
    println!("c = {}", num_c);

    Ok(())
}

fn L2_5() -> io::Result<()> {
    let mut num_a = get_int("a: ")?;
    let mut num_b = get_int("b: ")?;
    let mut num_c = get_int("c: ")?;
    let mut num_d = get_int("d: ")?;


    // swap the numbers
    (num_a, num_b, num_c, num_d) = (num_d, num_c, num_b, num_a);


    println!("a = {}", num_a);
    println!("b = {}", num_b);
    println!("c = {}", num_c);
    println!("d = {}", num_d);

    Ok(())
}

fn L2_6() -> io::Result<()> {
    let mut m1 = get_int("m1: ")?;
    let mut m2 = get_int("m2: ")?;
    let mut r = get_int("r: ")?;
    const G: f64 = 6.673e-8;


    println!("F = {}", G * ((m1 * m2)/i32::pow(r, 2)) as f64);

    Ok(())
}

fn L2_7() -> io::Result<()> {

    let H = get_int("H: ")?;
    let M = get_int("M: ")?;
    let S = get_int("S: ")?;

    if H > 23 {println!("ERROR: hours are formated badly")};
    if M > 59 {println!("ERROR: minutes are formated badly")};
    if S > 59 {println!("ERROR: seconds are formated badly")};

    Ok(())
}

fn L2_8() -> io::Result<()> {


    let x = get_int("x: ")?;
    let y = get_int("y: ")?;
    println!("is inside: {}", (y >= x && y >= -x && y <= 1.0));
    Ok(())
}

fn L2_9() -> io::Result<()> {
    let x = get_int("x: ")?;
    let y = get_int("y: ")?;

    println!("is inside: {}", (y >= -1.0 && y <= 1.0) && (x >= -1.0 && x <= 1.0));
    Ok(())
}

fn L2_10() -> io::Result<()> {
    let x = get_int("Enter a number: ")?;

    println!("    {}    {}    {}    {}", i32::pow(r, 2), i32::pow(r, 3), i32::pow(r, 4), i32::pow(r, 5));

    Ok(())
}

fn L2_11() -> io::Result<()> {
    let x = get_int("kambario ilgis: ")?;
    let y = get_int("kabario plotis: ")?;
    println!("kambario plotas: {}", x*y);    
    let k = get_int("plyteliu 1 m2 kaina: ")?;

    println!("visko kaina: {}", x*y*k);
    Ok(())
}

fn L2_12() -> io::Result<()> {
    let x1 = get_int("x1: ")?;
    let x1 = get_int("x2: ")?;
    let y1 = get_int("y1: ")?;
    let y1 = get_int("y2: ")?;


    println!("atstumas tar 2 tasku: {}", ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt());
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
    run("L2.10", L2_10)?;
    run("L2.11", L2_11)?;
    run("L2.12", L2_12)?;
    
    Ok(())
}
