use std::net::TcpListener;

#[tokio::test]
async fn check_health_check() {
  let address = spawn_app();
  let client = reqwest::Client::new();

  let response = client
    .get(&format!("{}/health-check", &address))
    .send()
    .await
    .expect("Failed to execute request");

  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
  let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
  let address = listener.local_addr().unwrap().to_string();
  let server = newsletter_api::run(listener).expect("Failed to bind address");
  let _ = tokio::spawn(server);
  format!("http://{}", &address)
}
