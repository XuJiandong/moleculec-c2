use crate::utils::{Output, generate};
use core::mem::size_of;
use molecule_codegen::ast::{self, DefaultContent, HasName, *};
use std::fmt::{Write, Result};

pub type Number = u32;
pub const NUMBER_SIZE: usize = size_of::<Number>();

pub trait Generator: HasName + DefaultContent {
    fn generate(&self, output: &mut Output) -> Result;
}

impl Generator for ast::Option_ {
    fn generate(&self, output: &mut Output) -> Result {
        Ok(())
    }
}

impl Generator for ast::Union {
    fn generate(&self, output: &mut Output) -> Result {
        Ok(())
    }
}

impl Generator for ast::Array {
    fn generate(&self, output: &mut Output) -> Result {
        // it's not needed
        Ok(())
    }
}

impl Generator for ast::Struct {
    fn generate(&self, output: &mut Output) -> Result {
        generate_common(output, self.name(), self.fields(), Some(self.field_sizes()))
    }
}

impl Generator for ast::FixVec {
    fn generate(&self, output: &mut Output) -> Result {
        // it's not needed
        Ok(())
    }
}

// the possible types as DynVec's item
// 1. FixVec
// 2. Table
// 3. DynVec
// 4. Option/Union
fn get_dynvec_item_type(dynvec: &ast::DynVec) -> String {
    let item_type = dynvec.item().typ();
    if let TopDecl::FixVec(a) = item_type.as_ref() {
        String::from("mol2_cursor_t")
    } else {
        String::from(item_type.name())
    }
}

