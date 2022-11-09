use serde::{Serialize, Deserialize};
use types::Type;

use crate::{block::ABasicBlock, scope::Scope, stmt::LinkId, var::AVar};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControlFlowGraph {
    // function parameters
    pub parameters: Vec<Type>,
    // implicit parameters that captured from outer scope.
    pub captured: Vec<Type>,
    pub output: Type,
    // first N items in vars are arguments.
    pub vars: Vec<AVar>,
    pub scopes: Vec<Scope>,
    pub blocks: Vec<ABasicBlock>,
    pub links: Vec<LinkId>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ControlFlowGraphId(pub usize);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Amir {
    pub entrypoint: ControlFlowGraphId,
    pub cfgs: Vec<ControlFlowGraph>,
}

impl ControlFlowGraph {
    pub fn get_type(&self) -> Type {
        Type::Function {
            parameters: self.parameters.clone(),
            body: Box::new(self.output.clone()),
        }
    }
}
