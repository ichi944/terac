#[derive(Debug)]
pub struct Client<P = Discovered> {
    pub provider: P,
    pub client_id: string,
}
