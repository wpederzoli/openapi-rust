pub const HELP_MESSAGE: &str = "
description: Generates Rust code from an OpenApi spec file.

usage: openapi-rust <flag> <input_file> -o <output_file>

example: openapi-rust generate openapi.yaml -o generated.rs

Arguments:
    <input_file> The OpenApi YAML file
    -o <output> The file where the generated code will be saved

Options:
    -h, --help Print this help message
";

pub const ERROR_MESSAGE: &str = "
    Bad arguments.

    Usage: openapi-rust <option> <input_file> -o <output_file>
";
