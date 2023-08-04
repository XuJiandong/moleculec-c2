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
/// - C/Rust transformed name: transformed from "field type name" or "item type name",
///   according to their type category, e.g. uint32_t, mol2_cursor_t, XXXType
/// - name, used for class name only
/// - class type name, name with "Type" suffix, e.g. SampleTableType
/// - common array, the set of Array, FixVec and DynVec, they share method like "len", "get"
/// - common table, the set of Table, Struct, they share same method like ".t->XXX"
use crate::utils::{ident_new, Output};
use molecule_codegen::ast::{self, HasName, *};
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use std::fmt;
use std::fmt::Result;

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

pub trait Generator: HasName {
    fn gen_c(&self, output: &mut Output) -> Result {
        format_imp!(output, "// warning: {} not implemented for C ", self.name());
        Ok(())
    }
    fn gen_rust(&self, output: &mut Output) -> Result {
        let name = ident_new(self.name());
        let q = quote! {
            pub struct #name { pub cursor : Cursor }

            impl From<Cursor> for #name {
                fn from(cursor: Cursor) -> Self {
                    Self { cursor }
                }
            }
        };
        format_imp!(output, "{}", q);
        Ok(())
    }
    fn gen_c_decl(&self, output: &mut Output) -> Result {
        let name = self.name();
        format_decl!(output, "struct {};", gen_class_name(name));
        format_decl!(output, "struct {}VTable;", name);
        format_decl!(output, "struct {}VTable *Get{}VTable(void);", name, name);
        format_decl!(output, "struct {0}Type make_{0}(mol2_cursor_t *cur);", name);
        Ok(())
    }
    fn gen_c_def(&self, output: &mut Output) -> Result {
        let name = self.name();
        // definition of class
        format_def!(
            output,
            r#"typedef struct {0}Type {{
        mol2_cursor_t cur;
        {0}VTable *t;
        }} {0}Type;
        "#,
            name
        );
        // definition of "make" class instance
        format_imp!(
            output,
            r#"struct {0}Type make_{0}(mol2_cursor_t *cur) {{
        {0}Type ret;
        ret.cur = *cur;
        ret.t = Get{0}VTable();
        return ret;
        }}"#,
            name
        );
        Ok(())
    }
}

