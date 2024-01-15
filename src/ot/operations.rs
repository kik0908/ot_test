use core::fmt::Debug;
use derive_getters::Getters;
use std::{any::Any, cmp::min, sync::Arc};

pub type Operation = Box<dyn OperationTrait>;
pub type ArcOperation = Arc<Operation>;

pub trait OperationTrait: Send + Sync + Debug {
    /// Applies the operation to the text
    ///
    /// ## Example:
    /// ```ignore
    /// let tmp: Box<dyn OperationTrait> = Box::new(InsertOperation{position: 1, revision: 0, text: "1"});
    /// let s = String::from("abc");
    /// tmp.apply(&mut s);
    /// assert_eq!(s, String::from("a1bc"));
    /// ```
    fn apply(&self, text: &mut String);

    fn apply_return(&self, text: &mut String) -> String;

    /// Transforms the operation relative to transmitted `operation`
    fn transform_relative_to(&mut self, operation: &dyn OperationTrait);

    /// Getter of the last known revision
    fn revision(&self) -> Revision;

    /// Setter of the last known revision
    fn set_revision(&mut self, revision: Revision);

    /// Position getter
    fn position(&self) -> Position;

    /// Casts your object in &dyn Any.
    ///
    /// Needed for downcasting.
    fn as_any(&self) -> &dyn Any;

    /// Casts your object in &mut dyn Any.
    ///
    /// Needed for downcasting.
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Printing, only for debug
    #[cfg(debug_assertions)]
    fn print(&self);
}
impl dyn OperationTrait + '_ {
    /// Downcasts your object to `&T`.
    ///
    /// Return `Some(&T)` if the cast was successful, otherwise `None`
    ///
    /// ## Example:
    /// ```ignore
    /// let tmp: Box<dyn OperationTrait> = Box::new(DeleteOperation{...});
    /// let ans1 = tmp.downcast::<DeleteOperation>() // Some(&DeleteOperation{...})
    /// let ans2 = tmp.downcast::<InsertOperation>() // None
    /// ```
    pub fn downcast<T: Any>(&self) -> Option<&T> {
        match self.as_any().downcast_ref::<T>() {
            Some(el) => Some(el),
            None => None,
        }
    }

    /// Downcasts your object to `&mut T`.
    ///
    /// Return `Some(&mut T)` if the cast was successful, otherwise `None`
    ///
    /// ## Example:
    /// ```ignore
    /// let tmp: Box<dyn OperationTrait> = Box::new(DeleteOperation{...});
    /// let ans1 = tmp.downcast::<DeleteOperation>() // Some(&mut DeleteOperation{...})
    /// let ans2 = tmp.downcast::<InsertOperation>() // None
    /// ```
    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        match self.as_any_mut().downcast_mut::<T>() {
            Some(el) => Some(el),
            None => None,
        }
    }
}

type Position = usize;
type Revision = usize;

#[derive(Debug, Getters)]
pub struct InsertOperation {
    #[getter(skip)]
    position: Position,
    #[getter(skip)]
    revision: Revision,
    text: String,
}

impl InsertOperation {
    pub fn new(position: Position, revision: Revision, text: String) -> Self {
        Self {
            position: position,
            revision: revision,
            text: text,
        }
    }
}

#[derive(Debug, Getters)]
pub struct DeleteOperation {
    #[getter(skip)]
    position: Position,
    #[getter(skip)]
    revision: Revision,
    len: usize,
}

impl DeleteOperation {
    pub fn new(position: Position, revision: Revision, len: usize) -> Self {
        Self {
            position: position,
            revision: revision,
            len: len,
        }
    }
}

/// Calculates the intersection of two segments. Accepts the ends of the segments,
/// returns None if the segments do not intersect,
/// Some(x, y) if the segment \[x, y-1\] is the area of intersection.
///
/// `first_start`, `first_end`, `second_start`, `second_end` are passed **including the ends**.
/// ## Examples:
///
/// The first one starts at 0 and has a length of 5,
/// and the second one starts at 3 and has a length of 4:
///
/// ```ignore
/// assert_eq!(intersection(0, 0+5-1, 3, 3+4-1), Some((3, 5)))
/// ```
///
/// The first one starts at 5 and has a length of 6,
/// and the second one starts at 0 and has a length of 3:
/// ```ignore
/// assert_eq!(intersection(5, 5+6-1, 0, 0+3-1), None)
/// ```
fn intersection(
    first_start: usize,
    first_end: usize,
    second_start: usize,
    second_end: usize,
) -> Option<(usize, usize)> {
    if first_end < second_start || second_end < first_start {
        return None;
    }

    let start: usize = if first_start > second_start {
        first_start
    } else {
        second_start
    };

    let end: usize = if second_end < first_end {
        second_end
    } else {
        first_end
    };

    Some((start, end + 1))
}

