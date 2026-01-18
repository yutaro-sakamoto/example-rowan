#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(u16)]
enum SyntaxKind {
    L_Paren = 0,
    R_Paren,
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

//fn parse(text: &str) -> Parse {
//    struct Parser {
//        tokens: Vec<(SyntaxKind, String)>,
//        builder: GreenNodeBuilder<'static>,
//        errors: Vec<String>,
//    }
//
//    enum SexpRes {
//        Ok,
//        Eof,
//        RParen,
//    }
//
//    impl Parser {
//        fn parse(mut self) -> Parse {
//            self.builder.start_node(Root.into());
//            loop {
//                match self.sexp() {
//                    SexpRes::Eof => break,
//                    SexpRes::RParen => {
//                        self.builder.start_node(ERROR.into());
//                        self.errors.push("unmatched `)".to_string());
//                        self.bump();
//                        self.builder.finish_node();
//                    }
//                    SexpRes::Ok => (),
//                }
//            }
//
//            self.skip_ws();
//            self.builder.finish_node();
//
//            Parse { green_node: self.builder.finish(), errors: self.errors }
//        }
//    }
//}