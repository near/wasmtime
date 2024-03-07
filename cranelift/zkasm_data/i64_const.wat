(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 9223372036854775807
	i64.const 9223372036854775807
	call $assert_eq_i64)
 (export "main" (func $main)))
