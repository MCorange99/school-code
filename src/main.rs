use std::io::Write;


fn uzduotis_1(x: bool) {
    println!("=== Uzduotis 1 ===");
    
    match x {
        true => println!("true"),
        false => println!("false"),
    }
    println!("=== Uzduotis 1 ===");
}

fn uzduotis_2(x: usize) {
    println!("=== Uzduotis 2 ===");
    println!("Number: {x}");

    match x {
        _ if x > 10 => println!("Daugiau"), 
        _ if x < 10 => println!("Maziau"), 
        _  => println!("Lygu"), 
    }

    
    println!("=== Uzduotis 2 ===");
}

fn uzduotis_3() {
    println!("=== Uzduotis 3 ===");
    let z = get_input_int("iveskite z: ");

    if z - 1 >= 0 {
        let fx = ((z - 1) as f64).powf(0.5);
        println!("z  = {z}");
        println!("fx = {fx}");
    } else {
        println!("z = {z}");
        println!("Funkcija neegzistuoja");
    }
    println!("=== Uzduotis 3 ===");
}

fn uzduotis_4() {
    println!("=== Uzduotis 4 ===");
    let x = get_input_int("iveskite x: ");
    let y = get_input_int("iveskite y: ");
    
    let f = (y*y - y*2*x + x*x) / (x*x*x - y);

    println!("f({x}, {y}) = {f}");
    
    println!("=== Uzduotis 4 ===");
}

fn uzduotis_5() {
    println!("=== Uzduotis 5 ===");
    let a = get_input_int("iveskite a: ") as f64;
    let x = get_input_int("iveskite x: ") as f64;

    let e = std::f64::consts::E;
    
    let f =  if x <= 0.0 {
        a*e.powf(-x)
    } else if 0.0 < x &&  x < 1.0 {
        5.0 * a * x
    } else {
        (x+1.0).sqrt()
    };

    println!("a = {a}\nf({x}) = {f}");

    println!("=== Uzduotis 5 ===");
}

fn uzduotis_6(plot: ((f64, f64), (f64, f64))) {
    println!("=== Uzduotis 6 ===");
    let x = get_input_int("iveskite cordinate x: ") as f64;
    let y = get_input_int("iveskite cordinate y: ") as f64;

    if x < plot.0.0 ||
       x > plot.1.0 ||
       y > plot.0.1 ||
       y < plot.1.1 {
        println!("Outside");
    } else {
        println!("Inside");
    }
    println!("=== Uzduotis 6 ===");
}

fn uzduotis_7() {
    println!("=== Uzduotis 7 ===");
    let a = get_input_int("a:");
    let b = get_input_int("b:");
    let c = get_input_int("c:");

    if a < 1 || b < 1 || c < 1 {
        println!("Blogi matmenys");
        return;
    }
    println!("Plytos   Turis:  {}", a * b * c);
    println!("Pavirsio Plotas: {}", 2*(a*b+b*c+c*a));
    println!("Briaunu  ilgis:  {}", 4*(a+b+c));
    println!("=== Uzduotis 7 ===");
}

fn uzduotis_8() {
    println!("=== Uzduotis 8 ===");
    let n = get_input_int("n:");
    if n % 2 == 0 {
        println!("Lyginis");
    } else {
        println!("Ne lyginis")
    }
    println!("=== Uzduotis 8 ===");
}

fn uzduotis_9() {
    println!("=== Uzduotis 9 ===");
    let n = get_input_int("n:");
    for i in 1..=n {
        println!("owo {i}!");
    }
    println!("=== Uzduotis 9 ===");
}

fn uzduotis_10(){
    println!("=== Uzduotis 10 ===");
    let s = get_input_str("Iveskite betkoki sakini: ");
    let mut s = s.as_str();
    
    println!("'{s}'");
    while s.len() > 1 {
        s = &s[1..s.len()-1];
        println!("'{s}'");
    }
    println!("=== Uzduotis 10 ===");
}

fn get_input_int(s: &str) -> isize {
    let mut inp = String::new();
    print!("{s}");
    let _ = std::io::stdout().flush();

    std::io::stdin()
        .read_line(&mut inp)
        .expect("error: unable to read user input");

    match inp.trim().parse::<isize>() {
        Ok(i) => i,
        Err(_) => {
            println!("Unable to parse number, try again");
            get_input_int(s)
        }
    }
        
}

fn get_input_str(s: &str) -> String {
    let mut inp = String::new();
    print!("{s}");
    let _ = std::io::stdout().flush();

    std::io::stdin()
        .read_line(&mut inp)
        .expect("error: unable to read user input");
    inp.trim().to_string()
}


fn main() {
    uzduotis_1(true);
    uzduotis_2(69);
    uzduotis_3();
    uzduotis_4();
    uzduotis_5();
    uzduotis_6((
        (-1.0, 1.0),
        (1.0, -1.0)
    ));
    uzduotis_7();
    uzduotis_8();
    uzduotis_9();
    uzduotis_10();
    //* do while neegzistuoja cia nes jo nereikia
    
}
