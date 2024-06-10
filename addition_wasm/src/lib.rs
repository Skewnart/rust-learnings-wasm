#[no_mangle]
pub extern "C" fn addition(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = addition(2, 2);
        assert_eq!(result, 4);
    }
}
