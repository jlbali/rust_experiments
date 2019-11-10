fn main() {
    // Tuple
    let tup = (3.0, 34, "Hello!");
    println!("Index 0: {}", tup.1);
    // Array
    let a = [1,5,19,20];
    //println!("A vale {}", a);
    println!("A[2] vale {}", a[2]);

    let x = 5;
    let y = {
        let x = 3;
        x + 1 // Sacar punto y coma para que sea una expresión y no un statement.
    };
    println!("y vale {}", y);
    let z = increment(y);
    println!("z vale {}", z);


    // Loop.
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break 2*counter;
        }
    };
    println!("result vale {}", result);

    // While.
    println!("Primer n de la suma de Gauss que supera 1050: {}", first_over(1050) );

    // For.
    let a = [10,20,30,40,50];
    let mut accum = 0;
    /*
    for x in a.iter() {
        accum += x;
    }
    */
    for x in a.iter() {
        println!("X vale {}", x);
    }

}


fn increment(n: i32) -> i32 {
    n + 1
}

fn first_over(n:i32) -> i32 {
    let mut counter = 0;
    let mut accum = 0;
    while accum < n {
        accum += counter;
        counter += 1;
    }
    counter
}
