use std::{collections::HashMap, path::{Path, PathBuf}, fs::{create_dir, File}, io::{Write, BufWriter}, error::Error};
use serde::{Deserialize, Serialize};

pub struct Manager {
    pub env_path: PathBuf,
    pub indices: HashMap<String, Index>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Index {
    pub id: String,
    pub path: String,
    #[serde(skip_serializing, default)]
    pub docs: HashMap<String, String>
}

impl Manager {
    pub fn new(path: &Path) -> Self {
        let env_path = path.join("data");
        let indices: HashMap<String, Index> = 
        if !Path::new(&env_path).is_dir() {
            // Create data folder and indices file if not exist
            create_dir(&env_path).unwrap();
            let file = File::create(env_path.join("__indices")).unwrap();
            let indices: HashMap<String, Index> = HashMap::default();
            serde_json::to_writer(file, &indices).unwrap();
            indices
        } else {
            // Parse existing indices
            let file = File::open(env_path.join("__indices")).unwrap();
            let mut indices: HashMap<String, Index> = serde_json::from_reader(file).expect("JSON was not well-formatted");
            for index in indices.values_mut() {
                let doc_file = File::open(Path::new(&index.path).join("__header")).unwrap();
                let docs: HashMap<String, String> = serde_json::from_reader(doc_file).expect("JSON was not well-formatted");
                index.docs = docs;
            }
            indices
        };
        Manager {
            env_path,
            indices,
        }
    }
    pub fn update_indices_file(&mut self) -> Result<(), Box<dyn Error>> {
        let indices_file = File::create(self.env_path.join("__indices"))?;
        let mut indices_writer = BufWriter::new(indices_file);
        serde_json::to_writer(&mut indices_writer, &self.indices)?;
        indices_writer.flush()?;
        Ok(())
    }
    pub fn update_header_file(&mut self, index_id: String) -> Result<(), Box<dyn Error>> {
        let index = self.indices.get(&index_id).unwrap();
        let header_file = File::create(Path::new(&index.path).join("__header"))?;
        let mut header_writer = BufWriter::new(header_file);
        serde_json::to_writer(&mut header_writer, &index.docs)?;
        header_writer.flush()?;
        Ok(())
    }
}
