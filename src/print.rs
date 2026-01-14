use crate::{core, meta, stlc};

pub trait Print {
    fn print(&self, style: PrintStyle) -> String {
        match style {
            PrintStyle::Verbose => self.print_verbose(),
            PrintStyle::Pretty => self.print_pretty(),
        }
    }

    fn print_verbose(&self) -> String;

    fn print_pretty(&self) -> String;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrintStyle {
    Verbose,
    Pretty,
}

impl Print for core::Term {
    fn print_verbose(&self) -> String {
        todo!()
    }

    fn print_pretty(&self) -> String {
        todo!()
    }
}

impl Print for stlc::Type {
    fn print_verbose(&self) -> String {
        todo!()
    }

    fn print_pretty(&self) -> String {
        todo!()
    }
}
