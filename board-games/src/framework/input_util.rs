pub mod convert_input {
    use std::str::FromStr;

    pub fn yes_or_no(answer: &str) -> Result<bool, String> {
        let answer = answer.trim();
        if answer == "y" {
            Ok(true)
        } else if answer == "n" {
            Ok(false)
        } else {
            Err(format!("入力が不正です: {}", answer))
        }
    }

    pub fn accept_empty_parse<T: FromStr>(answer: &str) -> Result<Option<T>, String> {
        if answer.is_empty() {
            Ok(None)
        } else if let Ok(parsed) = answer.trim().parse::<T>() {
            Ok(Some(parsed))
        } else {
            Err(format!("入力が不正です: {}", answer))
        }
    }

    pub fn simple_parse<T: FromStr>(answer: &str) -> Result<T, String> {
        if let Ok(parsed) = answer.trim().parse::<T>() {
            Ok(parsed)
        } else {
            Err(format!("入力が不正です: {}", answer))
        }
    }
}
