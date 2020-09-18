use crate::generator::Generator;
use molecule_codegen::{ast, VERSION};
use std::io;

pub fn generate(writer: &mut dyn Writer, ast: &ast::Ast) -> io::Result<()> {
    for decl in ast.major_decls() {
        match decl.as_ref() {
            ast::TopDecl::Option_(ref i) => i.generate(writer)?,
            ast::TopDecl::Union(ref i) => i.generate(writer)?,
            ast::TopDecl::Array(ref i) => i.generate(writer)?,
            ast::TopDecl::Struct(ref i) => i.generate(writer)?,
            ast::TopDecl::FixVec(ref i) => i.generate(writer)?,
            ast::TopDecl::DynVec(ref i) => i.generate(writer)?,
            ast::TopDecl::Table(ref i) => i.generate(writer)?,
            ast::TopDecl::Primitive(_) => unreachable!(),
        };
    }
    Ok(())
}

pub trait Writer {
    fn write_decl(&mut self, s: &str);
    fn write_def(&mut self, s: &str);
}

pub struct Output {
    decl: String,
    def: String,
}

impl Writer for Output {
    fn write_decl(&mut self, s: &str) {
        self.decl += s;
    }
    fn write_def(&mut self, s: &str) {
        self.def += s;
    }
}

impl Output {
    pub fn new() -> Output {
        Output {
            decl: String::new(),
            def: String::new(),
        }
    }

    pub fn combine(&self) -> String {
        let mut res = String::new();
        res.push_str(&self.decl);
        res.push_str(&self.def);
        return res;
    }
}
