extern crate hex;

fn hex_str(str: &str) -> String {
    return hex::encode(str);
}

fn algorithm(str: &str) -> String {
    let hexed_string = hex_str(str);
    let mut sum: u32 = hexed_string.bytes().map(u32::from).sum();

    for (i, e) in str.chars().enumerate() {
        let hex_char = hex_str(&e.to_string());
        sum += hex_char.bytes().map(u32::from).sum::<u32>() / (i as u32 + 1);
    }

    return hex_str(&sum.to_string());
}

fn main() {
    let strings_to_test = ["stRing", "Hello, World!", "Huh?", "THIS IS A VERY LONG STRING LIKE REALLY REALLY ReALlY long", "algorIthm"];

    for i in strings_to_test {
        println!("{}", algorithm(i));
        println!("{}", algorithm(&i.to_lowercase()));
        println!("{}\n", algorithm(&i.to_uppercase()));
        
    }
}

mod test {
    use crate::{algorithm, hex_str};

    #[test]
    fn test_1() {
        println!("{}", hex_str("a"))
    }

    #[test]
    fn test_2() {
        println!("{}", algorithm("Hello"))
    }
}
