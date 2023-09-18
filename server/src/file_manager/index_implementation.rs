use super::{errors::FileManagerError, Manager, manager::Index};
use std::{collections::HashMap, fs::{create_dir, File, remove_dir_all}, io::Write, error::Error};

impl Manager {
    pub fn create_index(&mut self, id: String) -> Result<(), Box<dyn Error>> {
        // Check for id unicity
        if self.indices.contains_key(&id) {return Err(
            FileManagerError::new("Index already exist").into()
        )}
        let index_path = &self.env_path.join(&id);
        let index = Index {
            path: index_path
                .clone()
                .into_os_string()
                .into_string()
                .unwrap(), 
            id: id.clone(),
            docs:HashMap::new(),
        };
        self.indices.insert(id, index);
        // Create folder
        create_dir(index_path)?;
        
        // Create header file
        let mut header_file = File::create(index_path.join("__header"))?;
        header_file.write_all(b"{}")?;

        self.update_indices_file()?;

        Ok(())
    }
    pub fn delete_index(&mut self, id: String) -> Result<(), Box<dyn Error>> {
        // Check for id unicity
        if !self.indices.contains_key(&id) {return Err(
            FileManagerError::new(&format!("Index {} doesn't exist", id)).into()
        )}
        let index = self.indices.get(&id).unwrap();
 
        // Delete folder
        remove_dir_all(&index.path)?;

        // Update meta data
        self.indices.remove(&id).unwrap();
        self.update_indices_file()?;

        Ok(())
    }
    pub fn get_index(&mut self, index: String) -> Result<Index, Box<dyn Error>> {
        match self.indices.get(&index) {
            Some(index) => Ok(index.clone()),
            None => Err(FileManagerError::new(&format!("Index {} doesn't exist", index)).into()),
        }
    }
}