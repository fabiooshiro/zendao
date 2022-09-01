#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let v = vec![1, 2, 3, 4];
        let result = v[3];
        println!("value = {}", result);
        assert_eq!(result, 4);
    }
}
