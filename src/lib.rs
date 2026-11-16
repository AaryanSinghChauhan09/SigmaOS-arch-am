// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-arch-am: arch am component model.
//!
//! Real implementation: arch_am_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchAmItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchAmError {
    EmptyArchAmItemName,
    DuplicateArchAmItem(String),
    EmptyEventSubject,
}

impl fmt::Display for ArchAmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyArchAmItemName => write!(f, "arch_am_item name must not be empty"),
            Self::DuplicateArchAmItem(id) => write!(f, "duplicate arch_am_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ArchAmError {}

pub struct ArchAm {
    pub name: String,
    arch_am_items: BTreeMap<String, ArchAmItem>,
    events: BTreeMap<String, Event>,
}

impl ArchAm {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            arch_am_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_arch_am_item(&mut self, item: ArchAmItem) -> Result<(), ArchAmError> {
        if item.name.trim().is_empty() {
            return Err(ArchAmError::EmptyArchAmItemName);
        }
        let id = item.id.clone();
        if self.arch_am_items.contains_key(&id) {
            return Err(ArchAmError::DuplicateArchAmItem(id.clone()));
        }
        self.arch_am_items.insert(id, item);
        Ok(())
    }

    pub fn arch_am_item_count(&self) -> usize {
        self.arch_am_items.len()
    }

    pub fn get_arch_am_item(&self, id: &str) -> Option<&ArchAmItem> {
        self.arch_am_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ArchAmError> {
        if event.subject.trim().is_empty() {
            return Err(ArchAmError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_arch_am_item(id: &str, name: &str) -> ArchAmItem {
        ArchAmItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ArchAm::new("ArchAm");
        assert_eq!(m.name, "ArchAm");
        assert_eq!(m.arch_am_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_arch_am_item() {
        let mut m = ArchAm::new("ArchAm");
        m.add_arch_am_item(sample_arch_am_item("a1", "Alpha")).unwrap();
        assert_eq!(m.arch_am_item_count(), 1);
        assert!(m.get_arch_am_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ArchAm::new("ArchAm");
        assert!(m.add_arch_am_item(sample_arch_am_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ArchAm::new("ArchAm");
        m.add_arch_am_item(sample_arch_am_item("x", "One")).unwrap();
        assert!(m.add_arch_am_item(sample_arch_am_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ArchAm::new("ArchAm");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
