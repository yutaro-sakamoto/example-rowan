//! Simple COBOL parser using Rowan
//!
//! Supports:
//! - IDENTIFICATION DIVISION.
//! - PROGRAM-ID. <name>.
//! - PROCEDURE DIVISION.
//! - DISPLAY "<string>".

use rowan::{GreenNode, GreenNodeBuilder};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum SyntaxKind {
    // Tokens
    IDENTIFICATION_KW = 0,
    DIVISION_KW,
    PROGRAM_ID_KW,
    PROCEDURE_KW,
    DISPLAY_KW,
    DOT,
    STRING_LITERAL,
    IDENT,
    WHITESPACE,
    NEWLINE,
    ERROR,

    // Nodes
    ROOT,
    IDENTIFICATION_DIVISION,
    PROGRAM_ID_CLAUSE,
    PROCEDURE_DIVISION,
    DISPLAY_STMT,
}

use SyntaxKind::*;

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CobolLang {}

impl rowan::Language for CobolLang {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= DISPLAY_STMT as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

pub type SyntaxNode = rowan::SyntaxNode<CobolLang>;
pub type SyntaxToken = rowan::SyntaxToken<CobolLang>;

#[allow(unused)]
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

// ============================================================================
// Lexer
// ============================================================================

fn lex(text: &str) -> Vec<(SyntaxKind, String)> {
    let mut tokens = Vec::new();
    let mut chars = text.char_indices().peekable();

    while let Some((start, ch)) = chars.next() {
        match ch {
            '.' => tokens.push((DOT, ".".to_string())),
            '"' => {
                // String literal
                let mut s = String::from("\"");
                while let Some((_, c)) = chars.next() {
                    s.push(c);
                    if c == '"' {
                        break;
                    }
                }
                tokens.push((STRING_LITERAL, s));
            }
            '\n' => tokens.push((NEWLINE, "\n".to_string())),
            c if c.is_whitespace() => {
                let mut s = String::from(c);
                while let Some(&(_, c)) = chars.peek() {
                    if c.is_whitespace() && c != '\n' {
                        s.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push((WHITESPACE, s));
            }
            c if c.is_alphabetic() || c == '-' => {
                let mut word = String::from(c);
                while let Some(&(_, c)) = chars.peek() {
                    if c.is_alphanumeric() || c == '-' {
                        word.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let upper = word.to_uppercase();
                let kind = match upper.as_str() {
                    "IDENTIFICATION" => IDENTIFICATION_KW,
                    "DIVISION" => DIVISION_KW,
                    "PROGRAM-ID" => PROGRAM_ID_KW,
                    "PROCEDURE" => PROCEDURE_KW,
                    "DISPLAY" => DISPLAY_KW,
                    _ => IDENT,
                };
                tokens.push((kind, word));
            }
            _ => tokens.push((ERROR, ch.to_string())),
        }
    }

    tokens
}

// ============================================================================
// Parser
// ============================================================================

pub struct Parse {
    green_node: GreenNode,
    pub errors: Vec<String>,
}

impl Parse {
    pub fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }
}

struct Parser {
    tokens: Vec<(SyntaxKind, String)>,
    pos: usize,
    builder: GreenNodeBuilder<'static>,
    errors: Vec<String>,
}

impl Parser {
    fn new(tokens: Vec<(SyntaxKind, String)>) -> Self {
        Self {
            tokens,
            pos: 0,
            builder: GreenNodeBuilder::new(),
            errors: Vec::new(),
        }
    }

    fn current(&self) -> Option<SyntaxKind> {
        self.tokens.get(self.pos).map(|(k, _)| *k)
    }

    fn current_text(&self) -> Option<&str> {
        self.tokens.get(self.pos).map(|(_, t)| t.as_str())
    }

    fn bump(&mut self) {
        if let Some((kind, text)) = self.tokens.get(self.pos) {
            self.builder.token((*kind).into(), text.as_str());
            self.pos += 1;
        }
    }

    fn skip_ws(&mut self) {
        while matches!(self.current(), Some(WHITESPACE) | Some(NEWLINE)) {
            self.bump();
        }
    }

    fn expect(&mut self, kind: SyntaxKind) -> bool {
        if self.current() == Some(kind) {
            self.bump();
            true
        } else {
            self.errors.push(format!("Expected {:?}, found {:?}", kind, self.current()));
            false
        }
    }

    fn parse(mut self) -> Parse {
        self.builder.start_node(ROOT.into());

        self.skip_ws();

        // Parse IDENTIFICATION DIVISION.
        if self.current() == Some(IDENTIFICATION_KW) {
            self.parse_identification_division();
        }

        self.skip_ws();

        // Parse PROGRAM-ID. <name>.
        if self.current() == Some(PROGRAM_ID_KW) {
            self.parse_program_id();
        }

        self.skip_ws();

        // Parse PROCEDURE DIVISION.
        if self.current() == Some(PROCEDURE_KW) {
            self.parse_procedure_division();
        }

        self.builder.finish_node();
        Parse {
            green_node: self.builder.finish(),
            errors: self.errors,
        }
    }

    fn parse_identification_division(&mut self) {
        self.builder.start_node(IDENTIFICATION_DIVISION.into());
        self.bump(); // IDENTIFICATION
        self.skip_ws();
        self.expect(DIVISION_KW);
        self.expect(DOT);
        self.builder.finish_node();
    }

    fn parse_program_id(&mut self) {
        self.builder.start_node(PROGRAM_ID_CLAUSE.into());
        self.bump(); // PROGRAM-ID
        self.expect(DOT);
        self.skip_ws();
        if self.current() == Some(IDENT) {
            self.bump(); // program name
        } else {
            self.errors.push("Expected program name".to_string());
        }
        self.expect(DOT);
        self.builder.finish_node();
    }

    fn parse_procedure_division(&mut self) {
        self.builder.start_node(PROCEDURE_DIVISION.into());
        self.bump(); // PROCEDURE
        self.skip_ws();
        self.expect(DIVISION_KW);
        self.expect(DOT);

        self.skip_ws();

        // Parse statements
        while self.pos < self.tokens.len() {
            self.skip_ws();
            if self.current() == Some(DISPLAY_KW) {
                self.parse_display_stmt();
            } else if self.current().is_some() {
                // Unknown token, skip
                self.bump();
            } else {
                break;
            }
        }

        self.builder.finish_node();
    }

    fn parse_display_stmt(&mut self) {
        self.builder.start_node(DISPLAY_STMT.into());
        self.bump(); // DISPLAY
        self.skip_ws();
        if self.current() == Some(STRING_LITERAL) {
            self.bump();
        } else {
            self.errors.push("Expected string literal after DISPLAY".to_string());
        }
        // Optional dot
        if self.current() == Some(DOT) {
            self.bump();
        }
        self.builder.finish_node();
    }
}

pub fn parse(text: &str) -> Parse {
    let tokens = lex(text);
    Parser::new(tokens).parse()
}

// ============================================================================
// AST wrappers
// ============================================================================

macro_rules! ast_node {
    ($ast:ident, $kind:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $ast(SyntaxNode);

        impl $ast {
            pub fn cast(node: SyntaxNode) -> Option<Self> {
                if node.kind() == $kind {
                    Some(Self(node))
                } else {
                    None
                }
            }

            pub fn syntax(&self) -> &SyntaxNode {
                &self.0
            }
        }
    };
}

ast_node!(Root, ROOT);
ast_node!(IdentificationDivision, IDENTIFICATION_DIVISION);
ast_node!(ProgramIdClause, PROGRAM_ID_CLAUSE);
ast_node!(ProcedureDivision, PROCEDURE_DIVISION);
ast_node!(DisplayStmt, DISPLAY_STMT);

impl Root {
    pub fn identification_division(&self) -> Option<IdentificationDivision> {
        self.0.children().find_map(IdentificationDivision::cast)
    }

    pub fn program_id(&self) -> Option<ProgramIdClause> {
        self.0.children().find_map(ProgramIdClause::cast)
    }

    pub fn procedure_division(&self) -> Option<ProcedureDivision> {
        self.0.children().find_map(ProcedureDivision::cast)
    }
}

impl ProgramIdClause {
    pub fn name(&self) -> Option<String> {
        self.0
            .children_with_tokens()
            .filter_map(|el| el.into_token())
            .find(|t| t.kind() == IDENT)
            .map(|t| t.text().to_string())
    }
}

impl ProcedureDivision {
    pub fn display_statements(&self) -> impl Iterator<Item = DisplayStmt> + '_ {
        self.0.children().filter_map(DisplayStmt::cast)
    }
}

impl DisplayStmt {
    pub fn string_literal(&self) -> Option<String> {
        self.0
            .children_with_tokens()
            .filter_map(|el| el.into_token())
            .find(|t| t.kind() == STRING_LITERAL)
            .map(|t| {
                let text = t.text();
                // Remove quotes
                text[1..text.len() - 1].to_string()
            })
    }
}

impl Parse {
    pub fn root(&self) -> Option<Root> {
        Root::cast(self.syntax())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_cobol() {
        let source = r#"
IDENTIFICATION DIVISION.
PROGRAM-ID. HELLO.
PROCEDURE DIVISION.
    DISPLAY "Hello, World!".
"#;

        let parse = parse(source);
        assert!(parse.errors.is_empty(), "Errors: {:?}", parse.errors);

        let root = parse.root().expect("Should have root");

        // Check IDENTIFICATION DIVISION
        assert!(root.identification_division().is_some());

        // Check PROGRAM-ID
        let program_id = root.program_id().expect("Should have PROGRAM-ID");
        assert_eq!(program_id.name(), Some("HELLO".to_string()));

        // Check PROCEDURE DIVISION
        let proc_div = root.procedure_division().expect("Should have PROCEDURE DIVISION");
        let displays: Vec<_> = proc_div.display_statements().collect();
        assert_eq!(displays.len(), 1);
        assert_eq!(displays[0].string_literal(), Some("Hello, World!".to_string()));
    }

    #[test]
    fn test_parse_multiple_display() {
        let source = r#"
IDENTIFICATION DIVISION.
PROGRAM-ID. MULTI.
PROCEDURE DIVISION.
    DISPLAY "First".
    DISPLAY "Second".
    DISPLAY "Third".
"#;

        let parse = parse(source);
        assert!(parse.errors.is_empty(), "Errors: {:?}", parse.errors);

        let root = parse.root().unwrap();
        let proc_div = root.procedure_division().unwrap();
        let displays: Vec<_> = proc_div.display_statements().collect();

        assert_eq!(displays.len(), 3);
        assert_eq!(displays[0].string_literal(), Some("First".to_string()));
        assert_eq!(displays[1].string_literal(), Some("Second".to_string()));
        assert_eq!(displays[2].string_literal(), Some("Third".to_string()));
    }
}

pub fn main() {
    let source = r#"
IDENTIFICATION DIVISION.
PROGRAM-ID. HELLO.
PROCEDURE DIVISION.
    DISPLAY "Hello, World!".
"#;

    println!("=== COBOL Source ===");
    println!("{}", source);

    let parse = parse(source);

    println!("=== Parse Errors ===");
    if parse.errors.is_empty() {
        println!("No errors");
    } else {
        for err in &parse.errors {
            println!("  - {}", err);
        }
    }

    println!("\n=== Syntax Tree ===");
    println!("{:#?}", parse.syntax());

    println!("\n=== AST Information ===");
    if let Some(root) = parse.root() {
        if root.identification_division().is_some() {
            println!("Has IDENTIFICATION DIVISION");
        }

        if let Some(program_id) = root.program_id() {
            println!("PROGRAM-ID: {:?}", program_id.name());
        }

        if let Some(proc_div) = root.procedure_division() {
            println!("PROCEDURE DIVISION with statements:");
            for stmt in proc_div.display_statements() {
                println!("  DISPLAY {:?}", stmt.string_literal());
            }
        }
    }
}
