
/// ## Terminology
/// - field, existing in "struct", "table"
/// - field name, name of field
/// - field type, type of field
/// - field type name, name of field type
/// - item, existing in "dynvec", "union" and "option"
/// - item name, there is no such thing, the item is without name
/// - item type, type of item
/// - item type name, name of item type
/// - type name, item type name or field type name
/// - raw name, a struct name without "Type", same as name
/// - type category, field type or item type are classified into 3 categories, see TypeCategory.
///   Different category has different implementation.
/// - C type name: transformed from "field type name" or "item type name",
///   according to their type category, e.g. uint32_t, mol2_cursor_t, XXXType
/// - name, used for class name only
/// - class type name, name with "Type" suffix, e.g. SampleTableType
/// - common array, the set of Array, FixVec and DynVec, they share method like "len", "get"
/// - common table, the set of Table, Struct, they share same method like ".t->XXX"

use crate::utils::{Output, generate};
use core::mem::size_of;
use molecule_codegen::ast::{self, DefaultContent, HasName, *};
use std::fmt::{Write, Result};
use std::fmt;

macro_rules! format_decl {
    ($output:expr, $($arg:tt)*) => {{
        let res = fmt::format(format_args!($($arg)*));
        $output.write_decl(&res);
    }}
}

macro_rules! format_def {
    ($output:expr, $($arg:tt)*) => {{
        let res = fmt::format(format_args!($($arg)*));
        $output.write_def(&res);
    }}
}

macro_rules! format_imp {
    ($output:expr, $($arg:tt)*) => {{
        let res = fmt::format(format_args!($($arg)*));
        $output.write_imp(&res);
    }}
}

pub trait Generator: HasName  {
    fn generate(&self, output: &mut Output) -> Result;
    fn gen_decl(&self, output: &mut Output) -> Result {
        let name = self.name();
        format_decl!(output, "struct {};", gen_class_name(name));
        format_decl!(output, "struct {}VTable;", name);
        format_decl!(output, "struct {}VTable *Get{}VTable(void);", name, name);
        format_decl!(output, "struct {0}Type make_{0}(mol2_cursor_t *cur);", name);
        Ok(())
    }
    fn gen_def(&self, output: &mut Output) -> Result {
        let name = self.name();
        // definition of class
        format_def!(output, r#"typedef struct {0}Type {{
        mol2_cursor_t cur;
        {0}VTable *t;
        }} {0}Type;
        "#, name);
        // definition of "make" class instance
        format_imp!(output, r#"struct {0}Type make_{0}(mol2_cursor_t *cur) {{
        {0}Type ret;
        ret.cur = *cur;
        ret.t = Get{0}VTable();
        return ret;
        }}"#, name);
        Ok(())
    }
}


