(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 1
	i64.const 1
	i64.and
	i64.const 1
	call $assert_eq_i64)
 (export "main" (func $main)))
