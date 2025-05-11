use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Element {
    /// Atomic number
    pub number: u8,
    /// Element symbol
    pub symbol: String,
    /// Element name
    pub name: String,
    /// Atomic mass
    pub mass: f64,
    /// Category of the element
    pub category: ElementCategory,
    /// Position in the periodic table (row, column)
    pub position: (u8, u8),
    /// Short description
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ElementCategory {
    AlkaliMetal,
    AlkalineEarthMetal,
    Lanthanide,
    Actinide,
    TransitionMetal,
    PostTransitionMetal,
    Metalloid,
    Nonmetal,
    Halogen,
    NobleGas,
    Unknown,
}

impl Element {
    pub fn color(&self) -> (u8, u8, u8) {
        match self.category {
            ElementCategory::AlkaliMetal => (255, 102, 102),        // Red
            ElementCategory::AlkalineEarthMetal => (255, 171, 102), // Orange
            ElementCategory::Lanthanide => (187, 153, 255),         // Light Purple
            ElementCategory::Actinide => (204, 153, 255),           // Purple
            ElementCategory::TransitionMetal => (255, 255, 102),    // Yellow
            ElementCategory::PostTransitionMetal => (153, 204, 255), // Light Blue
            ElementCategory::Metalloid => (102, 255, 178),          // Light Green
            ElementCategory::Nonmetal => (102, 255, 102),           // Green
            ElementCategory::Halogen => (102, 255, 255),            // Cyan
            ElementCategory::NobleGas => (204, 153, 204),           // Pink
            ElementCategory::Unknown => (180, 180, 180),            // Gray
        }
    }
}