impl Generator for ast::Option_ {
    fn generate(&self, output: &mut Output) -> Result {
        let (c_type_name, ftc) = get_type_category(self.item().typ(), self.item().typ().name());
        let name = self.name();
        self.gen_decl(output)?;

        format_decl!(output, "bool {0}_is_none_impl(struct {0}Type *);", name);
        format_decl!(output, "bool {0}_is_some_impl(struct {0}Type *);", name);
        format_decl!(output, "{0} {1}_unwrap_impl(struct {1}Type *);", prefix_struct(&c_type_name), name);
        // --------- declaration above ------------

        // ----------definition below -------------
        // definition of virtual table
        format_def!(output, "typedef struct {0}VTable {{", name);
        format_def!(output, "bool (*is_none)(struct {}Type *);", name);
        format_def!(output, "bool (*is_some)(struct {}Type *);", name);
        format_def!(output, "{1} (*unwrap)(struct {0}Type *);", name, prefix_struct(&c_type_name));
        format_def!(output, "}} {}VTable;", name);

        self.gen_def(output)?;

        // definition of "get" class vtable
        format_imp!(output, r#"struct {0}VTable *Get{0}VTable(void) {{
        static {0}VTable s_vtable;
        static int inited = 0;
        if (inited) return &s_vtable; "#, name);

        format_imp!(output, "s_vtable.is_none = {}_is_none_impl;", name);
        format_imp!(output, "s_vtable.is_some = {}_is_some_impl;", name);
        format_imp!(output, "s_vtable.unwrap = {}_unwrap_impl;", name);

        format_imp!(output, "return &s_vtable; }}");

        // entries of virtual tables
        format_imp!(output, r#"
        bool {0}_is_none_impl({0}Type *this) {{
          return mol2_option_is_none(&this->cur);
        }}"#, name);
        format_imp!(output, r#"
        bool {0}_is_some_impl({0}Type *this) {{
          return !mol2_option_is_none(&this->cur);
        }}"#, name);

        match ftc {
            TypeCategory::Type => {
                format_imp!(output, r#"
            {1} {0}_unwrap_impl({0}Type *this) {{
            {1} ret;
            mol2_cursor_t cur = this->cur;
            ret.cur = cur;
            ret.t = Get{2}VTable();
            return ret;
            }}"#, name, c_type_name, raw_name(&c_type_name));
            },
            TypeCategory::Primitive| TypeCategory::Array => {
                format_imp!(output, r#"
            {1} {0}_unwrap_impl({0}Type *this) {{
            {1} ret;
            ret = {2}(&this->cur);
            return ret;
            }}"#, name, c_type_name, get_convert_func(&c_type_name, &ftc));
            },
            TypeCategory::FixVec => {
                format_imp!(output, r#"
            {1} {0}_unwrap_impl({0}Type *this) {{
            mol2_cursor_t ret;
            ret = convert_to_rawbytes(&this->cur);
            return ret;
            }}"#, name, c_type_name);
            },
        }

        Ok(())
    }
}

impl Generator for ast::Union {
    fn generate(&self, output: &mut Output) -> Result {
        let name = self.name();
        self.gen_decl(output)?;

        format_decl!(output, "uint32_t {0}_item_id_impl(struct {0}Type *);", name);
        for item in self.items() {
            let item_type_name = item.typ().name();
            let (c_type_name, _) = get_type_category(item.typ(), item_type_name);
            format_decl!(output, "{0} {1}_as_{2}_impl(struct {1}Type *);",
            prefix_struct(&c_type_name), name, item_type_name);
        }
        // --------- declaration above ------------

        // ----------definition below -------------
        // definition of virtual table
        format_def!(output, "typedef struct {0}VTable {{", name);
        format_def!(output, "uint32_t (*item_id)(struct {}Type *);", name);
        for item in self.items() {
            let item_type_name = item.typ().name();
            let (c_type_name, _) = get_type_category(item.typ(), item_type_name);
            format_def!(output, "{0} (*as_{2})(struct {1}Type *);",
            prefix_struct(&c_type_name), name, item_type_name);
        }
        format_def!(output, "}} {}VTable;", name);

        self.gen_def(output)?;

        // definition of "get" class vtable
        format_imp!(output, r#"struct {0}VTable *Get{0}VTable(void) {{
        static {0}VTable s_vtable;
        static int inited = 0;
        if (inited) return &s_vtable; "#, name);

        format_imp!(output, "s_vtable.item_id = {}_item_id_impl;", name);
        for item in self.items() {
            let item_type_name = item.typ().name();
            format_imp!(output, "s_vtable.as_{1} = {0}_as_{1}_impl;", name, item_type_name);
        }

        format_imp!(output, "return &s_vtable; }}");

        // entries of virtual tables
        format_imp!(output, r#"
        uint32_t {0}_item_id_impl({0}Type *this) {{
          return mol2_unpack_number(&this->cur);
        }}"#, name);
        for item in self.items() {
            let item_type_name = item.typ().name();
            let (c_type_name, ftc) = get_type_category(item.typ(), item.typ().name());
            match ftc {
                TypeCategory::Type => {
                    format_imp!(output, r#"
                {1} {0}_as_{2}_impl({0}Type *this) {{
                {1} ret;
                mol2_union_t u = mol2_union_unpack(&this->cur);
                ret.cur = u.cursor;
                ret.t = Get{2}VTable();
                return ret;
                }}"#, name, c_type_name, item_type_name);
                },
                TypeCategory::Primitive | TypeCategory::Array => {
                    format_imp!(output, r#"
                {1} {0}_as_{3}_impl({0}Type *this) {{
                {1} ret;
                mol2_union_t u = mol2_union_unpack(&this->cur);
                ret = {2}(&u.cursor);
                return ret;
                }}"#, name, c_type_name, get_convert_func(&c_type_name, &ftc), item_type_name);
                },
                TypeCategory::FixVec => {
                    format_imp!(output, r#"
                {1} {0}_as_{2}_impl({0}Type *this) {{
                mol2_cursor_t ret;
                mol2_union_t u = mol2_union_unpack(&this->cur);
                ret = convert_to_rawbytes(&u.cursor);
                return ret;
                }}"#, name, c_type_name, item_type_name);
                },
            }
        }

        Ok(())
    }
}

impl Generator for ast::Array {
    fn generate(&self, output: &mut Output) -> Result {
        let (c_type_name, ftc) = get_type_category(self.item().typ(), self.item().typ().name());
        generate_common_array(self, output, self.name(), c_type_name.as_str(), ftc, Some(self), None, None)
    }
}

impl Generator for ast::Struct {
    fn generate(&self, output: &mut Output) -> Result {
        generate_common_table(self, output, self.name(), self.fields(), Some(self.field_sizes()))
    }
}

// in FixVec, all item size is same and known already, the count is unknown.
impl Generator for ast::FixVec {
    fn generate(&self, output: &mut Output) -> Result {
        let (c_type_name, ftc) = get_type_category(self.item().typ(), self.item().typ().name());
        generate_common_array(self, output, self.name(), c_type_name.as_str(), ftc, None, Some(self), None)
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
        gen_class_name(item_type.name())
    }
}

// the possible types as Array's item
// 1. Array
// 2. Byte
//
fn get_array_item_type(array: &ast::Array) -> (Option<String>, Option<TypeCategory>) {
    let item_type = array.item().typ();
    if let TopDecl::Primitive(a) = item_type.as_ref() {
        (None, None)
    } else {
        if let TopDecl::Array(aa) = item_type.as_ref() {
            if let TopDecl::Primitive(aaa) = aa.item().typ().as_ref() {
                let (c_type_name, ftc) = get_type_category(item_type, item_type.name());
                return (Some(c_type_name), Some(ftc))
            }
        }
        (Some(gen_class_name(item_type.name())), Some(TypeCategory::Type))
    }
}

fn get_fixvec_item_type(fixvec: &ast::FixVec) -> (Option<String>, Option<TypeCategory>) {
    let item_type = fixvec.item().typ();
    if let TopDecl::Primitive(a) = item_type.as_ref() {
        (None, None)
    } else {
        if let TopDecl::Array(aa) = item_type.as_ref() {
            if let TopDecl::Primitive(aaa) = aa.item().typ().as_ref() {
                let (c_type_name, ftc) = get_type_category(item_type, item_type.name());
                return (Some(c_type_name), Some(ftc));
            }
        }
        (Some(gen_class_name(item_type.name())), Some(TypeCategory::Type))
    }
}


// Process:
// 1. Array
// 2. FixVec
// 3. DynVec

// c_type_name, ftc are attributes of item, not container(array, fixvec, dynvec)
// same logic as generate_impl
fn generate_common_array(gen: &dyn Generator, output: &mut Output,
                         name: &str, c_type_name: &str, ftc: TypeCategory, array: Option<&ast::Array>,
                         fixvec: Option<&ast::FixVec>, dynvec: Option<&ast::DynVec>) -> Result {
    gen.gen_decl(output)?;

    format_decl!(output, "uint32_t {0}_len_impl(struct {0}Type *);", name);
    format_decl!(output, "{0} {1}_get_impl(struct {1}Type *, uint32_t, bool *);", prefix_struct(&c_type_name), name);
    // --------- declaration above ------------

    // ----------definition below -------------
    // definition of virtual table
    format_def!(output, "typedef struct {0}VTable {{", name);
    format_def!(output, "uint32_t (*len)(struct {}Type *);", name);
    format_def!(output, "{1} (*get)(struct {0}Type *, uint32_t, bool *);", name, prefix_struct(&c_type_name));
    format_def!(output, "}} {}VTable;", name);

    gen.gen_def(output)?;

    // definition of "get" class vtable
    format_imp!(output, r#"struct {0}VTable *Get{0}VTable(void) {{
            static {0}VTable s_vtable;
            static int inited = 0;
            if (inited) return &s_vtable; "#, name);

    format_imp!(output, "s_vtable.len = {}_len_impl;", name);
    format_imp!(output, "s_vtable.get = {}_get_impl;", name);

    format_imp!(output, "return &s_vtable; }}");

    // entries of virtual tables
    if let Some(arr) = array {
        format_imp!(output, r#"
                uint32_t {0}_len_impl({0}Type *this) {{
                  return {1};
                }}"#, name, arr.item_count());
    } else if let Some(fv) = fixvec {
        format_imp!(output, r#"
                uint32_t {0}_len_impl({0}Type *this) {{
                return mol2_fixvec_length(&this->cur);
                }}"#, name);
    } else {
        format_imp!(output, r#"
                uint32_t {0}_len_impl({0}Type *this) {{
                  return mol2_dynvec_length(&this->cur);
                }}"#, name);
    }
    generate_common_array_impl(output, name, c_type_name, ftc, array, fixvec);
    Ok(())
}

fn generate_common_array_impl(output: &mut Output, name: &str, c_type_name: &str,
                              ftc: TypeCategory, array: Option<&Array>, fixvec: Option<&FixVec>) {
    let slice_by = if let Some(fv) = fixvec {
        format!("mol2_fixvec_slice_by_index(&this->cur, {}, index)", fv.item_size())
    } else if let Some(arr) = array {
        format!("mol2_slice_by_offset2(&this->cur, {0}*index, {0})", arr.item_size())
    } else {
        format!("mol2_dynvec_slice_by_index(&this->cur, index)")
    };
    match ftc {
        TypeCategory::Type => {
            format_imp!(output, r#"
            {1} {0}_get_impl({0}Type *this,
                                    uint32_t index, bool *existing) {{
            {1} ret = {{0}};
            mol2_cursor_res_t res = {3};
            if (res.errno != MOL2_OK) {{
                *existing = false;
                return ret;
            }} else {{
                *existing = true;
            }}
            ret.cur = res.cur;
            ret.t = Get{2}VTable();
            return ret;
            }}"#, name, c_type_name, raw_name(&c_type_name), slice_by);
        },
        TypeCategory::Primitive | TypeCategory::Array => {
            format_imp!(output, r#"
            {1} {0}_get_impl({0}Type *this, uint32_t index, bool *existing) {{
            {1} ret = {{0}};
            mol2_cursor_res_t res = {3};
            if (res.errno != MOL2_OK) {{
                *existing = false;
                return ret;
            }} else {{
                *existing = true;
            }}
            ret = {2}(&res.cur);
            return ret;
            }}"#, name, c_type_name, get_convert_func(&c_type_name, &ftc), slice_by);
        },
        TypeCategory::FixVec => {
            format_imp!(output, r#"
            mol2_cursor_t {0}_get_impl({0}Type *this,
                                    uint32_t index, bool *existing) {{
            mol2_cursor_t ret = {{0}};
            mol2_cursor_res_t res = {1};
            if (res.errno != MOL2_OK) {{
                *existing = false;
                return ret;
            }} else {{
                *existing = true;
            }}
            return convert_to_rawbytes(&res.cur);
            }}"#, name, slice_by);
        }
    }
}


impl Generator for ast::DynVec {
    fn generate(&self, output: &mut Output) -> Result {
        let (c_type_name, ftc) = get_type_category(self.item().typ(), self.item().typ().name());
        generate_common_array(self, output, self.name(), c_type_name.as_str(), ftc, None, None, Some(self))
    }
}

fn prefix_struct(field_type: &str) -> String {
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
fn generate_common_table(gen: &dyn Generator, output: &mut Output, name: &str,
                         fields: &[FieldDecl], field_sizes: Option<&[usize]>) -> Result {
    gen.gen_decl(output)?;

    for field in fields {
        let field_name = &field.name();
        let (field_type, _) = get_type_category(field.typ(), field.typ().name());
        format_decl!(output, "{0} {1}_get_{2}_impl(struct {1}Type *);",
                                   prefix_struct(&field_type), name, field_name);
    }
    // --------- declaration above ------------

    // ----------definition below -------------
    // definition of virtual table
    format_def!(output, "typedef struct {0}VTable {{", name);
    for field in fields {
        let field_name = &field.name();
        let (c_type_name, _) = get_type_category(field.typ(), field.typ().name());
        format_def!(output, "{0} (*{1})(struct {2}Type *);", prefix_struct(&c_type_name), field_name, name);
    }
    format_def!(output, "}} {}VTable;", name);
    gen.gen_def(output)?;
    // definition of "get" class vtable
    format_imp!(output, r#"struct {0}VTable *Get{0}VTable(void) {{
    static {0}VTable s_vtable;
    static int inited = 0;
    if (inited) return &s_vtable;
    "#, name);
    for field in fields {
        let field_name = &field.name();
        let (field_type, _) = get_type_category(field.typ(), field.typ().name());
        format_imp!(output, "s_vtable.{0} = {1}_get_{0}_impl;", field_name, name);
    }
    format_imp!(output, "return &s_vtable; }}");
    // entries of virtual tables
    let mut index : usize = 0;
    for field in fields {
        generate_common_table_impl(output, name, field, index, field_sizes);
        index += 1;
    }
    Ok(())
}

fn generate_common_table_impl(output: &mut Output, name: &str, field: &FieldDecl,
                              index: usize, field_sizes: Option<&[usize]>) {
    let field_name = field.name();
    let (field_type, ftc) = get_type_category(field.typ(), field.typ().name());
    let slice_by = generate_slice_by(index, &field_sizes);
    match ftc {
        TypeCategory::Type => {
            format_imp!(output, r#"
            {2} {0}_get_{1}_impl({0}Type *this) {{
            {2} ret;
            mol2_cursor_t cur = {4};
            ret.cur = cur;
            ret.t = Get{3}VTable();
            return ret;
            }}"#, name, field_name, field_type, raw_name(&field_type), slice_by);
        },
        TypeCategory::Primitive| TypeCategory::Array => {
            format_imp!(output, r#"
            {2} {0}_get_{1}_impl({0}Type *this) {{
            {2} ret;
            mol2_cursor_t ret2 = {3};
            ret = {4}(&ret2);
            return ret;
            }}"#, name, field_name, field_type, slice_by, get_convert_func(&field_type, &ftc));
        },
        TypeCategory::FixVec => {
            format_imp!(output, r#"
            {2} {0}_get_{1}_impl({0}Type *this) {{
            mol2_cursor_t ret;
            mol2_cursor_t re2 = {3};
            ret = convert_to_rawbytes(&re2);
            return ret;
            }}"#, name, field_name, field_type, slice_by);
        },
    }
}

impl Generator for ast::Table {
    fn generate(&self, output: &mut Output) -> Result {
        generate_common_table(self, output, self.name(), self.fields(), None)
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

enum TypeCategory {
    Primitive,
    Array,
    FixVec,
    Type
}

fn get_type_category(typ: &TopDecl, type_name: &str) -> (String, TypeCategory) {
    // let field_type_name = field.typ().name();
    let mut ftc = TypeCategory::Primitive;

    let mut c_type_name = format!("{}", gen_class_name(&String::from(type_name)));
    match typ {
        // if the field type is array and the field type name is "uint8", "int8" ...
        // then it's primitive
        TopDecl::Array(a) => {
            let field_type_name = type_name.to_lowercase();
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
                    if let TopDecl::Primitive(_) = a.item().typ().as_ref() {
                        // array of byte
                        ftc = TypeCategory::Array;
                        "mol2_cursor_t"
                    } else {
                        ftc = TypeCategory::Type;
                        c_type_name.as_str()
                    }
                },
            };
            c_type_name = String::from(new_name)
        },
        TopDecl::Primitive(p) => {
            c_type_name = String::from("uint8_t");
        },
        TopDecl::FixVec(v) => {
            // FixVec is different than Array: it has a header.
            if let TopDecl::Primitive(_) = v.item().typ().as_ref() {
                // array of byte
                c_type_name = String::from("mol2_cursor_t");
                ftc = TypeCategory::FixVec;
            } else {
                ftc = TypeCategory::Type;
            }
        },
        _ => {
            ftc = TypeCategory::Type;
        },
    }
    (c_type_name, ftc)
}

fn raw_name(type_name: &str) -> String {
    String::from(type_name).replace("Type", "")
}

fn get_convert_func<'a>(c_type_name: &'a str, ftc: &TypeCategory) -> &'a str {
    let t = c_type_name.to_lowercase();
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
            if let TypeCategory::Array = ftc {
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
