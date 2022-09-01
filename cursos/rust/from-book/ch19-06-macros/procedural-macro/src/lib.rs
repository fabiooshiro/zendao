//use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
