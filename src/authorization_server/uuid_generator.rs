use uuid::Uuid;
use authorization_server::IdGenerator;

simple_generator!(pub UuidGenerator Uuid::new_v4().simple().to_string());

#[cfg(test)]
mod tests {
    extern crate regex;
    use super::*;

    #[test]
    fn should_be_alphanumeric() {
        let instance = UuidGenerator::instance();
        let id = instance.create_id();
        let tester = regex::Regex::new("^[A-Za-z0-9]+$").expect("Failed to create regex");
        assert!(tester.is_match(&id));
    }
}