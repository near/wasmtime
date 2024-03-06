(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 0xDEADBEEFDEADBEEF
	i64.popcnt
	i64.const 48
	call $assert_eq_i64)
 (export "main" (func $main)))
