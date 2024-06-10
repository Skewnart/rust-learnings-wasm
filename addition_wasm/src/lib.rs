// ## Other crates : 
// wasm-bindgen         // Meilleure interaction entre le rust et le JS (manipulation de DOM)
// console_log          // log autres que les panics
// wasm-pack            // Intégration npm

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