impl Generator for ast::DynVec {
    fn generate(&self, output: &mut Output) -> Result {
        let item_type = get_dynvec_item_type(self);
        let name = self.name();
        output.write_decl(&format!("struct {};", gen_class_name(name)));
        output.write_decl(&format!("struct {}VTable;", name));
        output.write_decl(&format!("struct {}VTable *Get{}VTable(void);", name, name));

        output.write_decl(&format!("uint32_t {0}_len_impl(struct {0}Type *);", name));
        output.write_decl(&format!("struct {0} {1}_get_impl(struct {1}Type *, uint32_t, int *);", item_type, name));
        // --------- declaration above ------------

        // ----------definition below -------------
        // definition of virtual table
        output.write_def(&format!("typedef struct {0}VTable {{", name));

        output.write_def(&format!("uint32_t (*len)(struct {}Type *);", name));
        output.write_def(&format!("{1} (*get)(struct {0}Type *, uint32_t, int *);", name, item_type));

        output.write_def(&format!("}} {}VTable;", name));
        // definition of class
        output.write_def(&format!(r#"typedef struct {0}Type {{
    mol2_cursor_t cur;
    {0}VTable *tbl;
    }} {0}Type;
    "#, name));
        // definition of "make" class instance
        output.write_def(&format!(r#"struct {0}Type make_{0}(mol2_cursor_t *cur) {{
    {0}Type ret;
    ret.cur = *cur;
    ret.tbl = Get{0}VTable();
    return ret;
              }}
              "#, name));
        // definition of "get" class vtable
        output.write_def(&format!(r#"struct {0}VTable *Get{0}VTable(void) {{
    static {0}VTable s_vtable;
    static int inited = 0;
    if (inited) return &s_vtable;
            "#, name));

        output.write_def(&format!("s_vtable.len = {}_len_impl;", name));
        output.write_def(&format!("s_vtable.get = {}_get_impl;", name));

        output.write_def(&format!("return &s_vtable; }}"));

        // entries of virtual tables
        output.write_imp(&format!(r#"
        uint32_t {0}_len_impl({0}Type *this) {{
          return mol2_dynvec_length(&this->cur);
        }}"#, name));

        if item_type == "mol2_cursor_t" {
            output.write_imp(&format!(r#"
    mol2_cursor_t {0}_get_impl({0}Type *this,
                            uint32_t index, int *existing) {{
    mol2_cursor_t ret;
    mol2_cursor_res_t res = mol2_dynvec_slice_by_index(&this->cur, index);
    if (res.errno != 0) {{
        *existing = 0;
        return ret;
    }} else {{
        *existing = 1;
    }}
    ret = convert_to_rawbytes(&res.cur);
    return ret;
    }}"#, name));
        } else {
            output.write_imp(&format!(r#"
    {1}Type {0}_get_impl({0}Type *this,
                            uint32_t index, int *existing) {{
    {0}Type ret;
    mol2_cursor_res_t res = mol2_dynvec_slice_by_index(&this->cur, index);
    if (res.errno != 0) {{
        *existing = 0;
        return ret;
    }} else {{
        *existing = 1;
    }}
        ret.cur = res.cur;
        ret.tbl = Get{0}VTable();
        return ret;
    }}"#, name, item_type));
        }
        Ok(())
    }
}

fn need_struct_prefix(field_type: &str) -> String {
    match field_type {
        "mol2_cursor_t"|"int8_t"|"uint8_t"|"int16_t"|"uint16_t"|
        "int32_t"|"uint32_t"|"int64_t"|"uint64_t" => {
            String::from(field_type)
        },
        _ => {
            format!("struct {}", field_type)
        }
    }
}

// Table/Struct
fn generate_common(output: &mut Output, name: &str,
                   fields: &[FieldDecl], field_sizes: Option<&[usize]>) -> Result {
    output.write_decl(&format!("struct {};", gen_class_name(name)));
    output.write_decl(&format!("struct {}VTable;", name));
    output.write_decl(&format!("struct {}VTable *Get{}VTable(void);", name, name));

    for field in fields {
        let field_name = &field.name();
        let (field_type, _) = gen_field_type(field);
        output.write_decl(&format!("{0} {1}_get_{2}_impl(struct {1}Type *);",
                                   need_struct_prefix(&field_type), name, field_name));
    }
    // --------- declaration above ------------

    // ----------definition below -------------
    // definition of virtual table
    output.write_def(&format!("typedef struct {0}VTable {{", name));
    for field in fields {
        let field_name = &field.name();
        let (field_type, _) = gen_field_type(field);
        output.write_def(&format!("{0} (*{1})(struct {2}Type *);", field_type, field_name, name));
    }
    output.write_def(&format!("}} {}VTable;", name));
    // definition of class
    output.write_def(&format!(r#"typedef struct {0}Type {{
    mol2_cursor_t cur;
    {0}VTable *tbl;
    }} {0}Type;
    "#, name));
    // definition of "make" class instance
    output.write_def(&format!(r#"struct {0}Type make_{0}(mol2_cursor_t *cur) {{
    {0}Type ret;
    ret.cur = *cur;
    ret.tbl = Get{0}VTable();
    return ret;
              }}
              "#, name));
    // definition of "get" class vtable
    output.write_def(&format!(r#"struct {0}VTable *Get{0}VTable(void) {{
    static {0}VTable s_vtable;
    static int inited = 0;
    if (inited) return &s_vtable;
            "#, name));
    for field in fields {
        let field_name = &field.name();
        let (field_type, _) = gen_field_type(field);
        output.write_def(&format!("s_vtable.{0} = {1}_get_{0}_impl;", field_name, name));
    }
    output.write_def(&format!("return &s_vtable; }}"));
    // entries of virtual tables
    let mut index : usize = 0;
    for field in fields {
        generate_impl(output, name, field, index, field_sizes);
        index += 1;
    }
    Ok(())
}

fn generate_impl(output: &mut Output, name: &str, field: &FieldDecl,
                 index: usize, field_sizes: Option<&[usize]>) {
    let field_name = field.name();
    let (field_type, ftc) = gen_field_type(field);
    let slice_by = generate_slice_by(index, &field_sizes);
    match ftc {
        FieldTypeCategory::Type => {
            output.write_imp(&format!(r#"
    {2} {0}_get_{1}_impl({0}Type *this) {{
    {2} ret;
    mol2_cursor_t cur = {4};
    ret.cur = cur;
    ret.tbl = Get{3}VTable();
    return ret;
    }}"#, name, field_name, field_type, raw_name(&field_type), slice_by));
        },
        FieldTypeCategory::Primitive|FieldTypeCategory::Array => {
            output.write_imp(&format!(r#"
    {2} {0}_get_{1}_impl({0}Type *this) {{
    {2} ret;
    mol2_cursor_t ret2 = {3};
    ret = {4}(&ret2);
    return ret;
    }}"#, name, field_name, field_type, slice_by, get_convert_func(&field_type, &ftc)));
        },
        FieldTypeCategory::FixVec => {
            output.write_imp(&format!(r#"
    {2} {0}_get_{1}_impl({0}Type *this) {{
    mol2_cursor_t ret;
    mol2_cursor_t re2 = {3};
    ret = convert_to_rawbytes(&re2);
    return ret;
    }}"#, name, field_name, field_type, slice_by));
        },
    }
}

impl Generator for ast::Table {
    fn generate(&self, output: &mut Output) -> Result {
        generate_common(output, self.name(), self.fields(), None)
    }
}

fn gen_class_name(name: &str) -> String {
    format!("{}Type", name)
}

// 1. category 1, primitive
// uint8, int8
// uint16, int16
// uint32, int32
// uint64, int64

// 2. category 2, array/fixvec
// <byte>
// array

// 3. category 3, normal type

enum FieldTypeCategory {
    Primitive,
    Array,
    FixVec,
    Type
}

fn gen_field_type(field: &ast::FieldDecl) -> (String, FieldTypeCategory) {
    let field_type_name = field.typ().name();
    let mut ftc = FieldTypeCategory::Primitive;

    let mut name = format!("{}", gen_class_name(&String::from(field_type_name)));
    match field.typ().as_ref() {
        // if the field type is array and the field type name is "uint8", "int8" ...
        // then it's primitive
        TopDecl::Array(a) => {
            let field_type_name = field.typ().name().to_lowercase();
            let new_name = match field_type_name.as_ref() {
                "uint8" => "uint8_t",
                "int8" => "int8_t",
                "uint16" => "uint16_t",
                "int16" => "int16_t",
                "uint32" => "uint32_t",
                "int32" => "int32_t",
                "uint64" => "uint64_t",
                "int64" => "int64_t",
                _ => {
                    ftc = FieldTypeCategory::Array;
                    "mol2_cursor_t"
                },
            };
            name = String::from(new_name)
        },
        TopDecl::FixVec(v) => {
            // FixVec is different than Array: it has a header.
            name = String::from("mol2_cursor_t");
            ftc = FieldTypeCategory::FixVec;
        },
        _ => {
            ftc = FieldTypeCategory::Type;
        },
    }
    (name, ftc)
}

fn raw_name(type_name: &str) -> String {
    String::from(type_name).replace("Type", "")
}

fn get_convert_func<'a>(type_name: &'a str, ftc: &FieldTypeCategory) -> &'a str {
    let t = type_name.to_lowercase();
    match t.as_ref() {
        "int8_t" => "convert_to_Int8",
        "uint8_t" => "convert_to_Uint8",
        "int16_t" => "convert_to_Int16",
        "uint16_t" => "convert_to_Uint16",
        "int32_t" => "convert_to_Int32",
        "uint32_t" => "convert_to_Uint32",
        "int64_t" => "convert_to_Int64",
        "uint64_t" => "convert_to_Uint64",
        _ => {
            if let FieldTypeCategory::Array = ftc {
                "convert_to_array"
            } else {
                assert!(false);
                "N/A"
            }
        }
    }
}

// generate slice function for struct or table.
// struct: mol2_slice_by_offset(&this->cur, 4, 2)
// table:  mol2_table_slice_by_index(&this->cur, 1);
fn generate_slice_by(index: usize, fields_sizes: &Option<&[usize]>) -> String {
    if let Some(ref fs) = fields_sizes {
        let size = fs[index];
        let mut offset = 0;
        for i in (0..index).rev() {
            offset += fs[i];
        }
        format!("mol2_slice_by_offset(&this->cur, {}, {})", offset, size)
    } else {
        format!("mol2_table_slice_by_index(&this->cur, {})", index)
    }
}