impl Generator for ast::Option_ {
    fn gen_c(&self, output: &mut Output) -> Result {
        let (transformed_name, tc) = get_c_type_category(self.item().typ());
        let name = self.name();
        self.gen_c_decl(output)?;

        format_decl!(output, "bool {0}_is_none_impl(struct {0}Type *);", name);
        format_decl!(output, "bool {0}_is_some_impl(struct {0}Type *);", name);
        format_decl!(
            output,
            "{0} {1}_unwrap_impl(struct {1}Type *);",
            prefix_struct(&transformed_name),
            name
        );
        // --------- declaration above ------------

        // ----------definition below -------------
        // definition of virtual table
        format_def!(output, "typedef struct {0}VTable {{", name);
        format_def!(output, "bool (*is_none)(struct {}Type *);", name);
        format_def!(output, "bool (*is_some)(struct {}Type *);", name);
        format_def!(
            output,
            "{1} (*unwrap)(struct {0}Type *);",
            name,
            prefix_struct(&transformed_name)
        );
        format_def!(output, "}} {}VTable;", name);

        self.gen_c_def(output)?;

        // definition of "get" class vtable
        format_imp!(
            output,
            r#"struct {0}VTable *Get{0}VTable(void) {{
        static {0}VTable s_vtable;
        static int inited = 0;
        if (inited) return &s_vtable; "#,
            name
        );

        format_imp!(output, "s_vtable.is_none = {}_is_none_impl;", name);
        format_imp!(output, "s_vtable.is_some = {}_is_some_impl;", name);
        format_imp!(output, "s_vtable.unwrap = {}_unwrap_impl;", name);

        format_imp!(output, "return &s_vtable; }}");

        // entries of virtual tables
        format_imp!(
            output,
            r#"
        bool {0}_is_none_impl({0}Type *this) {{
          return mol2_option_is_none(&this->cur);
        }}"#,
            name
        );
        format_imp!(
            output,
            r#"
        bool {0}_is_some_impl({0}Type *this) {{
          return !mol2_option_is_none(&this->cur);
        }}"#,
            name
        );

        match tc {
            TypeCategory::Option(_, _, _) => {
                panic!("should not happen in C");
            }
            TypeCategory::Type => {
                format_imp!(
                    output,
                    r#"
            {1} {0}_unwrap_impl({0}Type *this) {{
            {1} ret;
            mol2_cursor_t cur = this->cur;
            ret.cur = cur;
            ret.t = Get{2}VTable();
            return ret;
            }}"#,
                    name,
                    transformed_name,
                    raw_name(&transformed_name)
                );
            }
            TypeCategory::Primitive | TypeCategory::Array => {
                format_imp!(
                    output,
                    r#"
            {1} {0}_unwrap_impl({0}Type *this) {{
            {1} ret;
            ret = {2}(&this->cur);
            return ret;
            }}"#,
                    name,
                    transformed_name,
                    get_c_convert_func(&transformed_name, &tc)
                );
            }
            TypeCategory::FixVec => {
                format_imp!(
                    output,
                    r#"
            {1} {0}_unwrap_impl({0}Type *this) {{
            mol2_cursor_t ret;
            ret = convert_to_rawbytes(&this->cur);
            return ret;
            }}"#,
                    name,
                    transformed_name
                );
            }
        }

        Ok(())
    }
    // Rust version doesn't need this
}

impl Generator for ast::Union {
    fn gen_c(&self, output: &mut Output) -> Result {
        let name = self.name();
        self.gen_c_decl(output)?;

        format_decl!(output, "uint32_t {0}_item_id_impl(struct {0}Type *);", name);
        for item in self.items() {
            let item_type_name = item.typ().name();
            let (transformed_name, _) = get_c_type_category(item.typ());
            format_decl!(
                output,
                "{0} {1}_as_{2}_impl(struct {1}Type *);",
                prefix_struct(&transformed_name),
                name,
                item_type_name
            );
        }
        // --------- declaration above ------------

        // ----------definition below -------------
        // definition of virtual table
        format_def!(output, "typedef struct {0}VTable {{", name);
        format_def!(output, "uint32_t (*item_id)(struct {}Type *);", name);
        for item in self.items() {
            let item_type_name = item.typ().name();
            let (transformed_name, _) = get_c_type_category(item.typ());
            format_def!(
                output,
                "{0} (*as_{2})(struct {1}Type *);",
                prefix_struct(&transformed_name),
                name,
                item_type_name
            );
        }
        format_def!(output, "}} {}VTable;", name);

        self.gen_c_def(output)?;

        // definition of "get" class vtable
        format_imp!(
            output,
            r#"struct {0}VTable *Get{0}VTable(void) {{
        static {0}VTable s_vtable;
        static int inited = 0;
        if (inited) return &s_vtable; "#,
            name
        );

        format_imp!(output, "s_vtable.item_id = {}_item_id_impl;", name);
        for item in self.items() {
            let item_type_name = item.typ().name();
            format_imp!(
                output,
                "s_vtable.as_{1} = {0}_as_{1}_impl;",
                name,
                item_type_name
            );
        }

        format_imp!(output, "return &s_vtable; }}");

        // entries of virtual tables
        format_imp!(
            output,
            r#"
        uint32_t {0}_item_id_impl({0}Type *this) {{
          return mol2_unpack_number(&this->cur);
        }}"#,
            name
        );
        for item in self.items() {
            let item_type_name = item.typ().name();
            let (transformed_name, tc) = get_c_type_category(item.typ());
            match tc {
                TypeCategory::Option(_, _, _) => {
                    panic!("should not happen in C");
                }
                TypeCategory::Type => {
                    format_imp!(
                        output,
                        r#"
                {1} {0}_as_{2}_impl({0}Type *this) {{
                {1} ret;
                mol2_union_t u = mol2_union_unpack(&this->cur);
                ret.cur = u.cursor;
                ret.t = Get{2}VTable();
                return ret;
                }}"#,
                        name,
                        transformed_name,
                        item_type_name
                    );
                }
                TypeCategory::Primitive | TypeCategory::Array => {
                    format_imp!(
                        output,
                        r#"
                {1} {0}_as_{3}_impl({0}Type *this) {{
                {1} ret;
                mol2_union_t u = mol2_union_unpack(&this->cur);
                ret = {2}(&u.cursor);
                return ret;
                }}"#,
                        name,
                        transformed_name,
                        get_c_convert_func(&transformed_name, &tc),
                        item_type_name
                    );
                }
                TypeCategory::FixVec => {
                    format_imp!(
                        output,
                        r#"
                {1} {0}_as_{2}_impl({0}Type *this) {{
                mol2_cursor_t ret;
                mol2_union_t u = mol2_union_unpack(&this->cur);
                ret = convert_to_rawbytes(&u.cursor);
                return ret;
                }}"#,
                        name,
                        transformed_name,
                        item_type_name
                    );
                }
            }
        }

        Ok(())
    }
    fn gen_rust(&self, output: &mut Output) -> Result {
        let name = ident_new(self.name());

        let q = quote! {
            pub struct #name {
                pub cursor: Cursor,
            }

            impl From<Cursor> for #name {
                fn from(cursor: Cursor) -> Self {
                    Self { cursor }
                }
            }

