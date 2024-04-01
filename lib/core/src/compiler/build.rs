use crate::{
    ast::{Choice, Module, Node, SubNode, Text, TsModule},
    swc_utils::parse_ts_module,
    traits::Compile,
};

use super::constructs::{
    array, as_const, const_declare, export, expr_arrow_cb, ident, object, paren, satisfies, string,
    ts_type_ref, var_name,
};

impl Compile<swc_ecma_ast::Module> for TsModule {
    fn to_ts_ast(&self) -> swc_ecma_ast::Module {
        parse_ts_module(self.raw())
    }
}

impl Compile<Box<swc_ecma_ast::Expr>> for Text {
    fn to_ts_ast(&self) -> Box<swc_ecma_ast::Expr> {
        array(self.chunks().iter().map(|chunk| string(chunk)).collect())
    }
}

impl Compile<Box<swc_ecma_ast::Expr>> for Choice {
    fn to_ts_ast(&self) -> Box<swc_ecma_ast::Expr> {
        let mut items = vec![swc_ecma_ast::KeyValueProp {
            key: var_name("text"),
            value: self.text().to_ts_ast(),
        }];

        if let Some(then) = self.then() {
            items.push(swc_ecma_ast::KeyValueProp {
                key: var_name("then"),
                value: Box::new(swc_ecma_ast::Expr::Ident(ident(then.as_str()))),
            });
        }

        object(items)
    }
}

impl Compile<Box<swc_ecma_ast::Expr>> for SubNode {
    fn to_ts_ast(&self) -> Box<swc_ecma_ast::Expr> {
        object(vec![
            swc_ecma_ast::KeyValueProp {
                key: var_name("text"),
                value: self.text().to_ts_ast(),
            },
            swc_ecma_ast::KeyValueProp {
                key: var_name("choices"),
                value: array(
                    self.choices()
                        .iter()
                        .map(|choice| choice.to_ts_ast())
                        .collect(),
                ),
            },
        ])
    }
}

impl Compile<Box<swc_ecma_ast::Expr>> for Node {
    fn to_ts_ast(&self) -> Box<swc_ecma_ast::Expr> {
        object(vec![swc_ecma_ast::KeyValueProp {
            key: var_name("subNodes"),
            value: array(
                self.sub_nodes()
                    .iter()
                    .map(|sub_node| sub_node.to_ts_ast())
                    .collect(),
            ),
        }])
    }
}

impl Compile<swc_ecma_ast::Module> for Module {
    fn to_ts_ast(&self) -> swc_ecma_ast::Module {
        let mut module = self.code().to_ts_ast();

        self.nodes()
            .iter()
            .map(|(name, node)| -> swc_ecma_ast::ModuleItem {
                export(const_declare(
                    ident(name.as_str()),
                    expr_arrow_cb(paren(satisfies(
                        as_const(node.to_ts_ast()),
                        ts_type_ref("Node"),
                    ))),
                ))
            })
            .for_each(|item| module.body.push(item));

        module
    }
}
