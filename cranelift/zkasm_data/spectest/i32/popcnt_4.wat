(module
 (import "env" "assert_eq" (func $assert_eq (param i32) (param i32)))
 (func $main
	i32.const 0x80008000
	i32.popcnt
	i32.const 2
	call $assert_eq)
 (start $main))
