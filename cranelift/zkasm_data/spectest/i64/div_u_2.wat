(module
 (import "env" "assert_eq_i64" (func $assert_eq_i64 (param i64) (param i64)))
 (func $main
	i64.const 0
	i64.const 1
	i64.div_u
	i64.const 0
	call $assert_eq_i64)
 (start $main))