            impl #name {
                pub fn item_id(&self) -> Result<usize, Error> {
                    let item = self.cursor.union_unpack()?;
                    Ok(item.item_id)
                }
            }
        };
        format_imp!(output, "{}", q);

        for item in self.items() {
            let item_type_name = item.typ().name();
            let item_type_name = ident_new(&format!("as_{}", item_type_name.to_lowercase()));
            let (transformed_name, tc) = get_rust_type_category(item.typ());
            let transformed_name = syn::parse_str::<syn::Type>(&transformed_name).unwrap();
            let convert_code = tc.gen_convert_code();
            let q = quote! {
                impl #name {
                    pub fn #item_type_name(&self) -> Result<#transformed_name, Error> {
                        let item = self.cursor.union_unpack()?;
                        let cur = item.cursor.clone();
                        #convert_code
                    }
                }
            };
            format_imp!(output, "{}", q);
        }

        Ok(())
    }
}

impl Generator for ast::Array {
    fn gen_c(&self, output: &mut Output) -> Result {
        let (transformed_name, tc) = get_c_type_category(self.item().typ());
        generate_c_common_array(
            self,
            output,
            self.name(),
            transformed_name.as_str(),
            tc,
            Some(self),
            None,
            None,
        )
    }
    fn gen_rust(&self, output: &mut Output) -> Result {
        let (transformed_name, tc) = get_rust_type_category(self.item().typ());
        generate_rust_common_array(
            output,
            self.name(),
            transformed_name.as_str(),
            tc,
            Some(self),
            None,
            None,
        )
    }
}

impl Generator for ast::Struct {
    fn gen_c(&self, output: &mut Output) -> Result {
        generate_c_common_table(
            self,
            output,
            self.name(),
            self.fields(),
            Some(self.field_sizes()),
        )
    }
    fn gen_rust(&self, output: &mut Output) -> Result {
        generate_rust_common_table(
            self,
            output,
            self.name(),
            self.fields(),
            Some(self.field_sizes()),
        )
    }
}

