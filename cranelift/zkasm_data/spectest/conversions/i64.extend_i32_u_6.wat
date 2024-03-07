(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i32.const 0x80000000
	i64.extend_i32_u
	i64.const 0x0000000080000000
	call $assert_eq_i64)
 (export "main" (func $main)))
