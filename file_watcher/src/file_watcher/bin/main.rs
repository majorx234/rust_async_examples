use tokio;
use watchman_client::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = Connector::new().connect().await?;
  let resolved = client
     .resolve_root(CanonicalPath::canonicalize(".")?)
     .await?;

  // Basic globs -> names
  let files = client.glob(&resolved, &["**/*.rs"]).await?;
  println!("files: {:#?}", files);
  Ok(())
}
