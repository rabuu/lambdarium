use std::collections::HashMap;

use crate::{core, stlc};

/// Pretty print terms and types
///
/// You must either implement:
/// - `print` (good for congruence cases)
/// - `print_verbose` and `print_pretty`
pub trait Print {
    fn print(&self, style: PrintStyle) -> String {
        match style {
            PrintStyle::Verbose => self.print_verbose(),
            PrintStyle::Pretty => self.print_pretty(),
        }
    }

    fn print_verbose(&self) -> String {
        self.print(PrintStyle::Verbose)
    }

    fn print_pretty(&self) -> String {
        self.print(PrintStyle::Pretty)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrintStyle {
    Verbose,
    Pretty,
}

impl Print for core::Term {
    fn print(&self, style: PrintStyle) -> String {
        match self {
            core::Term::Var(x) => x.print(style),
            core::Term::App(x) => x.print(style),
            core::Term::Abs(x) => x.print(style),
        }
    }
}

impl Print for core::Variable {
    fn print(&self, _: PrintStyle) -> String {
        if self.inner.len() == 1 {
            self.inner.clone()
        } else {
            format!(r#""{}""#, self.inner)
        }
    }
}

impl Print for core::Application {
    fn print_verbose(&self) -> String {
        let lhs = self.lhs.print_verbose();
        let rhs = self.rhs.print_verbose();
        format!("({lhs} {rhs})")
    }

    fn print_pretty(&self) -> String {
        // TODO: pretty impl
        self.print_verbose()
    }
}

impl Print for core::Abstraction {
    fn print_verbose(&self) -> String {
        let binder = self.binder.print_verbose();
        let body = self.body.print_verbose();
        format!("(λ{binder}.{body})")
    }

    fn print_pretty(&self) -> String {
        // TODO: pretty impl
        self.print_verbose()
    }
}

impl Print for stlc::Type {
    fn print_verbose(&self) -> String {
        TypePrinter::default().print_verbose(self)
    }

    fn print_pretty(&self) -> String {
        // TODO: pretty impl
        self.print_verbose()
    }
}

type TypeVarMap = HashMap<stlc::TypeVariable, char>;

#[derive(Debug, Default)]
struct TypePrinter {
    next: usize,
    map: TypeVarMap,
}

impl TypePrinter {
    fn print_verbose(&mut self, typ: &stlc::Type) -> String {
        match typ {
            stlc::Type::Var(type_variable) => match self.map.get(type_variable) {
                Some(name) => name.to_string(),
                None => match self.next_name() {
                    Some(next) => {
                        self.map.insert(*type_variable, next);
                        next.to_string()
                    }
                    None => format!("?{}", type_variable.id),
                },
            },
            stlc::Type::Fun { lhs, rhs } => {
                let lhs = self.print_verbose(lhs);
                let rhs = self.print_verbose(rhs);
                format!("({lhs} -> {rhs})",)
            }
        }
    }

    fn next_name(&mut self) -> Option<char> {
        const NUM_OF_GREEK_CHARS: usize = 23;
        const GREEK: [char; NUM_OF_GREEK_CHARS] = [
            'α', 'β', 'γ', 'δ', 'ε', 'ζ', 'η', 'θ', 'ι', 'κ', 'μ', 'ν', 'ξ', 'ο', 'π', 'ρ', 'σ',
            'τ', 'υ', 'φ', 'χ', 'ψ', 'ω',
        ];

        let name = GREEK.get(self.next).copied();
        self.next += 1;
        name
    }
}
