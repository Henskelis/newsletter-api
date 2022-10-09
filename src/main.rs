use newsletter_api::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let listener = TcpListener::bind("127.0.0.1:0")?;
  let address = listener.local_addr()?.to_string();
  println!("Listening on http://{}", &address);
  run(listener)?.await
}
