fn main() {
    // Tuple Type
    tuple_type();
    tuple_type_destruction();
    tuple_type_period_access();

    // Array Type
    array_type();
    array_type_access();
}

fn tuple_type() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup = {:#?}", tup);
}

fn tuple_type_destruction() {
    let tup = (500, 6.4, 1);

    // similar a js
    let (x, y, z) = tup;

    println!("O valor de x é: {x}");
    println!("O valor de y é: {y}");
    println!("O valor de z é: {z}");
}

fn tuple_type_period_access() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
    println!("five_hundred = {five_hundred}");
    println!("six_point_four = {six_point_four}");
    println!("one = {one}");
}

fn array_type() {
    let a = [1, 2, 3, 4, 5];
    println!("a = {:#?}", a);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("months = {:#?}", months);

    // tipo e tamanho; tipo i32 com 5 elementos
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a[i32; 5] = {:#?}", a);

    let a = [3; 5];
    println!("a[3; 5] = {:#?}", a);
}

fn array_type_access() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    println!("first = {first}; second = {second};");
}