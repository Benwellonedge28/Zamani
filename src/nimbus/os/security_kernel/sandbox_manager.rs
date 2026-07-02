#![allow(dead_code, unused_variables, unused_imports)]
//! NIMBUS OS Security Kernel — Sandbox Manager.
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum SandboxLevel {
    Unrestricted,
    Permissive,  // Limited syscalls
    Strict,      // Minimal syscalls
    Isolated,    // No external I/O
    NullSandbox, // Complete isolation
}

#[derive(Debug, Clone)]
pub struct Sandbox {
    pub id: u64,
    pub level: SandboxLevel,
    pub allowed_syscalls: Vec<String>,
    pub memory_limit_mb: u64,
    pub cpu_quota: f32,
    pub active: bool,
}

pub struct SandboxManager {
    sandboxes: HashMap<u64, Sandbox>,
    next_id: u64,
    escapes_attempted: u64,
}

impl SandboxManager {
    pub fn new() -> Self {
        SandboxManager {
            sandboxes: HashMap::new(),
            next_id: 1,
            escapes_attempted: 0,
        }
    }

    pub fn create(&mut self, level: SandboxLevel, memory_limit_mb: u64) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let allowed = match &level {
            SandboxLevel::Permissive => vec!["read".into(), "write".into(), "mmap".into()],
            SandboxLevel::Strict => vec!["read".into(), "write".into()],
            _ => Vec::new(),
        };
        self.sandboxes.insert(
            id,
            Sandbox {
                id,
                level,
                allowed_syscalls: allowed,
                memory_limit_mb,
                cpu_quota: 0.1,
                active: true,
            },
        );
        id
    }

    pub fn allow_syscall(&mut self, id: u64, syscall: &str) -> bool {
        self.sandboxes
            .get_mut(&id)
            .map(|s| {
                s.allowed_syscalls.push(syscall.to_string());
                true
            })
            .unwrap_or(false)
    }

    pub fn attempt_escape(&mut self, _id: u64) -> bool {
        self.escapes_attempted += 1;
        false // Always blocked
    }

    pub fn terminate(&mut self, id: u64) -> bool {
        self.sandboxes.remove(&id).is_some()
    }
}

impl Default for SandboxManager {
    fn default() -> Self {
        Self::new()
    }
}
