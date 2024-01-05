(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i64.const 0
	i64.const 0
	i64.eq
	i32.const 1
	call $assert_eq_i32)
 (start $main))
