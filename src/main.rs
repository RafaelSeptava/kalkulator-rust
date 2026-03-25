mod kalkulator;
use kalkulator::*;

fn main() {
    println!("{:-^30}", "Kalkulator");

    let angka_pertama = input_angka("Masukan angka pertama:");
    let operator = input_operator();
    let angka_kedua = input_angka("Masukan angka kedua:");

    let hasil: f64;

    match operator {
        '+' => hasil = tambah(angka_pertama, angka_kedua),
        '-' => hasil = kurang(angka_pertama, angka_kedua),
        '*' => hasil = kali(angka_pertama, angka_kedua),
        '/' => hasil = bagi(angka_pertama, angka_kedua),
        _ => unreachable!(),
    }
        
    println!("\nHasil: {}", hasil);
}