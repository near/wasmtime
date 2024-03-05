(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i32.const 0x7fffffff
	i64.extend_i32_u
	i64.const 0x000000007fffffff
	call $assert_eq_i64)
 (export "main" (func $main)))
