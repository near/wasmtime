(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 0x8000800080008000
	i64.popcnt
	i64.const 4
	call $assert_eq_i64)
 (export "main" (func $main)))
