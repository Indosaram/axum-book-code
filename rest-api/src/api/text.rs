pub async fn text() -> String {
    tokio::fs::read_to_string("alice_in_wonderland.txt")
        .await
        .unwrap()
}
