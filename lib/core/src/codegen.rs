use crate::{
    ast::{Choice, Module, Node, NodeName, SubNode, Text, TsExpression, TsModule},
    traits::Codegen,
};

impl Codegen for TsModule {
    fn generate(&self) -> String {
        self.raw().into()
    }
}

impl Codegen for TsExpression {
    fn generate(&self) -> String {
        self.raw().into()
    }
}

impl Codegen for Text {
    fn generate(&self) -> String {
        self.chunks().join("")
    }
}

impl Codegen for NodeName {
    fn generate(&self) -> String {
        self.as_str().into()
    }
}

impl Codegen for Choice {
    fn generate(&self) -> String {
        let text = self.text().generate();
        let then = self.then().as_ref().map(|then| then.generate());

        let mut code = "\n- ".to_string();

        code.push_str(&match then {
            Some(then) => format!("{} => {}", text, then),
            None => text,
        });

        code
    }
}

impl Codegen for SubNode {
    fn generate(&self) -> String {
        let mut code = self.text().generate();

        for choice in self.choices() {
            code.push_str(&choice.generate());
        }

        code
    }
}

impl Codegen for Node {
    fn generate(&self) -> String {
        let mut code = String::new();
        for sub_node in self.sub_nodes() {
            code.push_str(&sub_node.generate());
        }
        code
    }
}

impl Codegen for Module {
    fn generate(&self) -> String {
        let mut code = String::new();
        for (name, node) in self.nodes() {
            code.push_str(&format!("--- {} ---\n", name.generate()));
            code.push_str(&node.generate());
            code.push('\n');
        }
        code
    }
}
