use crate::message::Message;

  #[test]
  fn test_basic() {
    let json_str = r#"
    {
      "Nonce":{"id":"Alice", "value":10}
    }
    "#;
    let msg: Message = serde_json::from_str(json_str).unwrap();
    println!("{:?}", msg);
  }
