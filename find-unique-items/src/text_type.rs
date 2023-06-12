use std::fmt::Display;

// Print any type that implements the ToString trait to stdout
pub fn info_other(data: impl ToString) {
    println!("{}", data.to_string());
}

// Print any type that implements the Display trait
pub fn info(data: impl Display) {
    println!("{}", data);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("you have no more ram")]
    #[test_case(3)]
    fn print_info_other_succeeds(data: impl ToString) {
        info_other(data);
    }

    #[test_case("you have no more ram")]
    #[test_case(3)]
    fn print_info_succeeds(data: impl Display) {
        info(data);
    }
}
