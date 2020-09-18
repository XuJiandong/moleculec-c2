use crate::utils::Writer;
use core::mem::size_of;
use molecule_codegen::ast::{self, DefaultContent, HasName};
use std::io;

pub type Number = u32;
pub const NUMBER_SIZE: usize = size_of::<Number>();

pub trait Generator: HasName + DefaultContent {
    fn generate(&self, writer: &mut dyn Writer) -> io::Result<()>;
    fn common_generate(&self, writer: &mut dyn Writer) -> io::Result<()> {
        Ok(())
    }
}

impl Generator for ast::Option_ {
    fn generate(&self, writer: &mut dyn Writer) -> io::Result<()> {
        Ok(())
    }
}

impl Generator for ast::Union {
    fn generate(&self, writer: &mut dyn Writer) -> io::Result<()> {
        Ok(())
    }
}

impl Generator for ast::Array {
    fn generate(&self, writer: &mut dyn Writer) -> io::Result<()> {
        Ok(())
    }
}

impl Generator for ast::Struct {
    fn generate(&self, writer: &mut dyn Writer) -> io::Result<()> {
        Ok(())
    }
}

impl Generator for ast::FixVec {
    fn generate(&self, writer: &mut dyn Writer) -> io::Result<()> {
        Ok(())
    }
}

impl Generator for ast::DynVec {
    fn generate(&self, writer: &mut dyn Writer) -> io::Result<()> {
        Ok(())
    }
}

impl Generator for ast::Table {
    fn generate(&self, writer: &mut dyn Writer) -> io::Result<()> {
        Ok(())
    }
}
