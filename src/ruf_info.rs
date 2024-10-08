//! This module contains the data structures used to store the information 
//! about the rufs used in the crate.

#[derive(Debug)]
pub struct UsedRufs(Vec<String>);

#[derive(Debug)]
pub struct CheckInfo {
    pub crate_name: String,
    pub used_rufs: UsedRufs,
    pub cfg: Vec<String>,
}

impl UsedRufs {
    pub fn new(rufs: Vec<String>) -> Self {
        UsedRufs(rufs)
    }
}
