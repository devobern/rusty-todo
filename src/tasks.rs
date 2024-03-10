mod states;
use states::TaskState;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    id: u32,
    description: String,
    state: TaskState
}

impl Task {
    pub fn new(id: u32, description :String) -> Task {
        Task {
            id,
            description,
            state: TaskState::Pending,
        }
    }

    pub fn task_done(&mut self){
        self.state = TaskState::Done;
    }

    pub fn task_start(&mut self){
        self.state = TaskState::Started;
    }

    pub fn task_list(&self){
        println!("{}: {:?} - {:?}", self.id, self.description, self.state);
    }

    pub fn get_id(&self) -> u32{
        self.id
    }

    pub fn get_desc(&self) -> &str{
        &self.description
    }
}