
pub trait ToTwoDigits {
    fn to_two_digits(&self) -> String;
}

impl ToTwoDigits for u32 {
    fn to_two_digits(&self) -> String {
        let value = format!("{}", *self);
        match value.len() == 1 {
            true  => "0".to_string() + &value,
            false => value
        }
    }
}