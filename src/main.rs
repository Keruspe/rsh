mod ast;
mod context;
mod parser;
mod variable;

fn main() -> anyhow::Result<()> {
    let script = std::env::args().nth(1).map_or_else(|| anyhow::bail!("Missing argument: script"), Ok)?;
    let script = std::fs::read_to_string(script)?;
    let script = parser::Parser::parse(script)?;
    let context = context::Context::default();
    context.run(script)
}
