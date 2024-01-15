use crate::ot::operations::{self, DeleteOperation, InsertOperation, Operation, OperationTrait};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub(super) struct WsConnectionQuery {
    pub document: String,
    pub client: String,
}

#[derive(Serialize, Deserialize)]
enum OperationType {
    INSERT,
    DELETE,
}

// IDK if this structure is needed
#[derive(Serialize)]
pub(super) struct OperationJSONFreeCopy<'a> {
    kind: OperationType,
    position: usize,
    revision: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    length: Option<usize>,
}

impl<'a> OperationJSONFreeCopy<'a> {
    pub fn from_operation(operation: &'a Operation) -> Self {
        if let Some(insert_operation) = operation.downcast::<InsertOperation>() {
            OperationJSONFreeCopy {
                kind: OperationType::INSERT,
                position: insert_operation.position(),
                revision: insert_operation.revision(),
                content: Some(insert_operation.text()),
                length: None,
            }
        } else if let Some(delete_operation) = operation.downcast::<DeleteOperation>() {
            OperationJSONFreeCopy {
                kind: OperationType::DELETE,
                position: delete_operation.position(),
                revision: delete_operation.revision(),
                content: None,
                length: Some(*delete_operation.len()),
            }
        } else {
            unreachable!();
        }
    }
}

#[derive(Deserialize)]
pub(super) struct OperationJSONContract {
    kind: OperationType,
    position: usize,
    revision: usize,
    content: Option<String>,
    length: Option<usize>,
}

impl OperationJSONContract {
    #[allow(unused)]
    pub fn from_operation(operation: &Operation) -> Self {
        if let Some(insert_operation) = operation.downcast::<InsertOperation>() {
            OperationJSONContract {
                kind: OperationType::INSERT,
                position: insert_operation.position(),
                revision: insert_operation.revision(),
                content: Some(insert_operation.text().clone()),
                length: None,
            }
        } else if let Some(delete_operation) = operation.downcast::<DeleteOperation>() {
            OperationJSONContract {
                kind: OperationType::DELETE,
                position: delete_operation.position(),
                revision: delete_operation.revision(),
                content: None,
                length: Some(*delete_operation.len()),
            }
        } else {
            unreachable!();
        }
    }
    pub fn as_operation(self) -> Option<Box<dyn operations::OperationTrait>> {
        match self.kind {
            OperationType::INSERT => Some(Box::new(operations::InsertOperation::new(
                self.position,
                self.revision,
                match self.content {
                    Some(content) => content,
                    None => return None,
                },
            ))),
            OperationType::DELETE => Some(Box::new(operations::DeleteOperation::new(
                self.position,
                self.revision,
                match self.length {
                    Some(len) => len,
                    None => return None,
                },
            ))),
        }
    }
}
