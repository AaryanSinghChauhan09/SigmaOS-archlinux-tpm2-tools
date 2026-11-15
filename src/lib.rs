//! SigmaOS-archlinux-tpm2-tools: TPM2 tools config model.

use std::collections::BTreeMap;
use std::fmt;

/// Primary domain entity for TPM2 tools config.
pub struct Tpm2Key {
    pub id: String,
    pub name: String,
    pub key_type: String,
    pub enabled: bool,
}

/// Secondary domain entity for TPM2 tools config.
pub struct Tpm2Policy {
    pub id: String,
    pub algorithm: String,
    pub description: String,
}

/// Errors for the TPM2 tools config domain.
#[derive(Debug, PartialEq)]
pub enum Tpm2Error {
    EmptyName,
    Duplicate(String),
    NotFound(String),
}

impl fmt::Display for Tpm2Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "name cannot be empty"),
            Self::Duplicate(id) => write!(f, "duplicate id: {id}"),
            Self::NotFound(id) => write!(f, "not found: {id}"),
        }
    }
}

impl std::error::Error for Tpm2Error {}

/// Registry and manager for TPM2 tools config.
pub struct Tpm2Manager {
    pub name: String,
    items: BTreeMap<String, Tpm2Key>,
    configs: BTreeMap<String, Tpm2Policy>,
}

impl Tpm2Manager {
    /// Create a new manager instance.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            items: BTreeMap::new(),
            configs: BTreeMap::new(),
        }
    }

    /// Add a primary item with fail-fast validation.
    pub fn add_item(&mut self, item: Tpm2Key) -> Result<(), Tpm2Error> {
        if item.name.is_empty() {
            return Err(Tpm2Error::EmptyName);
        }
        if self.items.contains_key(&item.id) {
            return Err(Tpm2Error::Duplicate(item.id.clone()));
        }
        self.items.insert(item.id.clone(), item);
        Ok(())
    }

    /// Retrieve a primary item by id.
    pub fn get_item(&self, id: &str) -> Option<&Tpm2Key> {
        self.items.get(id)
    }

    /// Count of primary items.
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Add a secondary configuration entry.
    pub fn add_config(&mut self, cfg: Tpm2Policy) {
        self.configs.insert(cfg.id.clone(), cfg);
    }

    /// Count of configuration entries.
    pub fn config_count(&self) -> usize {
        self.configs.len()
    }

    /// Enable or disable an item.
    pub fn set_enabled(&mut self, id: &str, enabled: bool) -> Result<(), Tpm2Error> {
        self.items
            .get_mut(id)
            .map(|item| item.enabled = enabled)
            .ok_or_else(|| Tpm2Error::NotFound(id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_Tpm2Key(id: &str, name: &str) -> Tpm2Key {
        Tpm2Key {
            id: id.to_string(),
            name: name.to_string(),
            key_type: "archlinux-tpm2-tools-default".to_string(),
            enabled: true,
        }
    }

    fn make_Tpm2Policy(id: &str, val: &str) -> Tpm2Policy {
        Tpm2Policy {
            id: id.to_string(),
            algorithm: val.to_string(),
            description: format!("{} config", id),
        }
    }

    #[test]
    fn test_create_aaa() {
        let mgr = Tpm2Manager::new("test");
        assert_eq!(mgr.name, "test");
        assert_eq!(mgr.count(), 0);
        assert_eq!(mgr.config_count(), 0);
    }

    #[test]
    fn test_add_item_aaa() {
        let mut mgr = Tpm2Manager::new("test");
        let item = make_Tpm2Key("item1", "First Item");
        assert!(mgr.add_item(item).is_ok());
        assert_eq!(mgr.count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut mgr = Tpm2Manager::new("test");
        let item = make_Tpm2Key("item1", "");
        assert_eq!(mgr.add_item(item), Err(Tpm2Error::EmptyName));
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut mgr = Tpm2Manager::new("test");
        let item1 = make_Tpm2Key("item1", "First");
        let item2 = make_Tpm2Key("item1", "Duplicate");
        mgr.add_item(item1).unwrap();
        assert_eq!(mgr.add_item(item2), Err(Tpm2Error::Duplicate("item1".to_string())));
    }

    #[test]
    fn test_config_addition_aaa() {
        let mut mgr = Tpm2Manager::new("test");
        let cfg = make_Tpm2Policy("cfg1", "value1");
        mgr.add_config(cfg);
        assert_eq!(mgr.config_count(), 1);
    }

    #[test]
    fn test_get_item_aaa() {
        let mut mgr = Tpm2Manager::new("test");
        let item = make_Tpm2Key("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.get_item("item1").is_some());
        assert!(mgr.get_item("none").is_none());
    }

    #[test]
    fn test_set_enabled_aaa() {
        let mut mgr = Tpm2Manager::new("test");
        let item = make_Tpm2Key("item1", "First Item");
        mgr.add_item(item).unwrap();
        assert!(mgr.set_enabled("item1", false).is_ok());
        assert!(!mgr.get_item("item1").unwrap().enabled);
        assert_eq!(mgr.set_enabled("none", true), Err(Tpm2Error::NotFound("none".to_string())));
    }
}
