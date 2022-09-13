use serde_json::json;

fn main() -> anyhow::Result<()> {
    let mut json = json!({});
    // BUG: 字符串 parse解析不到对应的类型
    json["123"] = "abc".parse()?;

    println!("{}", json);
    Ok(())
}
