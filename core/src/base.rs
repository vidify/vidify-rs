pub trait CardInfo {
    fn get_description(&self) -> &str;
    fn get_icon(&self) -> &str;
}
