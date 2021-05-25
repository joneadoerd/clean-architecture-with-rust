use adapter::id::{thought::Id, NewId};
use application::gateway::repository::thought::{Error, Repo, Result};
use entity::thought::Thought;
use std::{collections::HashMap, sync::RwLock};

#[derive(Default)]
pub struct InMemory {
    thoughts: RwLock<HashMap<Id, Thought>>,
}

impl Repo for InMemory {
    type Id = Id;
    fn save(&self, thought: Thought) -> Result<Self::Id> {
        let id = self.new_id()?;
        self.thoughts.write().unwrap().insert(id, thought);
        Ok(id)
    }
    fn get(&self, id: Self::Id) -> Result<Thought> {
        self.thoughts
            .read()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or(Error::NotFound)
    }
}

impl NewId<Id> for InMemory {
    type Err = Error;
    fn new_id(&self) -> Result<Id> {
        let next = self
            .thoughts
            .read()
            .unwrap()
            .iter()
            .map(|(id, _)| u32::from(*id))
            .max()
            .unwrap_or(0)
            + 1;
        Ok(Id::from(next))
    }
}
