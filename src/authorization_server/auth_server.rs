use client::Client;
use std::marker::PhantomData;
use authorization_server::ClientRepository;

pub struct AuthorizationServer<Gen, Scopes, ClientImpl, ClientRepo>
    where
        Gen: IdGenerator,
        Scopes: Eq + PartialEq,
        ClientImpl: Client,
        ClientRepo: ClientRepository<ClientImpl>
{
    id_gen: Gen,
    uri_slug: String,
    client_repo: ClientRepo,
    _scopes_type: ::std::marker::PhantomData<Scopes>,
    _client_impl_type: PhantomData<ClientImpl>
}

pub struct AccessToken<Scopes> where Scopes: Eq + PartialEq {
    token: String,
    refresh_token: Option<String>,
    scopes: Vec<Scopes>,
}

impl <Gen, Scopes, ClientImpl, ClientRepo>AuthorizationServer<Gen, Scopes, ClientImpl, ClientRepo>
    where
        Gen: IdGenerator,
        Scopes: Eq + PartialEq,
        ClientImpl: Client,
        ClientRepo: ClientRepository<ClientImpl>
{
    pub fn new<BaseUri>(base_uri: BaseUri, id_gen: Gen, client_repo: ClientRepo) -> Self
        where BaseUri: Into<String> {
        AuthorizationServer {
            id_gen,
            uri_slug: base_uri.into(),
            client_repo,
            _scopes_type: PhantomData,
            _client_impl_type: PhantomData,
        }
    }

    pub fn create_client<BaseUri>(&mut self, redirect: BaseUri) -> ClientImpl where BaseUri: Into<String> {
        let client = Client::from_params(
            self.id_gen.create_id(),
            self.id_gen.create_id(),
            redirect.into()
        );

        self.client_repo.commit(&client);

        client
    }

    pub fn create_access_token(&mut self, client: ClientImpl, scopes: Vec<Scopes>) -> AccessToken<Scopes> {
        AccessToken {
            token: String::from("Foo"),
            refresh_token: None,
            scopes,
        }
    }

    pub fn retrieve_client(&self, client_id: String) -> Option<&ClientImpl> {
        self.client_repo.find_by_id(&client_id)
    }
}

pub trait IdGenerator {
    fn instance() -> Self;
    fn create_id(&self) -> String;
}
