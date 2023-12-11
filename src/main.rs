use std::io::Write;
use school_code::{add_task, run_task};


fn uzduotis_2() {
    let x = get_input_int("iveskite x: ");

    match x {
        _ if x > 10 => println!("Daugiau"), 
        _ if x < 10 => println!("Maziau"), 
        _  => println!("Lygu"), 
    }

}

fn uzduotis_3() {
    let z = get_input_int("iveskite z: ");

    if z - 1 >= 0 {
        let fx = ((z - 1) as f64).powf(0.5);
        println!("z  = {z}");
        println!("fx = {fx}");
    } else {
        println!("z = {z}");
        println!("Funkcija neegzistuoja");
        return;
    }
}

fn uzduotis_4() {
    let x = get_input_int("iveskite x: ");
    let y = get_input_int("iveskite y: ");
    
    let f = (y*y - y*2*x + x*x) / (x*x*x - y);

    println!("f({x}, {y}) = {f}");
}

fn uzduotis_5() {
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

}

fn uzduotis_6() {
    let plot = (
        (-1.0, 1.0),
        (1.0, -1.0)
    );

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
}

fn uzduotis_7() {
    let a = get_input_int("a:");
    let b = get_input_int("b:");
    let c = get_input_int("c:");

    if a < 1 || b < 1 || c < 1 {
        println!("Blogi matmenis");
        return;
    }
    println!("Plytos   Turis:  {}", a * b * c);
    println!("Pavirsio Plotas: {}", 2*(a*b+b*c+c*a));
    println!("Briaunu  ilgis:  {}", 4*(a+b+c));

}

fn uzduotis_8() {
    let n = get_input_int("n:");
    if n % 2 == 0 {
        println!("Lyginis");
    } else {
        println!("Ne lyginis")
    }

}

fn uzduotis_9() {
    let n = get_input_int("n:");
    for i in 1..=n {
        println!("owo {i}!");
    }

}

fn uzduotis_10() {
    let s = get_input_str("Iveskite betkoki sakini: ");
    let mut s = s.as_str();
    
    println!("'{s}'");
    while s.len() > 1 {
        s = &s[1..s.len()-1];
        println!("'{s}'");
    }

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


fn main(){
    
    //?\ Add inline task
    add_task!(1, || {
        let x = true;
        match x {
            true => println!("true"),
            false => println!("false"),
        };
    });

    //? Add task by pointer
    add_task!(2, uzduotis_2);
    add_task!(3, uzduotis_3);
    add_task!(4, uzduotis_4);
    add_task!(5, uzduotis_5);
    add_task!(6, uzduotis_6);
    add_task!(7, uzduotis_7);
    add_task!(8, uzduotis_8);
    add_task!(9, uzduotis_9);
    add_task!(10, uzduotis_10);

    //* do while neegzistuoja cia nes jo nereikia

    //? Run all tasks
    run_task!(all);
    //? Run one task
    // run_task!(1);
}
