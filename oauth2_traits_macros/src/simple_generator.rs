#[macro_export]
macro_rules! simple_generator {
    ( $name:ident $body:expr ) => {
        struct $name {}
        impl IdGenerator for $name {
            fn instance() -> Self {
                $name {}
            }

            fn create_id(&self) -> String {
                $body
            }
        }
    };
    ( pub $name:ident $body:expr ) => {
        pub struct $name {}
        impl IdGenerator for $name {
            fn instance() -> Self {
                $name {}
            }

            fn create_id(&self) -> String {
                $body
            }
        }
    };
}

#[cfg(test)]
mod tests {
    trait IdGenerator { // Copy trait for test to avoid dep on parent
        fn instance() -> Self;
        fn create_id(&self) -> String;
    }

    #[test]
    fn simple_simple_gen() {
        simple_generator!(FooGenerator String::from("foo"));
        assert_eq!(&FooGenerator::instance().create_id(), "foo");
    }

    #[test]
    fn simple_generated() {
        simple_generator!(BarGenerator {
            let foo = 15;
            let bar = 45;
            format!("{}", foo + bar)
        });

        assert_eq!(&BarGenerator::instance().create_id(), "60");
    }
}