impl OperationTrait for InsertOperation {
    fn apply(&self, text: &mut String) {
        let mut a = String::with_capacity(text.len() + self.text.len());
        a.push_str(&text[..self.position]);
        a.push_str(text);
        a.push_str(&text[self.position..]);
        *text = a; // very bed, idk why

        // text.insert_str(self.position, &self.text); // TODO: Need to find a faster way to work with strings !!! // very slow
    }

    fn apply_return(&self, text: &mut String) -> String {
        let mut a = String::with_capacity(text.len() + self.text.len());
        a.push_str(&text[..self.position]);
        a.push_str(text);
        a.push_str(&text[self.position..]);
        a
    }
    fn transform_relative_to(&mut self, operation: &dyn OperationTrait) {
        if let Some(other) = operation.downcast::<InsertOperation>() {
            if self.position >= other.position {
                self.position += other.text.len()
            }
        }
        if let Some(other) = operation.downcast::<DeleteOperation>() {
            if self.position > other.position {
                self.position -= min(other.len, self.position - other.position)
            }
        }
    }

    fn revision(&self) -> Revision {
        self.revision
    }

    fn set_revision(&mut self, revision: Revision) {
        self.revision = revision
    }

    fn position(&self) -> Position {
        self.position
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    #[cfg(debug_assertions)]
    fn print(&self) {
        print!("{:?}, ", self);
    }
}

impl OperationTrait for DeleteOperation {
    fn apply(&self, text: &mut String) {
        let mut tmp = String::with_capacity(text.len() - self.len);
        tmp.push_str(&text[..self.position]);
        tmp.push_str(&text[self.position + self.len..]);
        *text = tmp; // TODO  Need to find a faster way to work with strings !!!
    }

    fn apply_return(&self, text: &mut String) -> String {
        let mut tmp = String::with_capacity(text.len() - self.len);
        tmp.push_str(&text[..self.position]);
        tmp.push_str(&text[self.position + self.len..]);

        tmp
    }

    fn transform_relative_to(&mut self, operation: &dyn OperationTrait) {
        if let Some(other) = operation.downcast::<InsertOperation>() {
            if self.position >= other.position {
                self.position += other.text.len();
            }
        }
        if let Some(other) = operation.downcast::<DeleteOperation>() {
            let intersection_len = if let Some(el) = intersection(
                self.position,
                self.position + self.len - 1,
                other.position,
                other.position + other.len - 1,
            ) {
                el.1 - el.0
            } else {
                0
            };

            if intersection_len == self.len {
                self.len = 0;
            } else {
                self.len -= intersection_len;
                if self.position >= other.position {
                    self.position -= other.len - intersection_len;
                }
            }
        }
    }

    fn revision(&self) -> Revision {
        self.revision
    }

    fn set_revision(&mut self, revision: Revision) {
        self.revision = revision
    }

    fn position(&self) -> Position {
        self.position
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    #[cfg(debug_assertions)]
    fn print(&self) {
        print!("{:?}, ", self);
    }
}

#[cfg(test)]
mod tests {
    macro_rules! newStr {
        ($elem:expr) => {
            String::from($elem)
        };
    }
    use super::{intersection, DeleteOperation, InsertOperation, OperationTrait};

    #[test]
    fn intersection_test() {
        let cases = [
            (((0, 4), (5, 10)), (None)),
            (((5, 10), (0, 3)), (None)),
            (((0, 1), (5, 10)), (None)),
            (((0, 5), (5, 10)), (Some((5, 6)))),
            (((2, 4), (0, 10)), (Some((2, 5)))),
            (((4, 6), (0, 4)), (Some((4, 5)))),
            (((2, 5), (3, 5)), (Some((3, 6)))),
            (((2, 10), (5, 11)), (Some((5, 11)))),
            (((0, 10), (2, 5)), (Some((2, 6)))),
            (((0, 4), (3, 6)), (Some((3, 5)))),
        ];

        for (((first_start, first_end), (second_start, second_end)), ans) in cases {
            let tmp = intersection(first_start, first_end, second_start, second_end);
            assert_eq!(
                tmp, ans,
                "\nInterseption test. intersection({}, {}, {}, {}) must be {ans:?}, not {tmp:?}\n",
                first_start, first_end, second_start, second_end
            );
        }
    }

