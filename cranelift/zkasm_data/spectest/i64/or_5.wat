(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 0x7fffffffffffffff
	i64.const 0x8000000000000000
	i64.or
	i64.const -1
	call $assert_eq_i64)
 (start $main))
