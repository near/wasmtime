(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 0
	i64.ctz
	i64.const 64
	call $assert_eq_i64)
 (export "main" (func $main)))
