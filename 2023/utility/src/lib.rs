
pub mod input {
    pub mod data_input {
        pub use std::io::{self, BufRead};

        pub fn text_input(exit: String) -> String {
            let mut data = String::new();
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                let temp = line.unwrap();
                if temp == exit {
                    break;
                } else {
                    data.push_str(&temp);
                    data.push('\n');
                }
            }
            data
        }
    }
}
