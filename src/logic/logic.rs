use crate::prelude::*;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum ActionType {
    Force(String),

    Recursive(String),

    Interactive(String),

    Directories(String),
}

impl From<&str> for ActionType {
    fn from(action: &str) -> Self {
        match action {
            "force" => ActionType::Force(action.into()),
            "recursive" => ActionType::Recursive(action.into()),
            "interactive" => ActionType::Interactive(action.into()),
            "directories" => ActionType::Directories(action.into()),
            _ => unreachable!(),
        }
    }
}

impl From<String> for ActionType {
    fn from(action: String) -> Self {
        match action.as_str() {
            "force" => ActionType::Force(action),
            "recursive" => ActionType::Recursive(action),
            "interactive" => ActionType::Interactive(action),
            "directories" => ActionType::Directories(action),
            _ => unreachable!(),
        }
    }
}

impl AsRef<str> for ActionType {
    fn as_ref(&self) -> &str {
        match self {
            ActionType::Force(action) => action.as_str(),
            ActionType::Recursive(action) => action.as_str(),
            ActionType::Interactive(action) => action.as_str(),
            ActionType::Directories(action) => action.as_str(),
        }
    }
}

pub struct Logic {
    pub actions: Vec<ActionType>,
    pub directory: PathBuf,
    buffered_actions: Vec<InternalAction<ActionType>>,
}

impl Logic {
    pub fn new(action: &[ActionType], directory: PathBuf) -> Self {
        let mut actions: Vec<ActionType> = Vec::new();

        for s_action in action {
            let action_type = ActionType::from(s_action.as_ref());
            actions.push(action_type);
        }

        Self {
            actions,
            directory,
            buffered_actions: vec![],
        }
    }
}

#[derive(Debug, Clone)]
enum InternalAction<T> {
    Force(T),
    Recursive(T),
    Interactive(T),
    Directories(T),
}

trait FromActionType<T>
where
    T: Clone,
{
    fn from_action_type(action: ActionType) -> InternalAction<ActionType>;
}

impl FromActionType<ActionType> for InternalAction<ActionType> {
    fn from_action_type(action: ActionType) -> InternalAction<ActionType> {
        match action {
            ActionType::Force(action) => InternalAction::Force(action.into()),
            ActionType::Recursive(action) => InternalAction::Recursive(action.into()),
            ActionType::Interactive(action) => InternalAction::Interactive(action.into()),
            ActionType::Directories(action) => InternalAction::Directories(action.into()),
        }
    }
}

impl Logic {
    pub fn take_action(&self) -> Result<()> {
        while let Some(action) = self.actions.iter().next() {
            match action {
                ActionType::Force(action) => self.with_force(),
                ActionType::Recursive(action) => self.with_recursive(),
                ActionType::Interactive(action) => self.with_interactive(),
                ActionType::Directories(action) => self.with_directories(),
            };
        }

        // This will take a parsed action ->>> match and call rel method
        Ok(())
    }

    fn append_action(&mut self, action: ActionType) {
        self.buffered_actions
            .push(InternalAction::from_action_type(action));
    }

    fn with_force(&self) -> Result<()> {
        todo!()
    }

    fn with_recursive(&self) -> Result<()> {
        todo!()
    }

    fn with_interactive(&self) -> Result<()> {
        todo!()
    }

    fn with_directories(&self) -> Result<()> {
        todo!()
    }
}
