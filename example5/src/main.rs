fn main() {

    let user1 = User{
        email: String::from("someone@hola.com"),
        username: String::from("alguien"),
        active: true,
        sign_in_count: 1,
    };
    //user1.email = String::from("pruebas@gmail.com");
    let rectangle = Rectangle{
        width: 30,
        height: 20,
    };
    println!("El área del rectángulo es {}", area(&rectangle));
    println!("El rectángulo es {:?}", rectangle);
    println!("Otra forma de obtener el área {}", rectangle.area());
    println!("rectangle es un cuadrado: {}", rectangle.is_square());
    let rect2 = Rectangle::square(25);
    println!("Rectangle puede almacenar rect: {}", rectangle.can_hold(&rect2));
}



struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Se agrega el derive para el pretty printing.
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}


impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Constructor de cuadrados
    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }

}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
