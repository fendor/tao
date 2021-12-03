use super::*;
use std::io::Write;

#[derive(Debug)]
pub enum Error {
    Mismatch(TyId, TyId, Option<Span>),
    CannotInfer(TyId, Option<Span>),
    Recursive(TyId, Span, Span),
    NoSuchField(TyId, SrcNode<Ident>),
    NoSuchLocal(SrcNode<Ident>),
    WrongNumberOfParams(Span, usize, Span, usize),
    NoBranches(Span),
    InvalidUnaryOp(SrcNode<ast::UnaryOp>, TyId, Span),
    InvalidBinaryOp(SrcNode<ast::BinaryOp>, TyId, Span, TyId, Span),
    NoSuchData(SrcNode<Ident>),
    NoSuchCons(SrcNode<Ident>),
    RecursiveAlias(AliasId, TyId, Span),
    DuplicateTypeName(Ident, Span, Span),
    DuplicateDefName(Ident, Span, Span),
    DuplicateConsName(Ident, Span, Span),
    DuplicateGenName(Ident, Span, Span),
    PatternNotSupported(TyId, SrcNode<ast::BinaryOp>, TyId, Span),
    NotExhaustive(Span, ExamplePat),
    WrongNumberOfGenerics(Span, usize, Span, usize),
    DefTypeNotSpecified(Span, Span, Ident),
}

