use crate::utils::has_duplicates;

#[derive(Debug)]
pub struct TsModule {
    code: String,
}

impl TsModule {
    pub fn new(code: &str) -> Self {
        Self {
            code: code.to_string(),
        }
    }
}

pub struct TsExpression {
    code: String,
}
impl TsExpression {
    pub fn new(code: &str) -> Self {
        Self {
            code: code.to_string(),
        }
    }
}

pub enum TextChunk {
    Plain(String),
    Expression(TsExpression),
}

#[derive(Debug)]
pub struct Text {
    chunks: Vec<String>,
}

impl Text {
    pub fn new(chunks: Vec<String>) -> Self {
        Self { chunks }
    }
}

impl Default for Text {
    fn default() -> Self {
        Self { chunks: vec![] }
    }
}

#[derive(Debug)]
pub struct Choice {
    text: Text,
    then: Option<NodeName>,
}

impl Default for Choice {
    fn default() -> Self {
        Self {
            text: Text::default(),
            then: None,
        }
    }
}

impl Choice {
    pub fn new(text: Text, then: Option<NodeName>) -> Self {
        Self { text, then }
    }
}

#[derive(Debug)]
pub struct SubNode {
    text: Text,
    choices: Vec<Choice>,
}

impl SubNode {
    pub fn new(text: Text, choices: Vec<Choice>) -> Self {
        Self { text, choices }
    }
}

#[derive(Debug)]
pub struct Node {
    sub_nodes: Vec<SubNode>,
}

impl Node {
    pub fn new(sub_nodes: Vec<SubNode>) -> Self {
        Self { sub_nodes }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct NodeName(String);

impl NodeName {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

#[derive(Debug)]
pub struct Module {
    code: TsModule,
    nodes: Vec<(NodeName, Node)>,
}

impl Module {
    pub fn new(code: TsModule, nodes: Vec<(NodeName, Node)>) -> Self {
        let names = nodes.iter().map(|(name, _)| name).collect::<Vec<_>>();

        if has_duplicates(&names) {
            panic!("Duplicate node names");
        }

        Self { code, nodes }
    }
}
