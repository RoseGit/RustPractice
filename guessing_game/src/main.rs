use std::io;

fn main() {
    println!("Guess the number!");

    /*Generamos un numero aleatorio*/
    let secret_number = rand::random_range(0..99);
    //println!("The secret number is {}", secret_number);

    println!("Please input your guess.");

    /*
    Usamos let para crear una variable
    Por default en rust, las variables son inmutables
    para permitir que una variable cambie su valor se le agrega la palabra reservada mut
    */
    let mut guess = String::new();

    /*
    io::stdin() es un objeto que permite manejar la entrada estandar de la terminal
    read_line(&mut guess) es un metodo de la entrada estandar para manejar la estrada del usuario
    y le decimos que el valor introducido lo guarde en la variable guess (&mut guess)
    El simbolo & indica que el argumento que se le pasa a read_line es una referencia, es decir
    no copia varias veces el valor en un codigo grande, por tanto no ocupa demasiada memoria y al
    igual que las variables por default las referencias son inmutables

    expect("Failed to read line"); En esta linea se devuelve un Enum de tipo Result que contiene
    dos variantes un Ok(indica que la operacion fue exitosa) y un Err( contiene informacion
    acerca de como o porque la operacion fallo), para nuestro ejemplo si algo falla en la entrada
    del  usuario, se nos mostraría "Failed to read line"

    IMPORTANTE: si no agregamos el metodo .expect el programa compilara pero arrojara una WARNING
    */
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    /*Imprimimos en terminal el valor que dio entrada el usuario.*/
    println!("You guessed: {guess}", );

}
