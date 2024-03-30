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

    pub fn code(&self) -> &str {
        &self.code
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

    pub fn code(&self) -> &str {
        &self.code
    }
}

pub enum TextChunk {
    Plain(String),
    Expression(TsExpression),
}

#[derive(Debug, Default)]
pub struct Text {
    chunks: Vec<String>,
}

impl Text {
    pub fn new(chunks: Vec<String>) -> Self {
        Self { chunks }
    }

    pub fn chunks(&self) -> &[String] {
        &self.chunks
    }
}

#[derive(Debug)]
pub struct Choice {
    text: Text,
    then: Option<NodeName>,
}

impl Choice {
    pub fn new(text: Text, then: Option<NodeName>) -> Self {
        Self { text, then }
    }

    pub fn then(&self) -> Option<&NodeName> {
        self.then.as_ref()
    }

    pub fn text(&self) -> &Text {
        &self.text
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

    pub fn text(&self) -> &Text {
        &self.text
    }

    pub fn choices(&self) -> &[Choice] {
        &self.choices
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

    pub fn sub_nodes(&self) -> &[SubNode] {
        &self.sub_nodes
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct NodeName(String);

impl NodeName {
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
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

    pub fn code(&self) -> &TsModule {
        &self.code
    }

    pub fn nodes(&self) -> &[(NodeName, Node)] {
        &self.nodes
    }
}
