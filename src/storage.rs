use serde::{Deserialize, Serialize};
use serde_json::Error;

pub enum StorageError {
    Serialize(Error),
    Save(std::io::Error),
}

pub trait Storage {
    type Item;
    fn load(&mut self);
    fn all(&self) -> Vec<Self::Item>;
    fn save(&self) -> Result<(), StorageError>;
}

pub struct JsonFile<T>
where
    for<'de> T: Deserialize<'de> + Serialize + Clone,
{
    file: String,
    entries: Vec<T>,
}

impl<T> Storage for JsonFile<T>
where
    for<'de> T: Deserialize<'de> + Serialize + Clone,
{
    type Item = T;

    fn load(&mut self) {
        let c = std::fs::read_to_string(self.file.clone())
            .expect("Something went wrong reading the file");

        self.entries = serde_json::from_str(c.as_str()).unwrap();
    }

    fn all(&self) -> Vec<Self::Item> {
        self.entries.clone()
    }

    fn save(&self) -> Result<(), StorageError> {
        let serialized_entries = serde_json::to_string(&self.entries);

        if let Err(err) = serialized_entries {
            return Err(StorageError::Serialize(err));
        }

        let serialized_entries = serialized_entries.unwrap();

        match std::fs::write(self.file.as_str(), serialized_entries) {
            Ok(_) => Ok(()),
            Err(err) => Err(StorageError::Save(err)),
        }
    }
}

impl<T> JsonFile<T>
where
    for<'de> T: Deserialize<'de> + Serialize + Clone,
{
    pub fn new(file: String, load: bool) -> Self {
        let mut storage = JsonFile {
            file,
            entries: vec![],
        };

        load.then(|| storage.load());

        storage
    }
}
