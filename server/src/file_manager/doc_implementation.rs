use serde::Serialize;
use serde_json::{Value, json};

use super::{errors::FileManagerError, Manager};
use std::{fs::{File, remove_file}, io::{Write, BufWriter}, error::Error, path::Path};

impl Manager {
    pub fn create_doc<T: Serialize>(&mut self, index_id: String, id: String, data: T) -> Result<(), Box<dyn Error>> {
        // Check index and doc
        match self.indices.get_mut(&index_id) {
            Some(index) => {
                if index.docs.contains_key(&id) { return Err(
                    FileManagerError::new("Doc already exist").into()
                )}
                // Create doc
                let doc_path = Path::new(&index.path).join(&id);
                let doc_file = File::create(&doc_path)?;
                let mut doc_writer = BufWriter::new(doc_file);
                serde_json::to_writer(&mut doc_writer, &data)?;
                doc_writer.flush()?;

                // Update index meta
                index.docs.insert(id, doc_path.into_os_string().into_string().unwrap());
                self.update_header_file(index_id)?;
            },
            None => return Err(
                FileManagerError::new(&format!("Index {} doesn't exist", index_id)).into()
            ),
        }
        Ok(())
    }
    pub fn update_doc<T: Serialize>(&mut self, index_id: String, doc: String, partial_data: T) -> Result<(), Box<dyn Error>> {
        // Check index and doc
        match self.indices.get_mut(&index_id) {
            Some(index) => {
                if !index.docs.contains_key(&doc) { return Err(
                    FileManagerError::new("Doc doen't exist").into()
                )}
                // Open doc
                let doc_path = Path::new(&index.path).join(&doc);
                let mut doc_file = File::open(&doc_path)?;
                let mut data: Value = serde_json::from_reader(&doc_file).expect("JSON was not well-formatted");
                merge(&mut data, json!(partial_data));
                doc_file = File::create(&doc_path)?;
                let mut doc_writer = BufWriter::new(doc_file);
                serde_json::to_writer(&mut doc_writer, &data)?;
                doc_writer.flush()?;
            },
            None => return Err(
                FileManagerError::new(&format!("Index {} doesn't exist", index_id)).into()
            ),
        }
        Ok(())
    }
    pub fn delete_doc(&mut self, index_id: String, id: String) -> Result<(), Box<dyn Error>> {
        // Check index and doc
        match self.indices.get_mut(&index_id) {
            Some(index) => {
                if !index.docs.contains_key(&id) { return Err(
                    FileManagerError::new("Doc doen't exist").into()
                )}
                // Remove doc
                let doc_path = Path::new(&index.path).join(&id);
                remove_file(doc_path)?;
                // Update index meta
                index.docs.remove(&id);
                self.update_header_file(index_id)?;
            },
            None => return Err(
                FileManagerError::new(&format!("Index {} doesn't exist", index_id)).into()
            ),
        }
        Ok(())
    }
    pub fn get_doc(&mut self, index_id: String, id: String) -> Result<Value, Box<dyn Error>> {
        // Check index and doc
        match self.indices.get_mut(&index_id) {
            Some(index) => {
                if !index.docs.contains_key(&id) { return Err(
                    FileManagerError::new("Doc doen't exist").into()
                )}
                // Open doc
                let doc_path = Path::new(&index.path).join(&id);
                let doc_file = File::open(doc_path)?;
                let data: Value = serde_json::from_reader(&doc_file).expect("JSON was not well-formatted");
                Ok(data)
            },
            None => Err(
                FileManagerError::new(&format!("Index {} doesn't exist", index_id)).into()
            ),
        }
    }
}

fn merge(a: &mut Value, b: Value) {
    if let Value::Object(a) = a {
        if let Value::Object(b) = b {
            for (k, v) in b {
                if v.is_null() {
                    a.remove(&k);
                }
                else {
                    merge(a.entry(k).or_insert(Value::Null), v);
                }
            } 

            return;
        }
    }    
    *a = b;
}