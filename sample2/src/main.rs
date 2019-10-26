use std::io;

fn main() {
    println!("Adivine el número!");
    println!("Ingrese su adivinanza:");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("No se pudo leer línea");
    println!("Ha adivinado {}", guess);
}

