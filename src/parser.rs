use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct DiaParser;

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
    }
}
