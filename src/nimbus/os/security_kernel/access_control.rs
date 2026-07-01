#![allow(dead_code, unused_variables, unused_imports)]
//! NIMBUS OS Security Kernel — Access Control (RBAC + capability-based).
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Role(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Permission(pub String);

#[derive(Debug, Clone)]
pub struct Subject {
    pub id: u64,
    pub name: String,
    pub roles: HashSet<Role>,
    pub capabilities: HashSet<String>,
}

pub struct AccessController {
    role_permissions: HashMap<String, HashSet<Permission>>,
    subjects: HashMap<u64, Subject>,
    audit_log: Vec<(u64, String, bool)>,
}

impl AccessController {
    pub fn new() -> Self {
        let mut ac = AccessController {
            role_permissions: HashMap::new(),
            subjects: HashMap::new(),
            audit_log: Vec::new(),
        };
        // Default roles
        ac.grant_role_permission("admin", "all");
        ac.grant_role_permission("user", "read");
        ac.grant_role_permission("agent", "read");
        ac.grant_role_permission("agent", "execute");
        ac
    }

    pub fn grant_role_permission(&mut self, role: &str, permission: &str) {
        self.role_permissions
            .entry(role.to_string())
            .or_default()
            .insert(Permission(permission.to_string()));
    }

    pub fn register_subject(&mut self, id: u64, name: &str, roles: Vec<&str>) {
        self.subjects.insert(id, Subject {
            id, name: name.to_string(),
            roles: roles.iter().map(|r| Role(r.to_string())).collect(),
            capabilities: HashSet::new(),
        });
    }

    pub fn check(&mut self, subject_id: u64, permission: &str) -> bool {
        let allowed = self.subjects.get(&subject_id).map(|s| {
            s.roles.iter().any(|role| {
                self.role_permissions.get(&role.0)
                    .map(|perms| perms.contains(&Permission(permission.to_string()))
                        || perms.contains(&Permission("all".to_string())))
                    .unwrap_or(false)
            }) || s.capabilities.contains(permission)
        }).unwrap_or(false);
        self.audit_log.push((subject_id, permission.to_string(), allowed));
        allowed
    }

    pub fn grant_capability(&mut self, subject_id: u64, cap: &str) -> bool {
        self.subjects.get_mut(&subject_id).map(|s| { s.capabilities.insert(cap.to_string()); true }).unwrap_or(false)
    }
}

impl Default for AccessController { fn default() -> Self { Self::new() } }
