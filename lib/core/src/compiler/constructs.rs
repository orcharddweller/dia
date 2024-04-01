use swc_common::DUMMY_SP;
use swc_ecma_ast::*;

pub fn ident(name: &str) -> Ident {
    Ident::new(name.into(), DUMMY_SP)
}

pub fn var_name(name: &str) -> PropName {
    PropName::Ident(ident(name))
}

pub fn const_declare(ident: Ident, value: Box<Expr>) -> Decl {
    let declaration = VarDeclarator {
        span: DUMMY_SP,
        name: Pat::Ident(BindingIdent {
            id: ident,
            type_ann: None,
        }),
        init: Some(value),
        definite: false,
    };
    Decl::Var(Box::new(VarDecl {
        span: DUMMY_SP,
        kind: VarDeclKind::Const,
        declare: false,
        decls: vec![declaration],
    }))
}

pub fn expr_arrow_cb(body: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Arrow(ArrowExpr {
        span: DUMMY_SP,
        is_async: false,
        is_generator: false,
        params: vec![],
        body: Box::new(BlockStmtOrExpr::Expr(body)),
        type_params: None,
        return_type: None,
    }))
}

pub fn string(value: &str) -> Box<Expr> {
    Box::new(Expr::Lit(Lit::Str(Str {
        span: DUMMY_SP,
        value: value.into(),
        raw: None,
    })))
}

pub fn array(exprs: Vec<Box<Expr>>) -> Box<Expr> {
    let elems = exprs
        .into_iter()
        .map(|expr| Some(ExprOrSpread { spread: None, expr }))
        .collect();

    Box::new(Expr::Array(ArrayLit {
        span: DUMMY_SP,
        elems,
    }))
}

pub fn object(key_value_pairs: Vec<KeyValueProp>) -> Box<Expr> {
    let props = key_value_pairs
        .into_iter()
        .map(|prop| PropOrSpread::Prop(Box::new(Prop::KeyValue(prop))))
        .collect();

    Box::new(Expr::Object(ObjectLit {
        span: DUMMY_SP,
        props,
    }))
}

pub fn export(decl: Decl) -> ModuleItem {
    ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl {
        span: DUMMY_SP,
        decl,
    }))
}

pub fn satisfies(expr: Box<Expr>, type_ann: Box<TsType>) -> Box<Expr> {
    Box::new(Expr::TsSatisfies(TsSatisfiesExpr {
        span: DUMMY_SP,
        expr,
        type_ann,
    }))
}

pub fn as_const(expr: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::TsConstAssertion(TsConstAssertion {
        span: DUMMY_SP,
        expr,
    }))
}

pub fn paren(expr: Box<Expr>) -> Box<Expr> {
    Box::new(Expr::Paren(ParenExpr {
        span: DUMMY_SP,
        expr,
    }))
}

pub fn ts_type_ref(name: &str) -> Box<TsType> {
    Box::new(TsType::TsTypeRef(TsTypeRef {
        span: DUMMY_SP,
        type_name: TsEntityName::Ident(ident(name)),
        type_params: None,
    }))
}
