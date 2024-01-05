(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i64.const -1
	i32.wrap_i64
	i32.const -1
	call $assert_eq_i32)
 (start $main))
