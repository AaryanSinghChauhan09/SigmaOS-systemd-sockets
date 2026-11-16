// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-systemd-sockets: systemd sockets component model.
//!
//! Real implementation: sockets, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Socket {
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
pub enum SystemdSocketsError {
    EmptySocketName,
    DuplicateSocket(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemdSocketsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySocketName => write!(f, "socket name must not be empty"),
            Self::DuplicateSocket(id) => write!(f, "duplicate socket {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemdSocketsError {}

pub struct SystemdSockets {
    pub name: String,
    sockets: BTreeMap<String, Socket>,
    events: BTreeMap<String, Event>,
}

impl SystemdSockets {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            sockets: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_socket(&mut self, item: Socket) -> Result<(), SystemdSocketsError> {
        if item.name.trim().is_empty() {
            return Err(SystemdSocketsError::EmptySocketName);
        }
        let id = item.id.clone();
        if self.sockets.contains_key(&id) {
            return Err(SystemdSocketsError::DuplicateSocket(id.clone()));
        }
        self.sockets.insert(id, item);
        Ok(())
    }

    pub fn socket_count(&self) -> usize {
        self.sockets.len()
    }

    pub fn get_socket(&self, id: &str) -> Option<&Socket> {
        self.sockets.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemdSocketsError> {
        if event.subject.trim().is_empty() {
            return Err(SystemdSocketsError::EmptyEventSubject);
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

    fn sample_socket(id: &str, name: &str) -> Socket {
        Socket {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemdSockets::new("SystemdSockets");
        assert_eq!(m.name, "SystemdSockets");
        assert_eq!(m.socket_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_socket() {
        let mut m = SystemdSockets::new("SystemdSockets");
        m.add_socket(sample_socket("a1", "Alpha")).unwrap();
        assert_eq!(m.socket_count(), 1);
        assert!(m.get_socket("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemdSockets::new("SystemdSockets");
        assert!(m.add_socket(sample_socket("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemdSockets::new("SystemdSockets");
        m.add_socket(sample_socket("x", "One")).unwrap();
        assert!(m.add_socket(sample_socket("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemdSockets::new("SystemdSockets");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
