(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 0xAAAAAAAA55555555
	i64.popcnt
	i64.const 32
	call $assert_eq_i64)
 (export "main" (func $main)))
