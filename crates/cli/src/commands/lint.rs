use clap::Parser;

use mago_interner::ThreadedInterner;
use mago_reporting::reporter::Reporter;
use mago_reporting::Level;
use mago_service::config::Configuration;
use mago_service::linter::LintService;
use mago_service::source::SourceService;

use crate::utils::bail;

#[derive(Parser, Debug)]
#[command(
    name = "lint",
    about = "Lint the project according to the `mago.toml` configuration or default settings",
    long_about = r#"
Lint the project according to the `mago.toml` configuration or default settings.

This command analyzes the project's source code and highlights issues based on the defined linting rules.

If `mago.toml` is not found, the default configuration is used. The command outputs the issues found in the project."
    "#
)]
pub struct LintCommand {
    #[arg(long, short, help = "Only show fixable issues", default_value_t = false)]
    pub only_fixable: bool,
}

pub async fn execute(command: LintCommand, configuration: Configuration) -> i32 {
    let interner = ThreadedInterner::new();

    let source_service = SourceService::new(interner.clone(), configuration.source);
    let source_manager = source_service.load().await.unwrap_or_else(bail);

    let lint_service = LintService::new(configuration.linter, interner.clone(), source_manager.clone());
    let issues = lint_service.run().await.unwrap_or_else(bail);
    let issues_contain_errors = issues.get_highest_level().is_some_and(|level| level >= Level::Error);

    if command.only_fixable {
        Reporter::new(source_manager).report_all(issues.only_fixable());
    } else {
        Reporter::new(source_manager).report_all(issues);
    }

    if issues_contain_errors {
        1
    } else {
        0
    }
}
