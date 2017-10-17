use super::*;
use client::Client;
use oauth2_traits_macros;

scopes!(TestScopes [ReadPosts, WritePosts]);
simple_generator!(TestGen String::from("foo"));

#[derive(Clone, Eq, PartialEq, Debug)]
struct TestClient {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
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

struct TestClientRepo {
    clients: ::std::collections::HashMap<String, TestClient>,
}

impl TestClientRepo {
    fn instance() -> Self {
        TestClientRepo {
            clients: ::std::collections::HashMap::new(),
        }
    }
}

impl ClientRepository<TestClient> for TestClientRepo {
    fn find_by_id(&self, client_id: &String) -> Option<&TestClient> {
        self.clients.get(client_id)
    }

    fn commit(&mut self, client: &TestClient) {
        self.clients.insert(client.get_client_id().clone(), client.clone());
    }

    fn invalidate(&mut self, client: &TestClient) {
        self.invalidate_id(client.get_client_id());
    }

    fn invalidate_id(&mut self, client_id: &String) {
        self.clients.remove(client_id);
    }
}

fn new_test_server() -> AuthorizationServer<TestGen, TestScopes, TestClient, TestClientRepo> {
    AuthorizationServer::new(
        "https://localhost",
        TestGen::instance(),
        TestClientRepo::instance()
    )
}

#[test]
fn creates_client_type() {
    let mut server = new_test_server();
    let client = server.create_client("https://myservice.com");

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

#[test]
fn stores_and_retrieves_created_clients() {
    let mut server = new_test_server();
    let client = server.create_client("https://testserver.co");
    let same_client = server.retrieve_client(client.get_client_id().clone()).unwrap();
    assert_eq!(client, *same_client);
}