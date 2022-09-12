use anyhow::Result;
use rs_xdiff::DiffConfig;

// folder name must be examples
// cargo run --example config
fn main() -> Result<()> {
    let content = include_str!("../fixtures/test.yml");
    let config = DiffConfig::from_yaml(content);

    println!("{:#?}", config);
    Ok(())
}
