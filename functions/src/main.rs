fn main() {
    println!("Hello, world!");
    another_function();
    another_function_with_parameters(52);
    print_labeled_measurement(5,'h');

    let x = five();
    println!("Valor de x: {x}");

    let y = plus_one(5);
    println!("Valor de y: {y}");
}

fn another_function(){
    println!("Another function.");
}

fn another_function_with_parameters(x: i32){
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

/*the expresions return the value 5,
Note: Expressions no llevan ; al final */
fn five()-> i32{
    5
}

fn plus_one(x: i32) -> i32{
    x + 1
}