fn main() {
    // aqui é necessário colocar o tipo "u32" para fazer o parse
    let guess: u32 = "42".parse().expect("Não é um número!"); // Not a Number
    println!("guess = {guess}");

    // Ponto flutuante
    floating_point();

    // Operações matemáticas
    math_operations();

    // Boolean
    boolean_type();

    // Caractere
    char_type();
}

fn floating_point() {
    println!("Floating point");
    let x = 2.0; // f64 por default/padrão

    let y: f32 = 3.0; // f32
    println!("  x = {x}; y = {y}");
}

fn math_operations() {
    println!("Operations");
    // adição ~ addition
    let sum = 5 + 10;
    println!("  sum = {sum}");

    // subtração ~ subtraction
    let difference = 95.5 - 4.3;
    println!("  difference = {difference}");

    // multiplicação ~ multiplication
    let product = 4 * 30;
    println!("  product = {product}");

    // divisão ~ division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    println!("  quotient = {quotient}; floored = {floored}");

    // resto ~ remainder
    let remainder = 43 % 5;
    println!("  remainder = {remainder}");
}

fn boolean_type() {
    println!("Boolean");
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("  t = {t}; f = {f}");
}

fn char_type() {
    println!("Char");
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("  c = {c}");
    println!("  z = {z}");
    println!("  heart_eyed_cat = {heart_eyed_cat}");
}
