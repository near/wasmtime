(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const -1
	i64.popcnt
	i64.const 64
	call $assert_eq_i64)
 (start $main))
