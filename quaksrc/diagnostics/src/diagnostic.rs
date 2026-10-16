use crate::span::Span;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone)]
pub struct Label {
    pub span: Span,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub errorcode: Option<String>,
    pub message: String,
    pub labels: Vec<Label>,
    pub hint: Option<String>,
}
impl Diagnostic {
    fn new(severity: Severity, message: &str) -> Self {
        Diagnostic {
            severity,
            errorcode: None,
            message: message.to_string(),
            labels: Vec::new(),
            hint: None,
        }
    }
    pub fn error(message: &str) -> Self {
        Self::new(Severity::Error, message)
    }
    pub fn warning(message: &str) -> Self {
        Self::new(Severity::Warning, message)
    }
    pub fn with_label(mut self, span: Span, message: &str) -> Self {
        self.labels.push(Label {
            span,
            message: message.to_string(),
        });
        self
    }
    pub fn with_hint(mut self, message: &str) -> Self {
        self.hint = Some(message.to_string());
        self
    }
    pub fn with_code(mut self, errc: &str) -> Self {
        self.errorcode = Some(errc.to_string());
        self
    }
}
