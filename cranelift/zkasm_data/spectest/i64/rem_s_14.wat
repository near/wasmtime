(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const -7
	i64.const 3
	i64.rem_s
	i64.const -1
	call $assert_eq_i64)
 (export "main" (func $main)))
