# Rust Client
## Exemple of use
```rust
use std::error::Error;
use rs_client::DbClient;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    name: String,
    author: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = DbClient::new("127.0.0.1".to_owned(), "5252".to_owned());
    let books = vec![
        Book {
            name: "la rivière à l'envers".to_owned(),
            author: "Jean-Claude Mourlevat".to_owned(),
        }
    ];
    client.set("books".to_string(), "test".to_string(), books).await?;
    let result = client.get::<Vec<Book>>("books".to_string(), "test".to_string()).await?;
    client.delete("books".to_string(), "test".to_string()).await?;
    println!("{:?}", result);
    Ok(())
}
```