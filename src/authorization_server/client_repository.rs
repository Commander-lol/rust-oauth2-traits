use client::Client;

pub trait ClientRepository<ClientImpl> where ClientImpl: Client {
    fn find_by_id(&self, client_id: &String) -> Option<&ClientImpl>;
    fn commit(&mut self, client: &ClientImpl);
    fn invalidate(&mut self, client: &ClientImpl);
    fn invalidate_id(&mut self, client_id: &String);
}