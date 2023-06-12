use std::fmt::Display;

// Print any type that implements the ToString trait to stdout
pub fn info_other(data: impl ToString) {
    println!("{}", data.to_string());
}

// Print any type that implements the Display trait
pub fn info(data: impl Display) {
    println!("{}", data);
}

pub fn info_u8(data: Vec<u8>) {
    println!("{}", String::from_utf8(data).unwrap());
}

pub fn info_u8s<T: From<T> + Into<Vec<u8>>>(data: T) {
    let text: Vec<u8> = data.into();
    println!("{}", String::from_utf8(text).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
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

    #[test_case("you have no more ram")]
    #[test_case(String::from("bloop"))]
    #[test_case(CString::new("blow").unwrap())]
    pub fn print_info_cstring<T: From<T> + Into<Vec<u8>>>(data: T) {
        info_u8s(data);
    }
}
