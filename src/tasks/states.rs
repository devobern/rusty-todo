use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskState {
    Pending,
    Started, 
    Done
}