// in FixVec, all item size is same and known already, the count is unknown.
impl Generator for ast::FixVec {
    fn gen_c(&self, output: &mut Output) -> Result {
        let (transformed_name, tc) = get_c_type_category(self.item().typ());
        generate_c_common_array(
            self,
            output,
            self.name(),
            transformed_name.as_str(),
            tc,
            None,
            Some(self),
            None,
        )
    }
    fn gen_rust(&self, output: &mut Output) -> Result {
        let (transformed_name, tc) = get_rust_type_category(self.item().typ());
        generate_rust_common_array(
            output,
            self.name(),
            transformed_name.as_str(),
            tc,
            None,
            Some(self),
            None,
        )
    }
}

impl Generator for ast::DynVec {
    fn gen_c(&self, output: &mut Output) -> Result {
        let (transformed_name, tc) = get_c_type_category(self.item().typ());
        generate_c_common_array(
            self,
            output,
            self.name(),
            transformed_name.as_str(),
            tc,
            None,
            None,
            Some(self),
        )
    }
    fn gen_rust(&self, output: &mut Output) -> Result {
        let (transformed_name, tc) = get_rust_type_category(self.item().typ());
        generate_rust_common_array(
            output,
            self.name(),
            transformed_name.as_str(),
            tc,
            None,
            None,
            Some(self),
        )
    }
}

impl Generator for ast::Table {
    fn gen_c(&self, output: &mut Output) -> Result {
        generate_c_common_table(self, output, self.name(), self.fields(), None)
    }
    fn gen_rust(&self, output: &mut Output) -> Result {
        generate_rust_common_table(self, output, self.name(), self.fields(), None)
    }
}

// Process:
// 1. Array
// 2. FixVec
// 3. DynVec

// c_type_name, tcs are attributes of item, not container(array, fixvec, dynvec)
// same logic as generate_impl
fn generate_c_common_array(
    gen: &dyn Generator,
    output: &mut Output,
    name: &str,
    c_type_name: &str,
    tc: TypeCategory,
    array: Option<&ast::Array>,
    fixvec: Option<&ast::FixVec>,
    _dynvec: Option<&ast::DynVec>,
) -> Result {
    gen.gen_c_decl(output)?;

    format_decl!(output, "uint32_t {0}_len_impl(struct {0}Type *);", name);
    format_decl!(
        output,
        "{0} {1}_get_impl(struct {1}Type *, uint32_t, bool *);",
        prefix_struct(c_type_name),
        name
    );
    // --------- declaration above ------------

    // ----------definition below -------------
    // definition of virtual table
    format_def!(output, "typedef struct {0}VTable {{", name);
    format_def!(output, "uint32_t (*len)(struct {}Type *);", name);
    format_def!(
        output,
        "{1} (*get)(struct {0}Type *, uint32_t, bool *);",
        name,
        prefix_struct(&c_type_name)
    );
    format_def!(output, "}} {}VTable;", name);

    gen.gen_c_def(output)?;

    // definition of "get" class vtable
    format_imp!(
        output,
        r#"struct {0}VTable *Get{0}VTable(void) {{
            static {0}VTable s_vtable;
            static int inited = 0;
            if (inited) return &s_vtable; "#,
        name
    );

    format_imp!(output, "s_vtable.len = {}_len_impl;", name);
    format_imp!(output, "s_vtable.get = {}_get_impl;", name);

    format_imp!(output, "return &s_vtable; }}");

    // entries of virtual tables
    if let Some(arr) = array {
        format_imp!(
            output,
            r#"
                uint32_t {0}_len_impl({0}Type *this) {{
                  return {1};
                }}"#,
            name,
            arr.item_count()
        );
    } else if fixvec.is_some() {
        format_imp!(
            output,
            r#"
                uint32_t {0}_len_impl({0}Type *this) {{
                return mol2_fixvec_length(&this->cur);
                }}"#,
            name
        );
    } else {
        format_imp!(
            output,
            r#"
                uint32_t {0}_len_impl({0}Type *this) {{
                  return mol2_dynvec_length(&this->cur);
                }}"#,
            name
        );
    }
    generate_c_common_array_impl(output, name, c_type_name, tc, array, fixvec);
    Ok(())
}

