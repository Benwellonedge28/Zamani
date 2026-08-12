#![allow(dead_code, unused_variables, unused_imports)]
//! Zamani Compiler — Incremental Compilation & Dependency Tracking

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

pub struct IncrementalCompiler {
    file_hashes: HashMap<PathBuf, u64>,
    dependency_graph: HashMap<PathBuf, HashSet<PathBuf>>,
    dirty_files: HashSet<PathBuf>,
}

impl IncrementalCompiler {
    pub fn new() -> Self {
        IncrementalCompiler {
            file_hashes: HashMap::new(),
            dependency_graph: HashMap::new(),
            dirty_files: HashSet::new(),
        }
    }

    pub fn record_file(&mut self, path: impl Into<PathBuf>, hash: u64, deps: Vec<PathBuf>) {
        let p = path.into();
        println!("[Incremental] Recording file state for '{:?}' (hash: {:#x})", p, hash);
        let old_hash = self.file_hashes.insert(p.clone(), hash);
        if old_hash != Some(hash) {
            self.dirty_files.insert(p.clone());
        }
        let mut dep_set = HashSet::new();
        for d in deps {
            dep_set.insert(d);
        }
        self.dependency_graph.insert(p, dep_set);
    }

    pub fn check_dirty(&self, path: &PathBuf) -> bool {
        self.dirty_files.contains(path)
    }

    pub fn get_compilation_plan(&self) -> Vec<PathBuf> {
        println!("[Incremental] Computing optimal compilation plan (Dirty files: {})...", self.dirty_files.len());
        self.dirty_files.iter().cloned().collect()
    }
}
