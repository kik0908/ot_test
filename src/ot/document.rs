use core::fmt::Debug;
use std::sync::Arc;

use super::operations::{ArcOperation, Operation};

pub trait DocumentTrait: Debug {
    fn apply(&mut self, operation: Operation) -> ArcOperation;
    fn revision(&self) -> usize;
}

#[derive(Debug)]
pub struct DocumentMem {
    operations: Vec<ArcOperation>,
}

impl DocumentMem {
    pub fn new() -> DocumentMem {
        DocumentMem {
            operations: Vec::new(),
        }
    }

    #[cfg(debug_assertions)]
    pub fn print(&self) {
        for i in &self.operations {
            i.print();
        }
        println!("");
    }
}

impl DocumentTrait for DocumentMem {
    fn apply(&mut self, operation: Operation) -> ArcOperation {
        // TODO add a check that revision is valid
        let mut op = operation;
        for i in &self.operations[op.revision()..] {
            op.transform_relative_to(Box::as_ref(&i));
        }
        op.set_revision(self.operations.len());
        let op = Arc::new(op);
        self.operations.push(op);
        Arc::clone(self.operations.last().unwrap())
    }

    fn revision(&self) -> usize {
        self.operations.len()
    }
}
