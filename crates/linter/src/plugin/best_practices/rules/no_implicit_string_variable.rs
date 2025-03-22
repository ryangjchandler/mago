use indoc::indoc;

use mago_ast::{Node, StringPart};
use mago_reporting::{Annotation, Issue, Level};
use mago_span::HasSpan;

use crate::{context::LintContext, definition::{RuleDefinition, RuleUsageExample}, directive::LintDirective, rule::Rule};

#[derive(Clone, Debug)]
pub struct NoImplicitStringVariableRule;

impl Rule for NoImplicitStringVariableRule {
    fn get_definition(&self) -> crate::definition::RuleDefinition {
        RuleDefinition::enabled("No Implicit String Variable", Level::Note)
            .with_description(indoc! {"
                Flags any occurences of implicit (unbraced) variables inside of a double-quoted string literal.
            "})
            .with_example(RuleUsageExample::invalid(
                "Using an implicit variable inside of a double-quoted string literal",
                indoc! {r#"
                    <?php

                    $name = "Alice";

                    echo "Hello, $name";
                "#},
            ))
            .with_example(RuleUsageExample::valid(
                "Using an explicit variable inside of a double-quoted string literal",
                indoc! {r#"
                    <?php

                    $name = "Alice";

                    echo "Hello, {$name}";
                "#},
            ))
    }

    fn lint_node(&self, node: Node<'_>, context: &mut LintContext<'_>) -> LintDirective {
        let Node::StringPart(StringPart::Expression(string_part_expression)) = node else {
            return LintDirective::default()
        };
        
        let code = context.lookup(&context.module.source.content);
        let expression = &code[string_part_expression.span().to_range()];

        context.report(
            Issue::new(context.level(), "Avoid using implicit variables in a double-quoted string literal.")
                .with_annotation(
                    Annotation::primary(string_part_expression.span())
                        .with_message("Consider wrapping this expression in braces to make it explicit.")
                )
                .with_help(format!("Did you mean `{{{}}}` instead?", expression))
        );

        LintDirective::default()
    }
}
