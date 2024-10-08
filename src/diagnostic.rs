use crate::prelude::*;

pub struct LintLsDiagnostic {
    pub source: String,
    pub line: u32,
    pub start_column: Option<u32>,
    pub end_column: Option<u32>,
    pub severity: Option<LintLsDiagnosticSeverity>,
    pub description: Option<String>,
}

impl From<LintLsDiagnostic> for Diagnostic {
    fn from(diag: LintLsDiagnostic) -> Self {
        let range = match (diag.start_column, diag.end_column) {
            (None, None) => Range {
                start: Position {
                    line: diag.line.saturating_sub(1),
                    character: 0,
                },
                end: Position {
                    line: diag.line.saturating_sub(1),
                    character: 0,
                },
            },
            (Some(column), None) | (None, Some(column)) => Range {
                start: Position {
                    line: diag.line.saturating_sub(1),
                    character: column.saturating_sub(1),
                },
                end: Position {
                    line: diag.line.saturating_sub(1),
                    character: column,
                },
            },
            (Some(start_column), Some(end_column)) => {
                let start_col = start_column.saturating_sub(1);
                Range {
                    start: Position {
                        line: diag.line.saturating_sub(1),
                        character: start_col,
                    },
                    end: Position {
                        line: diag.line.saturating_sub(1),
                        character: std::cmp::max(start_col + 1, end_column.saturating_sub(1)),
                    },
                }
            }
        };
        Self {
            range,
            severity: diag.severity.map(DiagnosticSeverity::from),
            code: None,
            code_description: None,
            source: Some(diag.source),
            message: diag.description.unwrap_or_else(|| "error".to_string()),
            related_information: None,
            tags: None,
            data: None,
        }
    }
}
