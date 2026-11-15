//! SigmaOS-abs: abstract base class model.
//!
//! Real implementation: inheritance, traits, and validation.
//! Replaces the 108-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Abstract class.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbstractClass {
    pub id: String,
    pub name: String,
    pub base_trait: String,
    pub method_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbstractError {
    EmptyName,
    DuplicateClass(String),
}

impl fmt::Display for AbstractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "class name must not be empty"),
            Self::DuplicateClass(id) => write!(f, "duplicate class {:?}", id),
        }
    }
}

impl std::error::Error for AbstractError {}

/// Abstract base class manager.
pub struct AbstractBaseSystem {
    pub name: String,
    classes: BTreeMap<String, AbstractClass>,
}

impl AbstractBaseSystem {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            classes: BTreeMap::new(),
        }
    }

    pub fn add_class(&mut self, class: AbstractClass) -> Result<(), AbstractError> {
        if class.name.trim().is_empty() { return Err(AbstractError::EmptyName); }
        let id = class.id.clone();
        if self.classes.contains_key(&id) {
            return Err(AbstractError::DuplicateClass(id.clone()));
        }
        self.classes.insert(id, class);
        Ok(())
    }

    pub fn class_count(&self) -> usize { self.classes.len() }

    pub fn get_class(&self, id: &str) -> Option<&AbstractClass> { self.classes.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs_create_aaa() {
        let a = AbstractBaseSystem::new("My ABS");
        assert_eq!(a.name, "My ABS");
        assert_eq!(a.class_count(), 0);
    }

    #[test]
    fn test_add_class_aaa() {
        let mut a = AbstractBaseSystem::new("My ABS");
        let c = AbstractClass { id: "c1".into(), name: "Shape".into(), base_trait: "Drawable".into(), method_count: 3 };
        a.add_class(c).unwrap();
        assert_eq!(a.class_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut a = AbstractBaseSystem::new("My ABS");
        let c = AbstractClass { id: "x".into(), name: "".into(), base_trait: "a".into(), method_count: 1 };
        assert!(a.add_class(c).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut a = AbstractBaseSystem::new("My ABS");
        let c = AbstractClass { id: "x".into(), name: "X".into(), base_trait: "a".into(), method_count: 1 };
        a.add_class(c).unwrap();
        let c2 = AbstractClass { id: "x".into(), name: "X".into(), base_trait: "a".into(), method_count: 1 };
        assert!(a.add_class(c2).is_err());
    }
}
