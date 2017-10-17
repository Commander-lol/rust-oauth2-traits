#[macro_export]
macro_rules! scopes {
    ( [$($scope:ident),*] ) => {
        scopes!(Scopes [$($scope),*]);
    };
    ( pub [$($scope:ident),*] ) => {
        scopes!(pub Scopes [$($scope),*]);
    };
    ( $name:ident [$($scope:ident),*] ) => {
        #[derive(Eq, PartialEq, Debug)]
        enum $name {
            _InvalidScope,
            $($scope),*
        }

        impl <'a>From<&'a str> for $name {
            fn from(name: &'a str) -> Self {
                match name {
                    $(
                        stringify!($scope) => $name::$scope,
                    )*
                    _ => $name::_InvalidScope,
                }
            }
        }

        impl From<String> for $name {
            fn from(name: String) -> Self {
                $name::from(name.as_str())
            }
        }
    };
    ( pub $name:ident [$($scope:ident),*] ) => {
        #[derive(Eq, PartialEq, Debug)]
        pub enum $name {
            _InvalidScope,
            $($scope),*
        }

        impl <'a>From<&'a str> for $name {
            fn from(name: &'a str) -> Self {
                match name {
                    $(
                        stringify!($scope) => $name::$scope,
                    )*
                    _ => $name::_InvalidScope,
                }
            }
        }

        impl From<String> for $name {
            fn from(name: String) -> Self {
                $name::from(name.as_str())
            }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn named_scopes() {
        scopes!(MyScopes [ReadPosts, WritePosts]);
        assert_ne!(MyScopes::ReadPosts, MyScopes::WritePosts);
    }
    #[test]
    fn default_scopes() {
        scopes!([ReadPosts, WritePosts]);
        assert_ne!(Scopes::ReadPosts, Scopes::WritePosts);
    }
    #[test]
    fn pub_named_scopes() {
        mod internal {
            scopes!(pub MyScopes [ReadPosts, WritePosts]);
        }
        assert_ne!(internal::MyScopes::ReadPosts, internal::MyScopes::WritePosts);
    }
    #[test]
    fn pub_default_scopes() {
        mod internal {
            scopes!(pub [ReadPosts, WritePosts]);
        }
        assert_ne!(internal::Scopes::ReadPosts, internal::Scopes::WritePosts);
    }
    #[test]
    fn create_from_string() {
        scopes!(StringScopes [ReadPosts]);
        assert_eq!(StringScopes::from(String::from("ReadPosts")), StringScopes::ReadPosts);
    }
    #[test]
    fn create_from_str() {
        scopes!(StringScopes [ReadPosts]);
        assert_eq!(StringScopes::from("ReadPosts"), StringScopes::ReadPosts);
    }
}