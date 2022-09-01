use attribute_like::route;
// #[route()]
pub fn add(left: usize, right: usize) -> usize {
    if left > right {
        println!("left is bigger");
    }
    println!("{}:{}", file!(), line!());
    println!("?");
    left + right
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(3, 2);
        assert_eq!(result, 5);
    }
}


fn main() {
    println!("Hello, world!");
}
