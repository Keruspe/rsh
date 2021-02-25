use crate::{ast, variable::Variable};

pub(crate) struct Parser;

impl Parser {
    pub(crate) fn parse(script: String) -> anyhow::Result<ast::Block> {
        let mut nodes = Vec::new();
        let mut n_line = 0;
        for line in script.lines() {
            n_line += 1;
            nodes.append(&mut Self::parse_line(n_line, line.trim())?);
        }
        Ok(nodes.into())
    }

    fn parse_line(n_line: u8, line: &str) -> anyhow::Result<Vec<ast::Node>> {
        if line.starts_with("let ") {
            Self::parse_decl(n_line, line)
        } else if line.starts_with("print ") {
            Ok(vec![Self::parse_print(n_line, line)?])
        } else if line.contains(" = ") {
            Ok(vec![Self::parse_assign(n_line, line)?])
        } else if line.is_empty() {
            Ok(Vec::new())
        } else {
            anyhow::bail!("Invalid instruction line {}: {}", n_line, line);
        }
    }

    fn parse_decl(n_line: u8, line: &str) -> anyhow::Result<Vec<ast::Node>> {
        let line = line.strip_prefix("let ").map_or_else(|| anyhow::bail!("Invalid instruction line {}: {}", n_line, line), Ok)?.trim();
        if line.contains(" = ") {
            let (variable, value) = Self::parse_assign_inner(n_line, line)?;
            Ok(vec![ast::Node::Decl(variable.clone()), ast::Node::Assign(variable, value)])
        } else if line.is_empty() {
            anyhow::bail!("Invalid vairable declaration line {}: missing name", n_line);
        } else {
            Ok(vec![ast::Node::Decl(line.to_string())])
        }
    }

    fn parse_print(n_line: u8, line: &str) -> anyhow::Result<ast::Node> {
        let variable = line.strip_prefix("print ").map_or_else(|| anyhow::bail!("Invalid instruction line {}: {}", n_line, line), Ok)?.trim();
        Ok(ast::Node::Print(variable.to_string()))
    }

    fn parse_assign(n_line: u8, line: &str) -> anyhow::Result<ast::Node> {
        let (variable, value) = Self::parse_assign_inner(n_line, line)?;
        Ok(ast::Node::Assign(variable, value))
    }

    fn parse_assign_inner(n_line: u8, line: &str) -> anyhow::Result<(String, Variable)> {
        let mut line = line.splitn(2, " = ");
        let variable = line.next().unwrap().trim();
        let value = line.next().map_or_else(|| anyhow::bail!("Invalid assignment line {} for variable {}: missing value", n_line, variable), Ok)?.trim();
        let value = value.strip_prefix("\"").and_then(|value| value.strip_suffix("\"")).map_or_else(|| anyhow::bail!("Invalid value line {} for variable {}: {}", n_line, variable, value), Ok)?;
        Ok((variable.to_string(), Variable::String(value.to_string())))
    }
}
