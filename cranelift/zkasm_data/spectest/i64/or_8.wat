(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 0xffffffffffffffff
	i64.const 0xffffffffffffffff
	i64.or
	i64.const 0xffffffffffffffff
	call $assert_eq_i64)
 (export "main" (func $main)))