fn generate_rust_common_array(
    output: &mut Output,
    name: &str,
    type_name: &str,
    tc: TypeCategory,
    array: Option<&ast::Array>,
    fixvec: Option<&ast::FixVec>,
    _dynvec: Option<&ast::DynVec>,
) -> Result {
    let n = ident_new(name);
    let q = quote! {
        pub struct #n {
            pub cursor: Cursor,
        }

        impl From<Cursor> for #n {
            fn from(cursor: Cursor) -> Self {
                Self { cursor }
            }
        }
    };

    format_imp!(output, "{}", q);

    // len
    if let Some(arr) = array {
        let item_count = Literal::usize_unsuffixed(arr.item_count());
        let q = quote! {
            impl #n {
                pub fn len(&self) -> usize { #item_count }
             }
        };
        format_imp!(output, "{}", q);
    } else if fixvec.is_some() {
        let q = quote! {
            impl #n {
                pub fn len(&self) -> Result<usize, Error> {  self.cursor.fixvec_length()  }
            }
        };
        format_imp!(output, "{}", q);
    } else {
        let q = quote! {
            impl #n {
                pub fn len(&self) -> Result<usize, Error> { self.cursor.dynvec_length() }
            }
        };
        format_imp!(output, "{}", q);
    }

    generate_rust_common_array_impl(output, name, type_name, tc, array, fixvec);
    Ok(())
}

fn generate_c_common_array_impl(
    output: &mut Output,
    name: &str,
    c_type_name: &str,
    tc: TypeCategory,
    array: Option<&Array>,
    fixvec: Option<&FixVec>,
) {
    let slice_by = if let Some(fv) = fixvec {
        format!(
            "mol2_fixvec_slice_by_index(&this->cur, {}, index)",
            fv.item_size()
        )
    } else if let Some(arr) = array {
        format!(
            "mol2_slice_by_offset2(&this->cur, {0}*index, {0})",
            arr.item_size()
        )
    } else {
        "mol2_dynvec_slice_by_index(&this->cur, index)".into()
    };
    match tc {
        TypeCategory::Option(_, _, _) => {
            panic!("should not happen in C");
        }
        TypeCategory::Type => {
            format_imp!(
                output,
                r#"
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
            }}"#,
                name,
                c_type_name,
                raw_name(&c_type_name),
                slice_by
            );
        }
        TypeCategory::Primitive | TypeCategory::Array => {
            format_imp!(
                output,
                r#"
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
            }}"#,
                name,
                c_type_name,
                get_c_convert_func(&c_type_name, &tc),
                slice_by
            );
        }
        TypeCategory::FixVec => {
            format_imp!(
                output,
                r#"
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
            }}"#,
                name,
                slice_by
            );
        }
    }
}

fn generate_rust_common_array_impl(
    output: &mut Output,
    name: &str,
    type_name: &str,
    tc: TypeCategory,
    array: Option<&Array>,
    fixvec: Option<&FixVec>,
) {
    let slice_by = if let Some(fv) = fixvec {
        format!("fixvec_slice_by_index({}, index)", fv.item_size())
    } else if let Some(arr) = array {
        format!("slice_by_offset({0}*index, {0})", arr.item_size())
    } else {
        format!("dynvec_slice_by_index(index)")
    };
    let convert_code = tc.gen_convert_code();
    format_imp!(
        output,
        r#"
        impl {0} {{
            pub fn get(&self, index: usize) -> Result<{1}, Error> {{
                let cur = self.cursor.{2}?;
                {3}
            }}
        }}
        "#,
        name,
        type_name,
        slice_by,
        convert_code
    );
}

