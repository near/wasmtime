(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i32.const 0x00010000
	i32.ctz
	i32.const 16
	call $assert_eq_i32)
 (start $main))
