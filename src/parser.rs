use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::ast::{Choice, Module, Node, NodeName, SubNode, Text, TsModule};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct DiaParser;

fn parse_node_name(pair: Pair<Rule>) -> NodeName {
    assert_eq!(pair.as_rule(), Rule::node_name);

    NodeName::new(pair.as_str())
}

fn parse_node_header(pair: Pair<Rule>) -> NodeName {
    assert_eq!(pair.as_rule(), Rule::node_header);

    let identifier = pair.into_inner().next().unwrap();

    parse_node_name(identifier)
}

fn parse_then(pair: Pair<Rule>) -> NodeName {
    assert_eq!(pair.as_rule(), Rule::then);

    let node_name = pair.into_inner().next().unwrap();

    parse_node_name(node_name)
}

fn parse_choice(pair: Pair<Rule>) -> Choice {
    assert_eq!(pair.as_rule(), Rule::choice);

    let mut text: Option<Text> = None;
    let mut then: Option<NodeName> = None;

    for child in pair.into_inner() {
        match child.as_rule() {
            Rule::text => {
                if text.is_none() {
                    text = Some(parse_text(child));
                } else {
                    panic!("Multiple text blocks in choice");
                }
            }
            Rule::then => {
                if then.is_none() {
                    then = Some(parse_then(child));
                } else {
                    panic!("Multiple then blocks in choice");
                }
            }
            r => panic!("Unexpected rule: {:?}", r),
        }
    }

    Choice::new(text.unwrap(), then)
}

fn parse_line(pair: Pair<Rule>) -> String {
    assert_eq!(pair.as_rule(), Rule::line);

    pair.as_str().to_string()
}

fn parse_text(pair: Pair<Rule>) -> Text {
    assert_eq!(pair.as_rule(), Rule::text);

    let mut lines = vec![];

    for child in pair.into_inner() {
        lines.push(parse_line(child));
    }

    Text::new(lines)
}

fn parse_choices(pair: Pair<Rule>) -> Vec<Choice> {
    assert_eq!(pair.as_rule(), Rule::choices);

    let mut choices = vec![];

    for child in pair.into_inner() {
        choices.push(parse_choice(child));
    }

    choices
}

fn parse_sub_node(pair: Pair<Rule>) -> SubNode {
    assert_eq!(pair.as_rule(), Rule::sub_node);

    let mut text: Option<Text> = None;
    let mut choices: Option<Vec<Choice>> = None;

    for child in pair.into_inner() {
        match child.as_rule() {
            Rule::text => {
                if text.is_none() {
                    text = Some(parse_text(child));
                } else {
                    panic!("Multiple text blocks in sub node");
                }
            }
            Rule::choices => {
                if choices.is_none() {
                    choices = Some(parse_choices(child));
                } else {
                    panic!("Multiple choices blocks in sub node");
                }
            }
            _ => unreachable!(),
        }
    }

    SubNode::new(text.unwrap_or_default(), choices.unwrap_or_default())
}

fn parse_sub_nodes(pair: Pair<Rule>) -> Vec<SubNode> {
    assert_eq!(pair.as_rule(), Rule::sub_nodes);

    let mut sub_nodes = vec![];

    for child in pair.into_inner() {
        sub_nodes.push(parse_sub_node(child));
    }

    sub_nodes
}

fn parse_node_body(pair: Pair<Rule>) -> Node {
    assert_eq!(pair.as_rule(), Rule::node_body);

    let mut sub_nodes: Option<Vec<SubNode>> = None;

    for child in pair.into_inner() {
        match child.as_rule() {
            Rule::sub_nodes => {
                if sub_nodes.is_none() {
                    sub_nodes = Some(parse_sub_nodes(child));
                } else {
                    panic!("Multiple sub nodes in node body");
                }
            }
            Rule::then => {}
            _ => unreachable!(),
        }
    }

    Node::new(sub_nodes.unwrap())
}

fn parse_top_level_node(pair: Pair<Rule>) -> (NodeName, Node) {
    assert_eq!(pair.as_rule(), Rule::node);

    let mut name: Option<NodeName> = None;
    let mut node: Option<Node> = None;

    for child in pair.into_inner() {
        match child.as_rule() {
            Rule::node_header => {
                if name.is_none() {
                    name = Some(parse_node_header(child));
                } else {
                    panic!("Multiple node names in node");
                }
            }
            Rule::node_body => {
                if node.is_none() {
                    node = Some(parse_node_body(child))
                } else {
                    panic!("Multiple node bodies in node");
                }
            }
            _ => unreachable!(),
        }
    }

    let name = name.unwrap();
    let node = node.unwrap();

    (name, node)
}

fn parse_module(pair: Pair<Rule>) -> Module {
    assert_eq!(pair.as_rule(), Rule::module);

    let mut code: Option<&str> = None;
    let mut nodes = vec![];

    for child in pair.into_inner() {
        match child.as_rule() {
            Rule::ts_code => {
                if code.is_none() {
                    code = Some(child.as_str());
                } else {
                    panic!("Multiple code blocks in module");
                }
            }
            Rule::node => {
                nodes.push(parse_top_level_node(child));
            }
            Rule::EOI => {}
            _ => unreachable!(),
        }
    }

    Module::new(TsModule::new(code.unwrap_or_default()), nodes)
}

pub fn parse(source: &str) -> Module {
    let mut pairs = DiaParser::parse(Rule::module, source).unwrap();

    assert!(pairs.len() == 1);

    let pair = pairs.next().unwrap();

    parse_module(pair)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::read_test_file;
    use pest::Parser;

    macro_rules! did_parse_test {
        ($($test_name:ident: $value:expr,)*) => [
        $(
            #[test]
            fn $test_name() {
                let (rule, path) = $value;

                let input = read_test_file(&format!("parser/{}", path));

                let result = DiaParser::parse(rule, &input);

                assert!(result.is_ok());

                let mut result = result.unwrap();

                let pair = result.next().unwrap();

                let span = pair.as_span();

                assert_eq!(span.as_str(), input);
            }
        )*
        ]
    }

    // expression

    did_parse_test! {
        expression_identifier: (Rule::expression, "expression/positive/identifier.txt"),
        expression_sum: (Rule::expression, "expression/positive/sum.txt"),
        expression_tricky_string: (Rule::expression, "expression/positive/tricky_string.txt"),
        expression_tricky_template: (Rule::expression, "expression/positive/tricky_template.txt"),
        expression_tricky_template2: (Rule::expression, "expression/positive/tricky_template2.txt"),
        expression_tricky_template3: (Rule::expression, "expression/positive/tricky_template3.txt"),
        expression_object: (Rule::expression, "expression/positive/object.txt"),
        expression_arrow_function: (Rule::expression, "expression/positive/arrow_function.txt"),
    }

    // module

    did_parse_test! {
        module_simple: (Rule::module, "module/positive/simple.dia"),
        module_2nodes: (Rule::module, "module/positive/2nodes.dia"),
        module_choices: (Rule::module, "module/positive/choices.dia"),
        module_then: (Rule::module, "module/positive/then.dia"),
        module_expression: (Rule::module, "module/positive/expression.dia"),
    }
}
