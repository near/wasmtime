(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i32.const 0
	i32.const 0
	i32.ge_u
	i32.const 1
	call $assert_eq_i32)
 (start $main))
