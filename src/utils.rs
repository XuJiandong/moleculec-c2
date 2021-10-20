use crate::generator::Generator;
use molecule_codegen::ast;
use std::fmt::Result;

pub const VERSION: u32 = 7002;

pub fn gen_c(output: &mut Output, ast: &ast::Ast) -> Result {
    for decl in ast.major_decls() {
        match decl.as_ref() {
            ast::TopDecl::Option_(ref i) => i.gen_c(output)?,
            ast::TopDecl::Union(ref i) => i.gen_c(output)?,
            ast::TopDecl::Array(ref i) => i.gen_c(output)?,
            ast::TopDecl::Struct(ref i) => i.gen_c(output)?,
            ast::TopDecl::FixVec(ref i) => i.gen_c(output)?,
            ast::TopDecl::DynVec(ref i) => i.gen_c(output)?,
            ast::TopDecl::Table(ref i) => i.gen_c(output)?,
            ast::TopDecl::Primitive(_) => unreachable!(),
        };
    }
    Ok(())
}

pub fn gen_rust(output: &mut Output, ast: &ast::Ast) -> Result {
    for decl in ast.major_decls() {
        match decl.as_ref() {
            ast::TopDecl::Option_(ref i) => i.gen_rust(output)?,
            ast::TopDecl::Union(ref i) => i.gen_rust(output)?,
            ast::TopDecl::Array(ref i) => i.gen_rust(output)?,
            ast::TopDecl::Struct(ref i) => i.gen_rust(output)?,
            ast::TopDecl::FixVec(ref i) => i.gen_rust(output)?,
            ast::TopDecl::DynVec(ref i) => i.gen_rust(output)?,
            ast::TopDecl::Table(ref i) => i.gen_rust(output)?,
            ast::TopDecl::Primitive(_) => unreachable!(),
        };
    }
    Ok(())
}

pub struct Output {
    decl: String,
    def: String,
    imp: String,
}

impl Output {
    pub fn new() -> Output {
        Output {
            decl: String::new(),
            def: String::new(),
            imp: String::new(),
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
    pub fn write_imp(&mut self, s: &str) {
        self.imp += s;
        self.def += "\n";
    }

    pub fn combine_c(&self, name: &str) -> String {
        let mut res = String::new();
        let name = name.to_uppercase();
        res.push_str(&format!(
            r###"
        #ifndef _{0}_API2_H_
        #define _{0}_API2_H_
        "###,
            name
        ));
        let prefix = format!(
            r###"
#define MOLECULEC2_VERSION {0}
#define MOLECULE2_API_VERSION_MIN 5000

#include "molecule2_reader.h"

#ifdef __cplusplus
extern "C" {{
#endif /* __cplusplus */
        "###,
            VERSION
        );

        res.push_str(&prefix);

        res.push_str("\n// ----forward declaration--------\n");
        res.push_str(&self.decl);
        res.push_str("\n// ----definition-----------------\n");
        res.push_str(&self.def);

        res.push_str("\n#ifndef MOLECULEC_C2_DECLARATION_ONLY\n");
        res.push_str("\n// ----implementation-------------\n");
        res.push_str(&self.imp);
        res.push_str("\n#endif // MOLECULEC_C2_DECLARATION_ONLY\n");

        let suffix = format!(
            r###"
#ifdef __cplusplus
}}
#endif /* __cplusplus */
        "###
        );
        res.push_str(&suffix);
        res.push_str(&format!(
            r###"
        #endif // _{0}_API2_H_
        "###,
            name
        ));
        res
    }
    pub fn combine_rust(&self, _: &str) -> String {
        let mut res = String::new();
        res.push_str(
            r#"
            #![allow(dead_code)]
            #![allow(unused_imports)]
            extern crate alloc;
            use alloc::vec::Vec;
            use molecule2::Cursor;

        "#,
        );
        res.push_str(self.imp.as_ref());
        res
    }
}
