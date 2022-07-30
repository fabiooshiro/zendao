fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        // "The value of x in the inner scope is: {x}"
        println!("O valor de x no escopo interno é: {x}");
    }

    // "The value of x is: {x}"
    println!("O valor de x é: {x}");
}
