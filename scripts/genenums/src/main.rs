use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    fs::remove_file("./bindings/java/src/enums.rs")
        .expect("Error: Unable to remove java/src/enums.rs file");
    
    let mut enums: HashMap<&str, Vec<&str>> = HashMap::new();
    enums.insert("Display", vec!["Block", "Flex", "Grid", "None"]);
    enums.insert("BoxGenerationMode", vec!["Normal", "None"]);
    enums.insert("Position", vec!["Relative", "Absolute"]);
    enums.insert("BoxSizing", vec!["BorderBox", "ContentBox"]);
    enums.insert("Overflow", vec!["Visible", "Clip", "Hidden", "Scroll"]);
    enums.insert("TextAlign", vec!["Auto", "LegacyLeft", "LegacyRight", "LegacyCenter"]);
    enums.insert("GridAutoFlow", vec!["Row", "Column", "RowDense", "ColumnDense"]);
    enums.insert("FlexWrap", vec!["NoWrap", "Wrap", "WrapReverse"]);
    enums.insert("FlexDirection", vec!["Row", "Column", "RowReverse", "ColumnReverse"]);
    enums.insert(
        "AlignContent",
        vec!["Start", "End", "FlexStart", "FlexEnd", "Center", "Stretch", "SpaceBetween", "SpaceEvenly", "SpaceAround"],
    );
    enums.insert("AlignItems", vec!["Start", "End", "FlexStart", "FlexEnd", "Center", "Baseline", "Stretch"]);
    enums.insert("AbsoluteAxis", vec!["Horizontal", "Vertical"]);

    for (key, value) in enums.into_iter() {
        create_enum(key, &value);
        create_transformer(key, &value);
    }
}

/// Enum generators

fn create_enum(name: &str, values: &[&str]) {
    create_java_enum(name, values);
}

fn create_java_enum(name: &str, values: &[&str]) {
    use convert_case::{Case, Casing};

    let package = "com.dioxuslabs.taffy.enums";
    let enum_name = name.to_case(Case::Pascal);

    let auto_gen_comment = "/*
**********************************************************
**            AUTOGENERATED CLASSES FOR TAFFY           **
**  This code was automatically generated. Do not edit. **
**********************************************************
*/
";

    let mut result = format!(
        r"{}package {};

public enum {} {{
",
        auto_gen_comment, package, enum_name
    );

    for value in values.iter() {
        result.push_str("    ");
        result.push_str(&value.to_case(Case::UpperSnake));
        result.push_str(",\n");
    }

    // eliminate the last comma
    if !values.is_empty() {
        result.pop();
        result.pop();
        result.push('\n');
    }

    result.push_str("    ;\n");
    result.push('\n');

    result.push_str("    private final int ordinal;\n");

    result.push('\n');
    result.push_str("    ");
    result.push_str(enum_name.as_str());
    result.push_str("() {\n");
    result.push_str("        this.ordinal = ordinal();\n");
    result.push_str("    }\n");

    result.push_str("}\n");

    fs::create_dir_all("./bindings/java/java/src/main/java/com/dioxuslabs/taffy/enums/")
        .expect("Couldn't create directories");
    let file =
        File::create(format!("./bindings/java/java/src/main/java/com/dioxuslabs/taffy/enums/{}.java", enum_name));
    file.expect("Error: File not found").write_all(result.as_ref()).expect("Error: Couldn't write to file");
}

/// Transformer generators

fn create_transformer(name: &str, values: &[&str]) {
    create_java_tranformer(name, values);
}

fn create_java_tranformer(name: &str, values: &[&str]) {
    let mut file_content: String = "use crate::traits::FromJavaEnum;".to_string();

    if Path::new("./bindings/java/src/enums.rs").exists() {
        file_content = fs::read_to_string(Path::new("./bindings/java/src/enums.rs")).unwrap()
    }

    let mut enum_values: String = "".to_string();
    for (index, value) in values.iter().enumerate() {
        enum_values.push_str(format!("\n            {index} => {name}::{value},").as_str());
    }
    enum_values.push_str("\n            _ => panic!(\"Invalid value: {internal}\"),");

    file_content = format!(
        "use taffy::{name};
{file_content}

impl FromJavaEnum for {name} {{
    const JAVA_CLASS: &'static str = \"Lcom/dioxuslabs/taffy/enums/{name};\";

    fn from_ordinal(internal: i32) -> Option<{name}> {{
        Some(match internal {{{enum_values}
        }})
    }}
}}"
    );

    let file = File::create("./bindings/java/src/enums.rs");
    file.expect("Error: File not found").write_all(file_content.as_ref()).expect("Error: Couldn't write to file");
}