impl Error {
    pub fn write<C: ariadne::Cache<SrcId>>(self, ctx: &Context, cache: C, writer: impl Write) {
        use ariadne::{Report, ReportKind, Label, Color, Fmt, Span};

        let display = |id| ctx.tys.display(&ctx.datas, id);

        let (msg, spans, notes) = match self {
            Error::Mismatch(a, b, loc) => (
                format!("Type mismatch between {} and {}", display(a).fg(Color::Red), display(b).fg(Color::Red)),
                {
                    let mut labels = vec![
                        (ctx.tys.get_span(a), format!("{}", display(a)), Color::Red),
                        (ctx.tys.get_span(b), format!("{}", display(b)), Color::Red),
                    ];
                    if let Some(loc) = loc {
                        labels.push((loc, format!("Types must be equal here"), Color::Yellow));
                    }
                    labels
                },
                vec![],
            ),
            Error::CannotInfer(a, origin) => (
                format!("Cannot infer type {}", display(a).fg(Color::Red)),
                match origin {
                    Some(origin) => vec![
                        (ctx.tys.get_span(a), format!("Use of generic definition"), Color::Red),
                        (origin, format!("Instantiation of this generic type"), Color::Red)
                    ],
                    None => vec![(ctx.tys.get_span(a), format!("{}", display(a)), Color::Red)],
                },
                vec![],
            ),
            Error::Recursive(a, span, part) => (
                format!("Recursive type {}", display(a).fg(Color::Red)),
                vec![
                    (span, format!("Mentions itself"), Color::Red),
                    (part, format!("Recursive element is here"), Color::Red),
                ],
                vec![],
            ),
            Error::NoSuchField(a, field) => (
                format!("No such field {} on {}", (*field).fg(Color::Red), display(a).fg(Color::Red)),
                vec![
                    (ctx.tys.get_span(a), format!("{}", display(a)), Color::Red),
                    (field.span(), format!("Field"), Color::Red),
                ],
                vec![],
            ),
            Error::NoSuchLocal(local) => (
                format!("No such local {}", (*local).fg(Color::Red)),
                vec![(local.span(), format!("Scope does not contain this"), Color::Red)],
                vec![],
            ),
            Error::WrongNumberOfParams(a, a_count, b, b_count) => (
                format!("Pattern arms must all have the same number of parameters"),
                vec![
                    (a, format!("Has {} parameter(s)", a_count), Color::Red),
                    (b, format!("Has {} parameter(s)", b_count), Color::Red),
                ],
                vec![],
            ),
            Error::NoBranches(span) => (
                format!("Pattern match must have at least one branch"),
                vec![(span, format!("Must have a branch"), Color::Red)],
                vec![],
            ),
            Error::InvalidUnaryOp(op, a, a_span) => (
                format!("Cannot apply {} to {}", (*op).fg(Color::Red), display(a).fg(Color::Red)),
                vec![(op.span().union(a_span), format!("Operation {} applied here", (*op).fg(Color::Red)), Color::Red)],
                match ctx.tys.get(a) {
                    Ty::Gen(_, _) => vec![format!(
                        "Consider adding a typeclass constraint like {}",
                        format!("{} < {:?}", display(a), *op).fg(Color::Blue),
                    )],
                    _ => vec![],
                },
            ),
            Error::InvalidBinaryOp(op, a, a_span, b, b_span) => (
                format!("Invalid operation {} {} {}", display(a).fg(Color::Red), (*op).fg(Color::Red), display(b).fg(Color::Red)),
                vec![(a_span.union(op.span()).union(b_span), format!("Operation {} applied here", (*op).fg(Color::Red)), Color::Red)],
                match ctx.tys.get(a) {
                    Ty::Gen(_, _) => vec![format!(
                        "Consider adding a typeclass constraint like {}",
                        format!("{} < {:?} {}", display(a), *op, display(b)).fg(Color::Blue),
                    )],
                    _ => vec![],
                },
            ),
            Error::NoSuchData(a) => (
                format!("No such type {}", (*a).fg(Color::Red)),
                vec![(a.span(), format!("Does not exist"), Color::Red)],
                vec![],
            ),
            Error::NoSuchCons(a) => (
                format!("No such constructor {}", (*a).fg(Color::Red)),
                vec![(a.span(), format!("Does not exist"), Color::Red)],
                vec![],
            ),
            Error::RecursiveAlias(alias, ty, span) => (
                format!("Recursive type alias"),
                vec![
                    (ctx.datas.get_alias_span(alias), format!("Alias mentions itself, leading to an infinite expansion"), Color::Red),
                    (span, format!("Recursion occurs here"), Color::Yellow),
                ],
                {
                    let alias = ctx.datas.get_alias(alias).unwrap();
                    vec![format!(
                        "Type aliases expand eagerly. Consider using a data type like {} instead.",
                        format!("data {} = {}", alias.name, display(alias.ty).substitute(ty, |f| write!(f, "{}", alias.name))).fg(Color::Blue),
                    )]
                },
            ),
            Error::DuplicateTypeName(name, old, new) => (
                format!("Type {} declared multiple times", name.fg(Color::Red)),
                vec![
                    (old, format!("Previous declaration"), Color::Yellow),
                    (new, format!("Conflicting declaration"), Color::Red),
                ],
                vec![],
            ),
            Error::DuplicateDefName(name, old, new) => (
                format!("Definition {} declared multiple times", name.fg(Color::Red)),
                vec![
                    (old, format!("Previous declaration"), Color::Yellow),
                    (new, format!("Conflicting declaration"), Color::Red),
                ],
                vec![],
            ),
            Error::DuplicateConsName(name, old, new) => (
                format!("Constructor {} declared multiple times", name.fg(Color::Red)),
                vec![
                    (old, format!("Previous declaration"), Color::Yellow),
                    (new, format!("Conflicting declaration"), Color::Red),
                ],
                vec![],
            ),
            Error::DuplicateGenName(name, old, new) => (
                format!("Type parameter {} declared multiple times", name.fg(Color::Red)),
                vec![
                    (old, format!("Previous type parameter"), Color::Yellow),
                    (new, format!("Conflicting type parameter"), Color::Red),
                ],
                vec![],
            ),
            Error::PatternNotSupported(lhs, op, rhs, span) => (
                format!("Arithmetic pattern {} {} {} is not supported", display(lhs).fg(Color::Red), (*op).fg(Color::Red), display(rhs).fg(Color::Red)),
                vec![(span, format!("Pattern {} used here", (*op).fg(Color::Red)), Color::Red)],
                vec![format!(
                    "Only specific arithmetic patterns, such as {}, are supported",
                    format!("Nat + Nat").fg(Color::Blue),
                )],
            ),
            Error::NotExhaustive(span, example) => (
                format!("Pattern match is not exhaustive"),
                vec![(span, format!("Pattern {} not covered", (&example).fg(Color::Red)), Color::Red)],
                vec![format!(
                    "Add another arm like {} to ensure that this case is covered.",
                    format!("| {} => ...", example).fg(Color::Blue),
                )],
            ),
            Error::WrongNumberOfGenerics(a, a_count, b, b_count) => (
                format!("Wrong number of type parameters"),
                vec![
                    (a, format!("Provided with {} parameters", a_count), Color::Red),
                    (b, format!("Has {} parameter(s)", b_count), Color::Yellow),
                ],
                vec![],
            ),
            Error::DefTypeNotSpecified(def, usage, name) => (
                format!("Type of {} must be fully specified", name.fg(Color::Red)),
                vec![
                    (def, format!("Definition does not have a fully specified type hint"), Color::Red),
                    (usage, format!("Type must be fully known here"), Color::Yellow),
                ],
                vec![format!(
                    "Add a type hint to the def like {}",
                    format!("def {} : ...", name).fg(Color::Blue),
                )],
            ),
        };

        let mut report = Report::build(ReportKind::Error, spans.first().unwrap().0.src(), spans.first().unwrap().0.start())
            .with_code(3)
            .with_message(msg);

        for (span, msg, col) in spans {
            report = report.with_label(Label::new(span)
                .with_message(msg)
                .with_color(col));
        }

        for note in notes {
            report = report.with_note(note);
        }

        report
            .finish()
            .write(cache, writer)
            .unwrap();
    }
}
