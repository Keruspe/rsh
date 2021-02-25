use crate::{ast, variable::Variable};

use std::{collections::HashMap, sync::Arc};

#[derive(Default)]
pub(crate) struct Context {
    parent: Option<Arc<Context>>,
    variables: HashMap<String, Variable>,
}

impl Context {
    pub(crate) fn new(parent: Option<Arc<Self>>) -> Self {
        Self {
            parent,
            variables: Default::default(),
        }
    }

    pub(crate) fn declare(&mut self, variable: String) {
        self.variables.insert(variable, Variable::Undefined);
    }

    pub(crate) fn assign(&mut self, variable: String, value: Variable) -> anyhow::Result<()> {
        if self.read(&variable).is_none() {
            anyhow::bail!("No such variable: {}", variable);
        }
        self.variables.insert(variable, value);
        Ok(())
    }

    pub(crate) fn read(&self, variable: &str) -> Option<&Variable> {
        self.variables.get(variable).or_else(|| self.parent.as_ref().and_then(|parent| parent.read(variable)))
    }

    pub(crate) fn run(mut self, block: ast::Block) -> anyhow::Result<()> {
        for node in block.into_iter() {
            match node {
                ast::Node::Decl(variable) => self.declare(variable),
                ast::Node::Assign(variable, value) => self.assign(variable, value)?,
                ast::Node::Print(variable) => println!("{}", self.read(&variable).map_or_else(|| anyhow::bail!("Unknown variable: {}", variable), Ok)?),
            }
        }
        Ok(())
    }
}
