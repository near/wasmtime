(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i32.const 1
	i32.const 33
	i32.shl
	i32.const 2
	call $assert_eq_i32)
 (start $main))
