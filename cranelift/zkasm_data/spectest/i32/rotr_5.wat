(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i32.const 0xff00cc00
	i32.const 1
	i32.rotr
	i32.const 0x7f806600
	call $assert_eq_i32)
 (start $main))
