//use std::str::FromStr;


fn main() {
    //println!("Hello, world!");
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("s vale {}", s);
    // Move (tipos no básicos).
    let s1 = String::from("hello");
    let s2 = s1; // Move de s2 a s1.
    //println!("s1 vale {}", s1); // <- Compiler error.

    let s1 = String::from("hello");
    let s2 = s1.clone(); // Hace un deep copy.
    println!("s1 vale {} y s2 vale {}", s1,s2);

    let s = String::from("Hola");
    take_ownership(s); // Se transfiere ownership, ya no se puede usar s.
    //println!("s afuera vale {}",s); // Error!

    let x = 45;
    make_copy(x);
    println!("Todavía puedo usar x que vale {}",x);

    let s = String::from("alo!");
    let s = take_and_give_ownership(s);
    println!("s valiendo afuera {}", s);

    // References (passing values without taking ownership).
    let s = String::from("Hola Mundo!");
    let length = string_length(&s);
    println!("La cadena {} tiene longitud {}", s, length );

    // Mutable references.
    let mut s = String::from("Hola");
    change(&mut s);
    println!("s vale {}", s);

    // Only one mutable reference per scope.
    let r1 = &mut s; // OK.
    let r2 = &mut s; // Problemático.

    //println!("r1 y r2 valen {},{}", r1,r2); //// ERROR.

    // Solución: uso de scopes.
    let mut s = String::from("Hola");
    {
        let r1 = &mut s;
        r1.push_str(" mundo");
    }
    let r2 = &mut s;
    r2.push_str("!!");
    println!("s vale {}", s);

    // Slices.
    let s = String::from("hello World!");
    let slice = &s[0..4];
    println!("Slice vale {}", slice);
    let first = first_word_redux(&s);
    println!("First word is {}", first);
    println!("2) First word is {}", first_word_redux2("goodbye people!"));
}


fn take_ownership(s: String) {
    println!("s vale {}", s);
}

fn make_copy(x: i32) {
    println!("x vale {}", x);
}

fn take_and_give_ownership(s: String) -> String {
    println!("s adentro vale {}", s);
    s
}

fn string_length(s: &String) -> usize {
    s.len()
}// s no se pierde aquí pues no había ownership de esa variable por el &.

// * es para dereferenciar..

fn change(s: &mut String)  {
    s.push_str(" mundo!!");
}

/*
fn dangle() -> &String{
    let mut s = String::from("Hola");
    &s // PELIGRO pues s queda dealocado!
}
*/

fn no_dangle() -> String{
    let mut s = String::from("Hola");
    s // PELIGRO pues s queda dealocado!
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }
    s.len()
}


fn first_word_redux(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word_redux2(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

