#![feature(macro_reexport)]

#[cfg(feature = "uuid")] extern crate uuid;

#[macro_use]
#[macro_reexport(scopes)]
extern crate oauth2_traits_macros;


pub mod authorization_server;
pub mod client;
pub mod resource_owner;
pub mod resource_server;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
