use std::collections::HashMap;

use crate::{core, meta, stlc};

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

impl core::Term {
    fn print_pretty_prec(&self, prec: usize) -> String {
        match self {
            core::Term::Var(variable) => variable.print_pretty(),
            core::Term::App(application) => application.print_pretty_prec(prec),
            core::Term::Abs(abstraction) => abstraction.print_pretty_prec(prec),
        }
    }

    fn strip_leading_binders(self) -> (core::Term, Vec<core::Variable>) {
        let mut binders = Vec::new();
        let mut t = self;
        while let core::Term::Abs(abs) = t {
            binders.push(abs.binder);
            t = *abs.body;
        }
        (t, binders)
    }
}

impl Print for core::Variable {
    fn print(&self, _: PrintStyle) -> String {
        if self.inner.len() == 1 {
            self.inner.clone()
        } else {
            format!(r#"<{}>"#, self.inner)
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
        self.print_pretty_prec(0)
    }
}

impl core::Application {
    fn print_pretty_prec(&self, prec: usize) -> String {
        let mut s = format!(
            "{}{}",
            self.lhs.print_pretty_prec(1),
            self.rhs.print_pretty_prec(2)
        );

        if prec > 1 {
            s = format!("({s})");
        }

        s
    }
}

impl Print for core::Abstraction {
    fn print_verbose(&self) -> String {
        let binder = self.binder.print_verbose();
        let body = self.body.print_verbose();
        format!("(λ{binder}.{body})")
    }

    fn print_pretty(&self) -> String {
        self.print_pretty_prec(0)
    }
}

impl core::Abstraction {
    fn print_pretty_prec(&self, prec: usize) -> String {
        let (body, binders) = core::Term::Abs(self.clone()).strip_leading_binders();
        let binders: String = binders.iter().map(Print::print_pretty).collect();

        let mut s = format!("λ{binders}.{}", body.print_pretty_prec(0));

        if prec > 0 {
            s = format!("({s})");
        }

        s
    }
}

impl Print for meta::MetaVariable {
    fn print(&self, _style: PrintStyle) -> String {
        if self.inner.len() == 1 {
            self.inner.clone()
        } else {
            format!(r#"<{}>"#, self.inner)
        }
    }
}

impl Print for stlc::Type {
    fn print_verbose(&self) -> String {
        TypePrinter::default().print_verbose(self)
    }

    fn print_pretty(&self) -> String {
        TypePrinter::default().print_pretty(self, 0)
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
            stlc::Type::Var(type_variable) => self.print_type_variable(type_variable),
            stlc::Type::Fun { lhs, rhs } => {
                let lhs = self.print_verbose(lhs);
                let rhs = self.print_verbose(rhs);
                format!("({lhs} -> {rhs})",)
            }
        }
    }

    fn print_pretty(&mut self, typ: &stlc::Type, prec: usize) -> String {
        match typ {
            stlc::Type::Var(type_variable) => self.print_type_variable(type_variable),
            stlc::Type::Fun { lhs, rhs } => {
                let mut s = format!(
                    "{} -> {}",
                    self.print_pretty(lhs, 1),
                    self.print_pretty(rhs, 0)
                );

                if prec > 0 {
                    s = format!("({s})");
                }

                s
            }
        }
    }

    fn print_type_variable(&mut self, var: &stlc::TypeVariable) -> String {
        match self.map.get(var) {
            Some(name) => name.to_string(),
            None => match self.next_name() {
                Some(next) => {
                    self.map.insert(*var, next);
                    next.to_string()
                }
                None => format!("?{}", var.id),
            },
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
