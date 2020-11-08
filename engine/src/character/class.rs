use crate::{Build, Id};

#[derive(Debug)]
pub enum ClassError {
    UnspecifiedName,
    UnspecifiedDescription,
}

#[derive(Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct ClassId(Id);

impl From<Id> for ClassId {
    fn from(id: Id) -> Self {
        ClassId(id)
    }
}

impl From<ClassId> for Id {
    fn from(id: ClassId) -> Self {
        id.0
    }
}

#[derive(Serialize, Deserialize)]
pub struct Class {
    name: String,
    description: String,
}

#[derive(Default)]
pub struct ClassBuilder {
    name: Option<String>,
    description: Option<String>,
}

impl Build for ClassBuilder {
    type Target = Class;
    type Error = ClassError;

    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<Self::Target, Self::Error> {
        Ok(Class {
            name: self.name.ok_or(ClassError::UnspecifiedName)?,
            description: self.description.ok_or(ClassError::UnspecifiedDescription)?,
        })
    }
}

impl ClassBuilder {
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }
}
