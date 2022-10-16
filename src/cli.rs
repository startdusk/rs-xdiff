use clap::{Parser, Subcommand};

use crate::ExtraArgs;

/// Diff two http requests and compare the difference of the responses
#[derive(Parser, Debug, Clone)]
#[clap(version, author, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug, Clone)]
#[non_exhaustive]
pub enum Action {
    /// Diff two API responses based on given profile
    Run(RunArgs),
    /// Parse URLs to generate a profile
    Parse,
}

#[derive(Parser, Debug, Clone)]
pub struct RunArgs {
    /// Profile name
    #[clap(short, long, value_parser)]
    pub profile: String,

    /// Overrides args. Could be used to override the query, headers and body of the request.
    /// For query params, use `-e key=value`.
    /// For headers, use `-e %key=value`.
    /// For body, use `-e @key=value`.
    #[clap(short, long, value_parser = parse_key_val, number_of_values = 1)]
    pub extra_params: Vec<KeyVal>,

    /// Configuration to use.
    #[clap(short, long, value_parser)]
    pub config: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyValType {
    Query,
    Header,
    Body,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyVal {
    key_type: KeyValType,
    key: String,
    value: String,
}

fn parse_key_val(s: &str) -> anyhow::Result<KeyVal> {
    let mut parts = s.splitn(2, '=');
    let key = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("Invalid key value pair(key parse)"))?
        .trim();
    let value = parts
        .next()
        .ok_or_else(|| anyhow::anyhow!("Invalid key value pair(value parse)"))?
        .trim();

    let (key_type, key) = match key.chars().next() {
        Some('%') => (KeyValType::Header, &key[1..]),
        Some('@') => (KeyValType::Body, &key[1..]),
        Some(v) if v.is_ascii_alphabetic() => (KeyValType::Query, key),
        _ => return Err(anyhow::anyhow!("Invalid key value pair")),
    };

    Ok(KeyVal {
        key_type,
        key: key.to_string(),
        value: value.to_string(),
    })
}

impl From<Vec<KeyVal>> for ExtraArgs {
    fn from(args: Vec<KeyVal>) -> Self {
        let mut headers = vec![];
        let mut query = vec![];
        let mut body = vec![];

        for arg in args {
            match arg.key_type {
                KeyValType::Header => headers.push((arg.key, arg.value)),
                KeyValType::Body => body.push((arg.key, arg.value)),
                KeyValType::Query => query.push((arg.key, arg.value)),
            }
        }
        Self {
            headers,
            query,
            body,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_vec_key_val_for_extra_args_should_work() {
        let args = vec![
            KeyVal {
                key_type: KeyValType::Header,
                key: "header_key".into(),
                value: "header_val".into(),
            },
            KeyVal {
                key_type: KeyValType::Query,
                key: "query_key".into(),
                value: "query_val".into(),
            },
            KeyVal {
                key_type: KeyValType::Body,
                key: "body_key".into(),
                value: "body_val".into(),
            },
        ];
        let extra_args = ExtraArgs::from(args);
        assert_eq!(
            extra_args,
            ExtraArgs {
                headers: vec![("header_key".into(), "header_val".into())],
                query: vec![("query_key".into(), "query_val".into())],
                body: vec![("body_key".into(), "body_val".into())]
            }
        )
    }

    #[test]
    fn parse_key_val_should_work() {
        let args = vec!["%key1=value1", "key2=value2", "@key3=value3", "key4=value4"];

        let key_vals = args
            .into_iter()
            .map(|arg| parse_key_val(arg))
            .collect::<anyhow::Result<Vec<_>>>()
            .unwrap();

        assert_eq!(
            key_vals,
            vec![
                KeyVal {
                    key_type: KeyValType::Header,
                    key: "key1".into(),
                    value: "value1".into(),
                },
                KeyVal {
                    key_type: KeyValType::Query,
                    key: "key2".into(),
                    value: "value2".into()
                },
                KeyVal {
                    key_type: KeyValType::Body,
                    key: "key3".into(),
                    value: "value3".into()
                },
                KeyVal {
                    key_type: KeyValType::Query,
                    key: "key4".into(),
                    value: "value4".into()
                }
            ]
        );
    }
}
