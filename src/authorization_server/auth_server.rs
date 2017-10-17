use client::Client;
use std::marker::PhantomData;

pub struct AuthorizationServer<Gen, Scopes>
    where
        Gen: IdGenerator,
        Scopes: Eq + PartialEq {
    uri_slug: String,
    id_gen: Gen,
    scopes_type: ::std::marker::PhantomData<Scopes>
}

pub struct AccessToken<Scopes> where Scopes: Eq + PartialEq {
    token: String,
    refresh_token: Option<String>,
    scopes: Vec<Scopes>,
}

impl <Gen, Scopes>AuthorizationServer<Gen, Scopes>
    where
        Gen: IdGenerator,
        Scopes: Eq + PartialEq {

    fn new<BaseUri>(base_uri: BaseUri, id_gen: Gen) -> Self
        where BaseUri: Into<String> {
        AuthorizationServer {
            uri_slug: base_uri.into(),
            id_gen,
            scopes_type: PhantomData,
        }
    }

    fn create_client<ClientImpl, BaseUri>(&self, redirect: BaseUri) -> ClientImpl
        where
            ClientImpl: Client,
            BaseUri: Into<String> {
        Client::from_params(
            self.id_gen.create_id(),
            self.id_gen.create_id(),
            redirect.into()
        )
    }

    fn create_access_token<ClientImpl>(&self, id: ClientImpl, scopes: Vec<Scopes>) -> AccessToken<Scopes>
        where ClientImpl: Client {
        AccessToken {
            token: String::from("Foo"),
            refresh_token: None,
            scopes,
        }
    }
}

pub trait IdGenerator {
    fn instance() -> Self;
    fn create_id(&self) -> String;
}


#[cfg(test)]
mod tests {
    use super::*;
    use oauth2_traits_macros;

    scopes!(TestScopes [ReadPosts, WritePosts]);

    struct TestClient {
        client_id: String,
        client_secret: String,
        redirect_uri: String,
    }

    struct TestGen {}

    impl IdGenerator for TestGen {
        fn instance() -> Self {
            TestGen {}
        }

        fn create_id(&self) -> String {
            String::from("foo")
        }
    }

    impl Client for TestClient {
        fn from_params(client_id: String, client_secret: String, redirect_uri: String) -> Self {
            TestClient {
                client_id,
                client_secret,
                redirect_uri,
            }
        }

        fn get_client_id(&self) -> &String {
            &self.client_id
        }

        fn get_client_secret(&self) -> &String {
            &self.client_secret
        }

        fn get_redirect_uri(&self) -> &String {
            &self.redirect_uri
        }
    }

    #[test]
    fn creates_client_type() {
        let server: AuthorizationServer<TestGen, TestScopes> = AuthorizationServer::new("https://localhost", TestGen::instance());
        let client: TestClient = server.create_client("https://myservice.com");

        assert_eq!(client.get_client_id(), &String::from("foo"));
        assert_eq!(client.get_client_secret(), &String::from("foo"));
        assert_eq!(client.get_redirect_uri(), &String::from("https://myservice.com"));
    }

    #[test]
    fn redundant() {
        let foo = TestScopes::ReadPosts;
        let bar = TestScopes::ReadPosts;
        let baz = TestScopes::WritePosts;

        assert_eq!(foo, bar);
        assert_ne!(foo, baz);
        assert_ne!(bar, baz);

        assert_eq!(TestScopes::from("ReadPosts"), foo);
        assert_eq!(TestScopes::from("WritePosts"), baz);
    }
}