// if2.rs

// Step 1: Make me compile!
// Step 2: Get the bar_for_fuzz and default_to_baz tests passing!
// Execute `rustlings hint if2` or use the `hint` watch subcommand for a hint.


pub fn fizzish_match(fizzish: &str) -> &str {
    match fizzish {
        "fizz" => "foo",
        "fuzz" => "bar",
        _ => "baz"
    }
}

// No test changes needed!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo_for_fizz() {
        assert_eq!(fizzish_match("fizz"), "foo")
    }

    #[test]
    fn bar_for_fuzz() {
        assert_eq!(fizzish_match("fuzz"), "bar")
    }

    #[test]
    fn default_to_baz() {
        assert_eq!(fizzish_match("literally anything"), "baz")
    }
}
