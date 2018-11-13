#[derive(Deserialize)]
pub struct TransferActivation {
    pub sender: String,
    pub amount: u64,
}

#[derive(Deserialize)]
pub struct CustomActivation<T> {
    pub body: T,
}
