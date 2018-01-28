//! An iterator is created by the split_whitespace method on str.

use std::str;

type TokenIterator<'a> = str::SplitWhitespace<'a>;

pub fn tokenize(text: &str) -> TokenIterator {
    return text.split_whitespace();
}

// testing to see if tokenize does what it's supposed to.
#[cfg(test)]
mod tests {
    use screen::Interfaceable;
    use screen::Screen;
    use super::*;

    #[test]
    fn tokenize_test() {
        let mut src = Screen::new();
        let test_tokens = tokenize("This is a test string.");
        for val in test_tokens {
            src.print(&format!("Iterator {}", val));
        }
        src.print("Please type something, hopefully with spaces.");
    }

    #[test]
     fn tokenize_test_v2() {
         let mut src = Screen::new();
         let test_str = src.prompt();
         let test_t = tokenize(&test_str);
         for val in test_t {
             src.print(&format!("Iterator {}", val));
         }
     }
}
