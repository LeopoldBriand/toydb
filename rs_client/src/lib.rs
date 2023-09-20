use serde::{Serialize, Deserialize};
use serde_json::Value;
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::error::Error;

pub struct DbClient {
    host: String,
    port: String,
}

impl DbClient {
    pub fn new(host: String, port: String) -> Self {
        DbClient{ host, port }
    }
    pub async fn get<T: for<'a> Deserialize<'a>>(&self, index: String, doc: String) -> Result<T, Box<dyn Error>> {
        let request = format!("GET {}:{}\n", index, doc);
        let response = self.query(request).await?;
        let value: T = serde_json::from_str(&response)?;
        Ok(value)
    }
    pub async fn get_index(&self, index: String) -> Result<IndexMeta, Box<dyn Error>> {
        let request = format!("GET {}\n", index);
        let response = self.query(request).await?;
        let value: IndexMeta = serde_json::from_str(&response)?;
        Ok(value)
    }
    pub async fn set<T: Serialize>(&self, index: String, doc: String, data: T) -> Result<DocMeta, Box<dyn Error>> {
        let value = serde_json::to_value(data)?;
        let request = match value {
            Value::Object(_) | Value::Array(_)=> format!("SET {}:{} = JSON{}\n", index, doc, value),
            Value::String(_) | Value::Number(_) | Value::Bool(_)=> format!("SET {}:{} = {}\n", index, doc, value),
            _ => panic!("Unknown value"),
        };
        let response = self.query(request).await?;
        let value: DocMeta = serde_json::from_str(&response)?;
        Ok(value)
    }
    pub async fn delete(&self, index: String, doc: String) -> Result<DocMeta, Box<dyn Error>> {
        let request = format!("DELETE {}:{}\n", index, doc);
        let response = self.query(request).await?;
        let value: DocMeta = serde_json::from_str(&response)?;
        Ok(value)
    }
    pub async fn delete_index(&self, index: String) -> Result<IndexMeta, Box<dyn Error>> {
        let request = format!("DELETE {}\n", index);
        let response = self.query(request).await?;
        let value: IndexMeta = serde_json::from_str(&response)?;
        Ok(value)
    }
    async fn query(&self, request: String) -> Result<String, Box<dyn Error>> {
        let addr: &[std::net::SocketAddr] = &[format!("{}:{}", self.host, self.port).parse().unwrap()];
        let mut stream = TcpStream::connect(&addr).await?;
        stream.write_all(request.as_bytes()).await?;

        let mut buffer = vec![0; 1024];
        let n = stream.read(&mut buffer).await?;
        return Ok(String::from_utf8_lossy(&buffer[..n]).into_owned());
    }
}

#[derive(Deserialize)]
pub struct IndexMeta {
    pub docs: Vec<String>,
    pub index: String,
}

#[derive(Deserialize)]
pub struct DocMeta {
    pub doc: String,
    pub index: String,
}

#[cfg(test)]
mod tests {
}