    #[test]
    fn insert_after_insert() {
        let cases = [
            (
                (
                    InsertOperation::new(0, 0, newStr!("123")),
                    InsertOperation::new(0, 0, newStr!("123")),
                ),
                (3),
            ),
            (
                (
                    InsertOperation::new(0, 0, newStr!("123")),
                    InsertOperation::new(4, 0, newStr!("123")),
                ),
                (7),
            ),
            (
                (
                    InsertOperation::new(4, 0, newStr!("123")),
                    InsertOperation::new(0, 0, newStr!("123")),
                ),
                (0),
            ),
            (
                (
                    InsertOperation::new(1, 0, newStr!("123")),
                    InsertOperation::new(0, 0, newStr!("123")),
                ),
                (0),
            ),
        ];

        for ((old_op, mut new_op), new_pos) in cases {
            new_op.transform_relative_to(&old_op);
            assert_eq!(
                new_op.position, new_pos,
                "Insert after insert test, old: {old_op:?}, new: {new_op:?}"
            );
        }
    }

    #[test]
    fn insert_after_delete() {
        let cases = [
            (
                (
                    DeleteOperation::new(0, 0, 2),
                    InsertOperation::new(0, 0, newStr!("123")),
                ),
                0,
            ),
            (
                (
                    DeleteOperation::new(0, 0, 2),
                    InsertOperation::new(2, 0, newStr!("123")),
                ),
                0,
            ),
            (
                (
                    DeleteOperation::new(0, 0, 2),
                    InsertOperation::new(3, 0, newStr!("123")),
                ),
                1,
            ),
            (
                (
                    DeleteOperation::new(0, 0, 2),
                    InsertOperation::new(6, 0, newStr!("123")),
                ),
                4,
            ),
            (
                (
                    DeleteOperation::new(4, 0, 2),
                    InsertOperation::new(3, 0, newStr!("123")),
                ),
                3,
            ),
            (
                (
                    DeleteOperation::new(5, 0, 2),
                    InsertOperation::new(0, 0, newStr!("123")),
                ),
                0,
            ),
        ];
        for ((old_op, mut new_op), new_pos) in cases {
            new_op.transform_relative_to(&old_op);
            assert_eq!(new_op.position, new_pos);
        }
    }

    #[test]
    fn delete_after_insert() {
        let cases = [
            (
                (
                    InsertOperation::new(0, 0, newStr!("123")),
                    DeleteOperation::new(0, 0, 2),
                ),
                3,
            ),
            (
                (
                    InsertOperation::new(5, 0, newStr!("123")),
                    DeleteOperation::new(0, 0, 2),
                ),
                0,
            ),
            (
                (
                    InsertOperation::new(0, 0, newStr!("123")),
                    DeleteOperation::new(1, 0, 2),
                ),
                4,
            ),
            (
                (
                    InsertOperation::new(0, 0, newStr!("1234")),
                    DeleteOperation::new(5, 0, 2),
                ),
                9,
            ),
        ];

        for ((old_op, mut new_op), new_pos) in cases {
            new_op.transform_relative_to(&old_op);
            assert_eq!(new_op.position, new_pos);
        }
    }

    #[test]
    fn delete_after_delete() {
        let cases = [
            (
                (DeleteOperation::new(0, 0, 5), DeleteOperation::new(0, 0, 5)),
                (0, 0),
            ), // 1
            (
                (DeleteOperation::new(0, 0, 3), DeleteOperation::new(5, 0, 5)),
                (2, 5),
            ), // 2
            (
                (DeleteOperation::new(0, 0, 6), DeleteOperation::new(3, 0, 5)),
                (0, 2),
            ), // 3
            (
                (DeleteOperation::new(0, 0, 5), DeleteOperation::new(1, 0, 4)),
                (1, 0),
            ), // 4
            (
                (DeleteOperation::new(4, 0, 3), DeleteOperation::new(0, 0, 5)),
                (0, 4),
            ), // 5
            (
                (DeleteOperation::new(3, 0, 2), DeleteOperation::new(0, 0, 9)),
                (0, 7),
            ), // 6
            (
                (DeleteOperation::new(0, 0, 5), DeleteOperation::new(1, 0, 9)),
                (0, 5),
            ), // 7
            (
                (DeleteOperation::new(0, 0, 5), DeleteOperation::new(1, 0, 3)),
                (1, 0),
            ), // 8
            //TODO: Check 8 case in go-serve-test
            (
                (DeleteOperation::new(5, 0, 5), DeleteOperation::new(1, 0, 3)),
                (1, 3),
            ), // 9
            (
                (DeleteOperation::new(4, 0, 2), DeleteOperation::new(0, 0, 9)),
                (0, 7),
            ), // 10
        ];

        for (idx, ((old_op, mut new_op), (new_pos, new_len))) in cases.into_iter().enumerate() {
            new_op.transform_relative_to(&old_op);
            assert_eq!(
                new_op.position,
                new_pos,
                "Wrong position in {} case! Old: {old_op:?}, new: {new_op:?}",
                idx + 1
            );
            assert_eq!(
                new_op.len,
                new_len,
                "Wrong length in {} case! Old: {old_op:?}, new: {new_op:?}",
                idx + 1
            );
        }
    }
}
