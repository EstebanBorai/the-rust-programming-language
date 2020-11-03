//! Convert temperatures between Fahrenheit and Celsius.

use utils::{input, input_float, input_int};

fn to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32_f32) * 0.555
}

fn to_fahrenheit(celsius: f32) -> f32 {
    (celsius * 1.8) + 32_f32
}

fn main() {
    println!("[1]: Convert from °F to °C");
    println!("[2]: Convert from °C to °F");

    let option = input_int(Some("Select a conversion mode: "));
    match option {
        1 => {
            let value_to_convert = input_float(Some("Enter input value: "));

            println!("{}°F = {}°C", value_to_convert, to_celsius(value_to_convert))
        },
        2 => {
            let value_to_convert = input_float(Some("Enter input value: "));

            println!("{}°F = {}°C", value_to_convert, to_fahrenheit(value_to_convert))
        },
        _ => println!("Invalid option provided {}", option)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_converts_fahrenheit_to_celsius_32f() {
        let celsius = to_celsius(32_f32);

        assert_eq!(celsius, 0_f32);
    }

    #[test]
    pub fn it_converts_fahrenheit_to_celsius_64f() {
        let celsius = to_celsius(64_f32);

        assert_eq!(celsius, 17.76);
    }

    #[test]
    pub fn it_converts_celsius_to_farenheit_32c() {
        let fahrenheit = to_fahrenheit(32_f32);

        assert_eq!(fahrenheit, 89.6);
    }

    #[test]
    pub fn it_converts_celsius_to_farenheit_64c() {
        let fahrenheit = to_fahrenheit(64_f32);

        assert_eq!(fahrenheit, 147.2);
    }
}
