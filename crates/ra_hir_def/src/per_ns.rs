//! In rust, it is possible to have a value, a type and a macro with the same
//! name without conflicts.
//!
//! `PerNs` (per namespace) captures this.

use hir_expand::MacroDefId;

use crate::{visibility::ResolvedVisibility, ModuleDefId};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PerNs {
    pub types: Option<(ModuleDefId, ResolvedVisibility)>,
    pub values: Option<(ModuleDefId, ResolvedVisibility)>,
    pub macros: Option<(MacroDefId, ResolvedVisibility)>,
}

impl Default for PerNs {
    fn default() -> Self {
        PerNs { types: None, values: None, macros: None }
    }
}

impl PerNs {
    pub fn none() -> PerNs {
        PerNs { types: None, values: None, macros: None }
    }

    pub fn values(t: ModuleDefId, v: ResolvedVisibility) -> PerNs {
        PerNs { types: None, values: Some((t, v)), macros: None }
    }

    pub fn types(t: ModuleDefId, v: ResolvedVisibility) -> PerNs {
        PerNs { types: Some((t, v)), values: None, macros: None }
    }

    pub fn both(types: ModuleDefId, values: ModuleDefId, v: ResolvedVisibility) -> PerNs {
        PerNs { types: Some((types, v)), values: Some((values, v)), macros: None }
    }

    pub fn macros(macro_: MacroDefId, v: ResolvedVisibility) -> PerNs {
        PerNs { types: None, values: None, macros: Some((macro_, v)) }
    }

    pub fn is_none(&self) -> bool {
        self.types.is_none() && self.values.is_none() && self.macros.is_none()
    }

    pub fn take_types(self) -> Option<ModuleDefId> {
        self.types.map(|it| it.0)
    }

    pub fn take_types_vis(self) -> Option<(ModuleDefId, ResolvedVisibility)> {
        self.types
    }

    pub fn take_values(self) -> Option<ModuleDefId> {
        self.values.map(|it| it.0)
    }

    pub fn take_macros(self) -> Option<MacroDefId> {
        self.macros.map(|it| it.0)
    }

    pub fn filter_visibility(self, mut f: impl FnMut(ResolvedVisibility) -> bool) -> PerNs {
        PerNs {
            types: self.types.filter(|(_, v)| f(*v)),
            values: self.values.filter(|(_, v)| f(*v)),
            macros: self.macros.filter(|(_, v)| f(*v)),
        }
    }

    pub fn with_visibility(self, vis: ResolvedVisibility) -> PerNs {
        PerNs {
            types: self.types.map(|(it, _)| (it, vis)),
            values: self.values.map(|(it, _)| (it, vis)),
            macros: self.macros.map(|(it, _)| (it, vis)),
        }
    }

    pub fn or(self, other: PerNs) -> PerNs {
        PerNs {
            types: self.types.or(other.types),
            values: self.values.or(other.values),
            macros: self.macros.or(other.macros),
        }
    }
}
