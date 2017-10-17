pub trait Client {
    fn from_params(client_id: String, client_secret: String, redirect_uri: String) -> Self;
    fn get_client_id(&self) -> &String;
    fn get_client_secret(&self) -> &String;
    fn get_redirect_uri(&self) -> &String;
}