
#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
  Nonce {id: String, value: u64}
}