use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct ReallyGoodPancakes;

fn main() {
    ReallyGoodPancakes::hello_macro();
}
