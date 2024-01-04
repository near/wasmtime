(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i32.const 2147483648
	i32.const 2
	i32.mul
	i32.const 0
	call $assert_eq_i32)
 (start $main))
