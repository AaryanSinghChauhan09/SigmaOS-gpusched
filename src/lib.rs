// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-gpusched: gpusched component model.
//!
//! Real implementation: gpusched_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GpuschedItem {
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
pub enum GpuschedError {
    EmptyGpuschedItemName,
    DuplicateGpuschedItem(String),
    EmptyEventSubject,
}

impl fmt::Display for GpuschedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyGpuschedItemName => write!(f, "gpusched_item name must not be empty"),
            Self::DuplicateGpuschedItem(id) => write!(f, "duplicate gpusched_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for GpuschedError {}

pub struct Gpusched {
    pub name: String,
    gpusched_items: BTreeMap<String, GpuschedItem>,
    events: BTreeMap<String, Event>,
}

impl Gpusched {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            gpusched_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_gpusched_item(&mut self, item: GpuschedItem) -> Result<(), GpuschedError> {
        if item.name.trim().is_empty() {
            return Err(GpuschedError::EmptyGpuschedItemName);
        }
        let id = item.id.clone();
        if self.gpusched_items.contains_key(&id) {
            return Err(GpuschedError::DuplicateGpuschedItem(id.clone()));
        }
        self.gpusched_items.insert(id, item);
        Ok(())
    }

    pub fn gpusched_item_count(&self) -> usize {
        self.gpusched_items.len()
    }

    pub fn get_gpusched_item(&self, id: &str) -> Option<&GpuschedItem> {
        self.gpusched_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), GpuschedError> {
        if event.subject.trim().is_empty() {
            return Err(GpuschedError::EmptyEventSubject);
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

    fn sample_gpusched_item(id: &str, name: &str) -> GpuschedItem {
        GpuschedItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Gpusched::new("Gpusched");
        assert_eq!(m.name, "Gpusched");
        assert_eq!(m.gpusched_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_gpusched_item() {
        let mut m = Gpusched::new("Gpusched");
        m.add_gpusched_item(sample_gpusched_item("a1", "Alpha")).unwrap();
        assert_eq!(m.gpusched_item_count(), 1);
        assert!(m.get_gpusched_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Gpusched::new("Gpusched");
        assert!(m.add_gpusched_item(sample_gpusched_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Gpusched::new("Gpusched");
        m.add_gpusched_item(sample_gpusched_item("x", "One")).unwrap();
        assert!(m.add_gpusched_item(sample_gpusched_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Gpusched::new("Gpusched");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
