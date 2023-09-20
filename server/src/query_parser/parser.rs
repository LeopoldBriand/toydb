use std::error::Error;
use pest::Parser;
use pest_derive::Parser;
use pest::iterators::Pair;
use serde_json::{json, Value};

use crate::file_manager::Manager;

use super::grammar::{Expression, Label, SetExpression};


#[derive(Parser)]
#[grammar = "grammar.pest"]
struct DBParser {}

pub fn parse(manager: &mut Manager, expression: &str) -> Result<Value, Box<dyn Error>> {
    let result = parse_expression(DBParser::parse(Rule::expression, expression)?.next().unwrap());
    match result {
        Expression::Get(label) => {
            match label {
                Label::Index(index) => {
                    let data = manager.get_index(index)?;
                    let docs: Vec<String> = data.docs.keys().cloned().collect();
                    Ok(json!({"id": data.id, "docs": docs}))
                },
                Label::Doc((index, doc)) => manager.get_doc(index, doc),
            }
        },
        Expression::Set(setter) => {
            if !manager.indices.contains_key(&setter.doc.0) {
                manager.create_index(setter.doc.0.clone())?;
            }
            let index = manager.indices.get(&setter.doc.0).unwrap();
            match index.docs.get_key_value(&setter.doc.1) {
                Some(doc) => {
                    manager.update_doc(setter.doc.0.clone(), doc.0.clone(), setter.data)?;
                    Ok(json!({"index": &setter.doc.0, "doc": &setter.doc.1}))
                },
                None => {
                    manager.create_doc(setter.doc.0.clone(), setter.doc.1.clone(), setter.data)?;
                    Ok(json!({"index": &setter.doc.0, "doc": &setter.doc.1}))
                },
            }
        },
        Expression::Delete(label) => {
            match label {
                Label::Index(index) => {
                    let docs: Vec<String> = manager.indices.get(&index).unwrap().docs.keys().cloned().collect();
                    manager.delete_index(index.clone())?;
                    Ok(json!({"index": index , "docs": docs}))
                },
                Label::Doc((index, doc)) => {
                    manager.delete_doc(index.clone(), doc.clone())?;
                    Ok(json!({"index": index, "doc": doc}))
                },
            }
        }
    }
}

fn parse_expression(pair: Pair<Rule>) -> Expression {
    match pair.as_rule() {
        Rule::get_expression => {
            let mut label:Vec<Value> = vec![];
            for pair in pair.into_inner() {
                label.push(parse_others(pair));
            }
            Expression::Get (
                match label.len() {
                    1 => Label::Index(label[0].as_str().unwrap().to_owned()),
                    2 => Label::Doc((label[0].as_str().unwrap().to_owned(), label[1].as_str().unwrap().to_owned())),
                    _ => unreachable!()
                }
            )
        },
        Rule::set_expression => {
            let mut parsed: Vec<Value> = vec![];
            for pair in pair.into_inner() {
                parsed.push(parse_others(pair));
            }
            Expression::Set (
                SetExpression {
                    doc: (parsed[0].as_str().unwrap().to_owned(), parsed[1].as_str().unwrap().to_owned()),
                    data: parsed[2].clone(),
                }
            )
        },
        Rule::delete_expression => {
            let mut label:Vec<Value> = vec![];
            for pair in pair.into_inner() {
                label.push(parse_others(pair));
            }
            Expression::Delete (
                match label.len() {
                    1 => Label::Index(label[0].as_str().unwrap().to_owned()),
                    2 => Label::Doc((label[0].as_str().unwrap().to_owned(), label[1].as_str().unwrap().to_owned())),
                    _ => unreachable!()
                }
                
            )
        },
        _ => unreachable!()
    }
}

fn parse_others(pair: Pair<Rule>) -> Value {
    let rule = pair.as_rule();
    match rule {
        Rule::index => {
            let p = pair.as_str().to_owned();
            Value::String(p)
        },
        Rule::doc => Value::String(pair.as_str().to_owned()),
        Rule::data => parse_data(pair.into_inner().next().unwrap()),
        _ => unreachable!()
    }
}

fn parse_data(pair: Pair<Rule>) -> Value {
    let rule = pair.as_rule();
    match rule {
        Rule::object => Value::Object(
            pair.into_inner()
                .map(|pair| {
                    let mut inner_rules = pair.into_inner();
                    let name = inner_rules
                        .next()
                        .unwrap()
                        .into_inner()
                        .next()
                        .unwrap()
                        .as_str()
                        .to_owned();
                    let value = parse_data(inner_rules.next().unwrap());
                    (name, value)
                })
                .collect(),
        ),
        Rule::array => Value::Array(pair.into_inner().map(parse_data).collect()),
        Rule::string => Value::String(pair.into_inner().next().unwrap().as_str().to_owned()),
        Rule::number => Value::Number(pair.as_str().parse().unwrap()),
        Rule::bool => Value::Bool(pair.as_str().parse().unwrap()),
        Rule::null => Value::Null,
        _ => unreachable!()
    }
}
