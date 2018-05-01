pub mod image;

/// Result type that reports STB errors
pub type StbResult<T> = std::result::Result<T, String>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