fn prefix_struct(field_type: &str) -> String {
    match field_type {
        "mol2_cursor_t" | "int8_t" | "uint8_t" | "int16_t" | "uint16_t" | "int32_t"
        | "uint32_t" | "int64_t" | "uint64_t" => String::from(field_type),
        _ => {
            format!("struct {}", field_type)
        }
    }
}

// Table/Struct
fn generate_c_common_table(
    gen: &dyn Generator,
    output: &mut Output,
    name: &str,
    fields: &[FieldDecl],
    field_sizes: Option<&[usize]>,
) -> Result {
    gen.gen_c_decl(output)?;

    for field in fields {
        let field_name = &field.name();
        let (transformed_name, _) = get_c_type_category(field.typ());
        format_decl!(
            output,
            "{0} {1}_get_{2}_impl(struct {1}Type *);",
            prefix_struct(&transformed_name),
            name,
            field_name
        );
    }
    // --------- declaration above ------------

    // ----------definition below -------------
    // definition of virtual table
    format_def!(output, "typedef struct {0}VTable {{", name);
    for field in fields {
        let field_name = &field.name();
        let (transformed_name, _) = get_c_type_category(field.typ());
        format_def!(
            output,
            "{0} (*{1})(struct {2}Type *);",
            prefix_struct(&transformed_name),
            field_name,
            name
        );
    }
    format_def!(output, "}} {}VTable;", name);
    gen.gen_c_def(output)?;
    // definition of "get" class vtable
    format_imp!(
        output,
        r#"struct {0}VTable *Get{0}VTable(void) {{
    static {0}VTable s_vtable;
    static int inited = 0;
    if (inited) return &s_vtable;
    "#,
        name
    );
    for field in fields {
        let field_name = &field.name();
        format_imp!(output, "s_vtable.{0} = {1}_get_{0}_impl;", field_name, name);
    }
    format_imp!(output, "return &s_vtable; }}");
    for (index, field) in fields.iter().enumerate() {
        generate_c_common_table_impl(output, name, field, index, field_sizes);
    }
    Ok(())
}

fn generate_rust_common_table(
    _gen: &dyn Generator,
    output: &mut Output,
    name: &str,
    fields: &[FieldDecl],
    field_sizes: Option<&[usize]>,
) -> Result {
    let n = ident_new(name);
    let q = quote! {

        pub struct #n {
            pub cursor: Cursor,
        }

        impl From<Cursor> for #n {
            fn from(cursor: Cursor) -> Self {
                #n { cursor }
            }
        }
    };
    format_imp!(output, "{}", q);
    for (index, field) in fields.iter().enumerate() {
        generate_rust_common_table_impl(output, name, field, index, field_sizes);
    }
    Ok(())
}

fn generate_c_common_table_impl(
    output: &mut Output,
    name: &str,
    field: &FieldDecl,
    index: usize,
    field_sizes: Option<&[usize]>,
) {
    let field_name = field.name();
    let (transformed_name, tc) = get_c_type_category(field.typ());
    let slice_by = generate_c_slice_by(index, &field_sizes);
    match tc {
        TypeCategory::Option(_, _, _) => {
            panic!("should not happen in C");
        }
        TypeCategory::Type => {
            format_imp!(
                output,
                r#"
            {2} {0}_get_{1}_impl({0}Type *this) {{
            {2} ret;
            mol2_cursor_t cur = {4};
            ret.cur = cur;
            ret.t = Get{3}VTable();
            return ret;
            }}"#,
                name,
                field_name,
                transformed_name,
                raw_name(&transformed_name),
                slice_by
            );
        }
        TypeCategory::Primitive | TypeCategory::Array => {
            format_imp!(
                output,
                r#"
            {2} {0}_get_{1}_impl({0}Type *this) {{
            {2} ret;
            mol2_cursor_t ret2 = {3};
            ret = {4}(&ret2);
            return ret;
            }}"#,
                name,
                field_name,
                transformed_name,
                slice_by,
                get_c_convert_func(&transformed_name, &tc)
            );
        }
        TypeCategory::FixVec => {
            format_imp!(
                output,
                r#"
            {2} {0}_get_{1}_impl({0}Type *this) {{
            mol2_cursor_t ret;
            mol2_cursor_t re2 = {3};
            ret = convert_to_rawbytes(&re2);
            return ret;
            }}"#,
                name,
                field_name,
                transformed_name,
                slice_by
            );
        }
    }
}

