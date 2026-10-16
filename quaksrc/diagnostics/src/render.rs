use crate::{Diagnostic, Severity};
use ariadne::{Label as AriadneLabel, Report, ReportKind, Source};
pub fn render(diagnostic: &Diagnostic, source: &str, file: &str) {
    let kind = match diagnostic.severity {
        Severity::Error => ReportKind::Error,
        Severity::Warning => ReportKind::Warning,
    };
    let range = match diagnostic.labels.first() {
        Some(label) => label.span.start..label.span.end,
        None => 0..0,
    };
    let mut report = Report::build(kind, (file, range)).with_message(&diagnostic.message);
    for label in &diagnostic.labels {
        report = report.with_label(
            AriadneLabel::new((file, label.span.start..label.span.end))
                .with_message(&label.message),
        );
    }
    if let Some(hint) = &diagnostic.hint {
        report = report.with_help(hint);
    }
    if let Some(errc) = &diagnostic.errorcode {
        report = report.with_code(errc)
    }
    report
        .finish()
        .eprint((file, Source::from(source)))
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Span;

    #[test]
    fn test_diagnostic_rendering() {
        let source = "*fn {\n     Inp (string) x; x.power(3);\n}";
        let error_span = Span::new(27, 37);

        let diagnostic = Diagnostic::error("no method named `power` found for type `string`")
            .with_code("Q0001")
            .with_label(error_span, "method not found in `string`")
            .with_hint("Method `power` is invalid for type `string`.");
        render(&diagnostic, source, "test_errors.qk");
    }
}
