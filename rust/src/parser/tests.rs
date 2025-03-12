#[cfg(test)]
mod tests {
    use crate::parser::parse_source_file;
    use crate::ast::Kind;

    fn parse_program(source: &str) -> Result<(), String> {
        match parse_source_file("test.ts", source) {
            Ok(source_file) => {
                if !source_file.diagnostics.is_empty() {
                    return Err(format!("Parse diagnostics: {:?}", source_file.diagnostics));
                }
                Ok(())
            }
            Err(e) => Err(format!("Parse error: {}", e)),
        }
    }

    fn expect_parse_success(source: &str) {
        match parse_program(source) {
            Ok(_) => (),
            Err(e) => panic!("Expected successful parse, but got error: {}", e),
        }
    }

    fn expect_parse_error(source: &str) {
        match parse_program(source) {
            Ok(_) => panic!("Expected parse error, but got success"),
            Err(_) => (),
        }
    }

    #[test]
    fn test_empty_file() {
        expect_parse_success("");
    }

    #[test]
    fn test_simple_function_declaration() {
        expect_parse_success("function foo() {}");
    }

    #[test]
    fn test_function_with_parameters() {
        expect_parse_success("function foo(a, b) {}");
    }

    #[test]
    fn test_function_with_typed_parameters() {
        expect_parse_success("function foo(a: string, b: number) {}");
    }

    #[test]
    fn test_function_with_return_type() {
        expect_parse_success("function foo(): string {}");
    }

    #[test]
    fn test_function_with_typed_parameters_and_return_type() {
        expect_parse_success("function foo(a: string, b: number): string {}");
    }

    #[test]
    fn test_function_with_body() {
        expect_parse_success("function foo() { return 42; }");
    }

    #[test]
    fn test_function_call() {
        expect_parse_success("foo();");
    }

    #[test]
    fn test_function_call_with_arguments() {
        expect_parse_success("foo(a, b);");
    }

    #[test]
    fn test_function_call_with_string_and_number_literals() {
        expect_parse_success("foo(\"hello\", 42);");
    }

    #[test]
    fn test_binary_expression() {
        expect_parse_success("a + b;");
    }

    #[test]
    fn test_demo_program() {
        expect_parse_success("
            function demo(a: string, b: number): string {
                return a + b;
            }
            
            demo(\"hello\", 42);
        ");
    }

    #[test]
    fn test_invalid_function_declaration() {
        expect_parse_error("function {}");
    }

    #[test]
    fn test_invalid_parameter_syntax() {
        expect_parse_error("function foo(a:) {}");
    }

    #[test]
    fn test_invalid_return_type_syntax() {
        expect_parse_error("function foo(): {}");
    }
}