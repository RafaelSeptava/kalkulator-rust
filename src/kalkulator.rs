use std::io;

pub fn tambah(a: f64, b: f64) -> f64 {
    a + b
}

pub fn kurang(a: f64, b: f64) -> f64 {
    a - b
}

pub fn kali(a: f64, b: f64) -> f64 {
    a * b
}

pub fn bagi(a: f64, mut b: f64) -> f64 {
    while b == 0.0 {
        println!("\nTidak bisa membagi dengan nol!");
        b = input_angka("Masukan angka kedua:");
    }

    a / b
}

pub fn input_angka(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca input!");

        match input.trim().parse() {
            Ok(angka) => return angka,
            Err(_) => {
                println!("\nHanya bisa memasukan angka!");
                continue;
            }
        }
    }
}

pub fn input_operator() -> char {
    loop {
        println!("Masukan operator(+, -, *, /):");

        let mut operator = String::new();
        io::stdin().read_line(&mut operator).expect("Gagal membaca input!");

        let operator = operator.trim().chars().next();

        match operator {
            Some('+') | Some('-') | Some('*') | Some('/') => return operator.unwrap(),
            _ => {
                println!("\nHanya bisa memasukan 4 operator!");
                continue;
            }
        }
    }
}