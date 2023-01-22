use std::io;

fn another_function(x: i32) {
    println!("Der Wert von x ist: {x}");
}

fn five() -> i32 {
    5 //Rückgabewert darf kein Semikolon haben, sonst Fehler!
}

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

    //Parse
    let guess: u32 = "42".parse().expect("Keine Zahl!");

    //Tupel
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("Der Wert von y ist: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    //Array
    let a = [1, 2, 3, 4, 5];

    let months = ["Januar", "Februar", "März", "April", "Mai", "Juni", "Juli",
              "August", "September", "Oktober", "November", "Dezember"];

    let a: [i32; 5 /* anzahl der Elemente*/ ] = [1, 2, 3, 4, 5]; 

    let a = [3; 5]; // = let a = [3, 3, 3, 3, 3];
    
    //Zugriff auf Array-Elemente
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    //Var mit Rückgabe
    let y = {
        let x = 3;
        x + 1
    };

    println!("Der Wert von y ist: {y}");


    //
    let condition = true;
    let number = if condition { 5 } else { 6 }; //let in einer If-Anweisung

    println!("Der Wert der Zahl ist: {number}");

    //Rückgabe von Werten von Schleifen
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Das Ergebnis ist {result}");

    //Schleifen kann man auch einen Namen zuweisen
    let mut count = 0;
    'counting_up: loop {
        println!("Zähler = {count}");
        let mut remaining = 10;

        loop {
            println!("Restliche = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("Zähler-Endstand = {count}");

}
