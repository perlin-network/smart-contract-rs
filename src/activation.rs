#[derive(Deserialize)]
pub struct TransferActivation {
    pub sender: Vec<u8>,
    pub amount: u64,
}

#[derive(Deserialize)]
pub struct CustomActivation<T> {
    pub body: T,
}
