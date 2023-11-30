(module
 (import "env" "assert_eq" (func $assert_eq (param i32) (param i32)))
 (func $main
	i64.const 0xffffffff00000000
	i32.wrap_i64
	i32.const 0x00000000
	call $assert_eq)
 (start $main))
