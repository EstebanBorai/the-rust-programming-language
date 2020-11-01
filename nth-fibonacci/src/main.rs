use utils::input_int;

/// Calculates the nth fibonacci number
fn fibonacci(nth: i32) -> i32 {
    if nth <= 0 {
        panic!("Invalid value, value must be an integer greather than 0")
    }

    match nth {
        1 => 0,
        2 => 1,
        _ => fibonacci(nth - 1) + fibonacci(nth - 2)
    }
}

fn main() {
    let value = input_int(Some("Enter the nth fibonacci number expected: "));

    println!("{}", fibonacci(value));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_perfect_square(n: f32) -> bool {
        let square_root = n.sqrt();

        square_root.powi(2) == n
    }

    fn is_fibonacci(n: f32) ->  bool {
        is_perfect_square(5.0 * n * n + 4.0) || is_perfect_square(5.0 * n * n - 4.0)
    }


    #[test]
    pub fn it_generates_valid_fibonacci() {
        for n in 1..=10 {
            let fibo = fibonacci(n);
            println!("{}", fibo);

            assert_eq!(is_fibonacci(fibo as f32), true);
        }
    }

    #[test]
    pub fn it_generates_fibonacci_sequence() {
        let sequence = vec![
            0,
            1,
            1,
            2,
            3,
            5,
            8,
            13,
            21,
            34,
            55,
            89,
            144
        ];

        for n in 0..sequence.len() {
            let fibo = fibonacci((n + 1) as i32);
            println!("{}:{}", fibo, sequence[n]);

            assert_eq!(fibo, sequence[n]);
        }
    }
}
