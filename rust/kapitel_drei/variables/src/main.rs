use std::io;

fn main() {
    let mut x = 5;
    println!("Der Wert von x ist: {x}");
    x = 6;
    println!("Der Wert von x ist: {x}");

    //Beschatten
    //https://rust-lang-de.github.io/rustbook-de/ch03-01-variables-and-mutability.html
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("Der Wert von x im inneren Gültigkeitsbereich ist: {x}");
    }

    println!("Der Wert von x ist: {x}");

    //Tupel
    let _x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = _x.0;
    let _six_point_four = _x.1;
    let _one = _x.2;

    //Ungültiger Array Zugriff
    let a = [1, 2, 3, 4, 5];

    println!("Bitte gib einen Array-Index ein.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Fehler beim Lesen der Zeile");

    let index: usize = index
        .trim()
        .parse()
        .expect("Eingegebener Index war keine Zahl");

    let element = a[index];

    println!("Der Wert von element beim Index {index} ist: {element}");
}
