(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 0x00008000
	i64.popcnt
	i64.const 1
	call $assert_eq_i64)
 (start $main))
