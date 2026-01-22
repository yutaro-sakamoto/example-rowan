#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
enum SyntaxKind {
    L_PAREN = 0,
    R_PAREN,
    WORD,
    WHITESPACE,
    ERROR,

    LIST,
    ATOM,
    ROOT,
}

use SyntaxKind::*;

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        Self(kind as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Lang{}
impl rowan::Language for Lang {
    type Kind = SyntaxKind;
    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= ROOT as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

use rowan::GreenNode;

use rowan::GreenNodeBuilder;

struct Parse {
    green_node: GreenNode,
    #[allow(unused)]
    errors: Vec<String>,
}

fn parse(text: &str) -> Parse {
    struct Parser {
        tokens: Vec<(SyntaxKind, String)>,
        builder: GreenNodeBuilder<'static>,
        errors: Vec<String>,
    }

    enum SexpRes {
        Ok,
        Eof,
        RParen,
    }

    impl Parser {
        fn parse(mut self) -> Parse {
            self.builder.start_node(ROOT.into());
            loop {
                match self.sexp() {
                    SexpRes::Eof => break,
                    SexpRes::RParen => {
                        self.builder.start_node(ERROR.into());
                        self.errors.push("unmatched `)".to_string());
                        self.bump();
                        self.builder.finish_node();
                    }
                    SexpRes::Ok => (),
                }
            }

            self.skip_ws();
            self.builder.finish_node();

            Parse { green_node: self.builder.finish(), errors: self.errors }
        }

        fn list(&mut self) {
            assert_eq!(self.current(), Some(L_PAREN));

            self.builder.start_node(LIST.into());
            self.bump();
            loop {
                match self.sexp() {
                    SexpRes::Eof => {
                        self.errors.push("expected `)`".to_string());
                        break;
                    }
                    SexpRes::RParen => {
                        self.bump();
                        break;
                    }
                    SexpRes::Ok => (),
                }
            }
            self.builder.finish_node();
        }

        fn sexp(&mut self) -> SexpRes {
            self.skip_ws();
            let t = match self.current() {
                None => return SexpRes::Eof,
                Some(R_PAREN) => return SexpRes::RParen,
                Some(t) => t,
            };
            match t {
                L_PAREN => self.list(),
                WORD => {
                    self.builder.start_node(ATOM.into());
                    self.bump();
                    self.builder.finish_node();
                }
                ERROR => self.bump(),
                _ => unreachable!(),
            }
            SexpRes::Ok
        }

        fn bump(&mut self) {
            let (kind, text) = self.tokens.pop().unwrap();
            self.builder.token(kind.into(), text.as_str());
        }

        fn current(&self) -> Option<SyntaxKind> {
            self.tokens.last().map(|(kind, _)| *kind)
        }

        fn skip_ws(&mut self) {
            while self.current() == Some(WHITESPACE) {
                self.bump()
            }
        }
    }

    let mut tokens = lex(text);
    tokens.reverse();
    Parser { tokens, builder: GreenNodeBuilder::new(), errors: Vec::new() }.parse()
}

type SyntaxNode = rowan::SyntaxNode<Lang>;

#[allow(unused)]
type SyntaxToken = rowan::SyntaxToken<Lang>;

#[allow(unused)]
type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

impl Parse {
    fn syntax(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green_node.clone())
    }
}

macro_rules! ast_node {
    ($ast:ident, $kind:ident) => {
        #[derive(PartialEq, Eq, Hash)]
        #[repr(transparent)]
        struct $ast(SyntaxNode);
        impl $ast {
            #[allow(unused)]
            fn cast(node: SyntaxNode) -> Option<Self> {
                if node.kind() == $kind { Some(Self(node)) } else { None }
            }
        }
    }
}

ast_node!(Root, ROOT);
ast_node!(List, LIST);
ast_node!(Atom, ATOM);

#[derive(PartialEq, Eq, Hash)]
#[repr(transparent)]
struct Sexp(SyntaxNode);

enum SexpKind {
    Atom(Atom),
    List(List),
}

impl Sexp {
    fn cast(node: SyntaxNode) -> Option<Self> {
        if Atom::cast(node.clone()).is_some() || List::cast(node.clone()).is_some() {
            Some(Sexp(node))
        } else {
            None
        }
    }

    fn kind(&self) -> SexpKind {
        Atom::cast(self.0.clone())
            .map(SexpKind::Atom)
            .or_else(|| List::cast(self.0.clone()).map(SexpKind::List))
            .unwrap()
    }
}

impl Root {
    fn sexps(&self) -> impl Iterator<Item = Sexp> + '_ {
        self.0.children().filter_map(Sexp::cast)
    }
}

enum Op {
    Add,
    Sub,
    Div,
    Mul,
}

impl Atom {
    fn eval(&self) -> Option<i64> {
        self.text().parse().ok()
    }

    fn as_op(&self) -> Option<Op> {
        let op = match self.text().as_str() {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "/" => Op::Div,
            _ => return None,
        };
        Some(op)
    }

    fn text(&self) -> String {
        match self.0.green().children().next() {
            Some(rowan::NodeOrToken::Token(token)) => token.text().to_string(),
            _ => unreachable!(),
        }
    }
}

impl List {
    fn sexps(&self) -> impl Iterator<Item = Sexp> + '_ {
        self.0.children().filter_map(Sexp::cast)
    }

    fn eval(&self) -> Option<i64> {
        let op = match self.sexps().nth(0)?.kind() {
            SexpKind::Atom(atom) => atom.as_op()?,
            _ => return None,
        };
        let arg1 = self.sexps().nth(1)?.eval()?;
        let arg2 = self.sexps().nth(2)?.eval()?;
        let res = match op {
            Op::Add => arg1 + arg2,
            Op::Sub => arg1 - arg2,
            Op::Mul => arg1 * arg2,
            Op::Div if arg2 == 0 => return None,
            Op::Div => arg1 / arg2,
        };
        Some(res)
    }
}

impl Sexp {
    fn eval(&self) -> Option<i64> {
        match self.kind() {
            SexpKind::Atom(atom) => atom.eval(),
            SexpKind::List(list) => list.eval(),
        }
    }
}

impl Parse {
    fn root(&self) -> Root {
        Root::cast(self.syntax()).unwrap()
    }
}

fn lex(text: &str) -> Vec<(SyntaxKind, String)> {
    fn tok(t: SyntaxKind) -> m_lexer::TokenKind {
        m_lexer::TokenKind(rowan::SyntaxKind::from(t).0)
    }

    fn kind(t: m_lexer::TokenKind) -> SyntaxKind {
        match t.0 {
            0 => L_PAREN,
            1 => R_PAREN,
            2 => WORD,
            3 => WHITESPACE,
            4 => ERROR,
            _ => unreachable!(),
        }
    }

    let lexer = m_lexer::LexerBuilder::new()
        .error_token(tok(ERROR))
        .tokens(&[
            (tok(L_PAREN), r"\("),
            (tok(R_PAREN), r"\)"),
            (tok(WORD), r"[^\s()]+"),
            (tok(WHITESPACE), r"\s+"),
        ])
        .build();
    
    lexer.tokenize(text)
        .into_iter()
        .map(|t| (t.len, kind(t.kind)))
        .scan(0usize, |start_offset, (len, kind)| {
            let s: String = text[*start_offset..*start_offset + len].into();
            *start_offset += len;
            Some((kind, s))
        }) 
        .collect()
}