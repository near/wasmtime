(module
 (import "env" "assert_eq" (func $assert_eq (param i32) (param i32)))
 (func $main
	i32.const 0x00008000
	i32.ctz
	i32.const 15
	call $assert_eq)
 (start $main))