fn generate_rust_common_table_impl(
    output: &mut Output,
    name: &str,
    field: &FieldDecl,
    index: usize,
    field_sizes: Option<&[usize]>,
) {
    let field_name = field.name();
    let (transformed_name, tc) = get_rust_type_category(field.typ());
    let slice_by = generate_rust_slice_by(index, &field_sizes);
    let convert_code = tc.gen_convert_code();
    format_imp!(
        output,
        r#"
        impl {0} {{
            pub fn {1}(&self) -> Result<{2}, Error> {{
                let cur = self.cursor.{3}?;
                {4}
             }}
         }}
        "#,
        name,
        field_name,
        transformed_name,
        slice_by,
        convert_code
    );
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

// 4. category 4, Option
enum TypeCategory {
    Primitive,
    Array,
    FixVec,
    Type,
    // 1st: nested level
    // 2nd: is nested type is FixVec or not
    // 3rd: has From<T>
    Option(u32, bool, bool),
}

impl TypeCategory {
    pub fn is_fixvec(&self) -> bool {
        match self {
            Self::FixVec => true,
            _ => false,
        }
    }
    pub fn has_from(&self) -> bool {
        match self {
            Self::Type | Self::Array => true,

            _ => false,
        }
    }
    pub fn gen_convert_code(&self) -> TokenStream {
        let code = match self {
            &TypeCategory::Option(level, flag, has_from) => {
                if level == 1 {
                    if flag {
                        quote! {
                            if cur.option_is_none() {
                                Ok(None)
                            } else {
                                let cur = cur.convert_to_rawbytes()?;
                                Ok(Some(cur.into()))
                            }
                        }
                    } else {
                        if has_from {
                            quote! {
                                if cur.option_is_none() {
                                    Ok(None)
                                } else {
                                    Ok(Some(cur.into()))
                                }
                            }
                        } else {
                            quote! {
                                if cur.option_is_none() {
                                    Ok(None)
                                } else {
                                    Ok(Some(cur.try_into()?))
                                }
                            }
                        }
                    }
                } else if level == 2 {
                    if flag {
                        quote! {
                            if cur.option_is_none() {
                                Ok(None)
                            } else {
                                let cur = cur.convert_to_rawbytes()?;
                                Ok(Some(Some(cur.try_into())))
                           }
                        }
                    } else {
                        quote! {
                            if cur.option_is_none() {
                                Ok(None)
                            } else {
                                Ok(Some(Some(cur.into())))
                            }
                        }
                    }
                } else {
                    panic!("can't support")
                }
            }
            TypeCategory::Type => quote! { Ok(cur.into()) },
            TypeCategory::Primitive => quote! { cur.try_into() },
            TypeCategory::Array => quote! { Ok(cur) },
            TypeCategory::FixVec => {
                quote! {
                    cur.convert_to_rawbytes()
                }
            }
        };
        code
    }
}

// return: transformed name and it's type category
fn get_c_type_category(typ: &TopDecl) -> (String, TypeCategory) {
    let name = typ.name();
    let mut tc = TypeCategory::Primitive;

    let mut transformed_name = gen_class_name(&String::from(name));
    match typ {
        // if the field type is array and the field type name is "uint8", "int8" ...
        // then it's primitive
        TopDecl::Array(a) => {
            let field_type_name = name.to_lowercase();
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
                        tc = TypeCategory::Array;
                        "mol2_cursor_t"
                    } else {
                        tc = TypeCategory::Type;
                        transformed_name.as_str()
                    }
                }
            };
            transformed_name = String::from(new_name)
        }
        TopDecl::Primitive(_) => {
            transformed_name = String::from("uint8_t");
        }
        TopDecl::FixVec(v) => {
            // FixVec is different than Array: it has a header.
            if let TopDecl::Primitive(_) = v.item().typ().as_ref() {
                // array of byte
                transformed_name = String::from("mol2_cursor_t");
                tc = TypeCategory::FixVec;
            } else {
                tc = TypeCategory::Type;
            }
        }
        _ => {
            tc = TypeCategory::Type;
        }
    }
    (transformed_name, tc)
}

