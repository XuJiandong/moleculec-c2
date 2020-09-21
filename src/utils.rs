use crate::generator::Generator;
use molecule_codegen::{ast, VERSION};
use std::io;
use std::fmt::{Write, Result};

pub fn generate(output: &mut Output, ast: &ast::Ast) -> Result {
    for decl in ast.major_decls() {
        match decl.as_ref() {
            ast::TopDecl::Option_(ref i) => i.generate(output)?,
            ast::TopDecl::Union(ref i) => i.generate(output)?,
            ast::TopDecl::Array(ref i) => i.generate(output)?,
            ast::TopDecl::Struct(ref i) => i.generate(output)?,
            ast::TopDecl::FixVec(ref i) => i.generate(output)?,
            ast::TopDecl::DynVec(ref i) => i.generate(output)?,
            ast::TopDecl::Table(ref i) => i.generate(output)?,
            ast::TopDecl::Primitive(_) => unreachable!(),
        };
    }
    Ok(())
}

pub struct Output {
    decl: String,
    def: String,
}

impl Output {
    pub fn new() -> Output {
        Output {
            decl: String::new(),
            def: String::new(),
        }
    }

    pub fn write_decl(&mut self, s: &str) {
        self.decl += s;
        self.decl += "\n";
    }
    pub fn write_def(&mut self, s: &str) {
        self.def += s;
        self.def += "\n";
    }
    pub fn combine(&self) -> String {
        let mut res = String::new();
        res.push_str(&self.decl);
        res.push_str("\n// -------------------\n");
        res.push_str(&self.def);
        return res;
    }
}

// now we can use write!(&output, "{}", ...) now
impl Write for Output {
    fn write_str(&mut self, s: &str) -> Result {
        self.write_def(s);
        Ok(())
    }
}