// see TypeCategory
fn get_rust_type_category(typ: &TopDecl) -> (String, TypeCategory) {
    let name = typ.name();
    let mut tc = TypeCategory::Primitive;
    let mut transformed_name = String::from(name);
    match typ {
        // if the field type is array and the field type name is "uint8", "int8" ...
        // then it's primitive
        TopDecl::Array(a) => {
            let field_type_name = name.to_lowercase();
            let new_name = match field_type_name.as_ref() {
                // see https://github.com/XuJiandong/moleculec-c2#extra-support-for-known-types
                "uint8" => "u8",
                "int8" => "i8",
                "uint16" => "u16",
                "int16" => "i16",
                "uint32" => "u32",
                "int32" => "i32",
                "uint64" => "u64",
                "int64" => "i64",
                _ => {
                    if let TopDecl::Primitive(_) = a.item().typ().as_ref() {
                        // array of byte
                        tc = TypeCategory::Array;
                        "Cursor"
                    } else {
                        // array of Types
                        tc = TypeCategory::Type;
                        transformed_name.as_str()
                    }
                }
            };
            transformed_name = String::from(new_name);
        }
        TopDecl::Primitive(_) => {
            transformed_name = String::from("u8");
        }
        TopDecl::FixVec(v) => {
            // FixVec is different than Array: it has a header.
            if let TopDecl::Primitive(_) = v.item().typ().as_ref() {
                // array of byte
                transformed_name = String::from("Cursor");
                tc = TypeCategory::FixVec;
            } else {
                tc = TypeCategory::Type;
            }
        }
        TopDecl::Option_(o) => {
            // Option<Script>, etc
            let (inner_name, inner_tc) = get_rust_type_category(o.item().typ());
            match inner_tc {
                TypeCategory::Option(level, flag, has_from) => {
                    tc = TypeCategory::Option(level + 1, flag, has_from);
                    transformed_name = format!("Option<{}>", inner_name);
                }
                _ => {
                    transformed_name = format!("Option<{}>", inner_name);
                    tc = TypeCategory::Option(1, inner_tc.is_fixvec(), inner_tc.has_from());
                }
            }
        }
        _ => {
            tc = TypeCategory::Type;
        }
    }
    (transformed_name, tc)
}

fn raw_name(type_name: &str) -> String {
    String::from(type_name).replace("Type", "")
}

fn get_c_convert_func(c_type_name: &str, tc: &TypeCategory) -> &'static str {
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
            if let TypeCategory::Array = tc {
                "convert_to_array"
            } else {
                panic!("should not happen");
            }
        }
    }
}

// generate slice function for struct or table.
// struct: mol2_slice_by_offset(&this->cur, 4, 2)
// table:  mol2_table_slice_by_index(&this->cur, 1);
fn generate_c_slice_by(index: usize, fields_sizes: &Option<&[usize]>) -> String {
    if let Some(fs) = fields_sizes {
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

fn generate_rust_slice_by(index: usize, fields_sizes: &Option<&[usize]>) -> String {
    if let Some(fs) = fields_sizes {
        let size = fs[index];
        let mut offset = 0;
        for i in (0..index).rev() {
            offset += fs[i];
        }
        format!("slice_by_offset({}, {})", offset, size)
    } else {
        format!("table_slice_by_index({})", index)
    }
}
