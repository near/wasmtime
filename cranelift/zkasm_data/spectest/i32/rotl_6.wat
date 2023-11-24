(module
 (import "env" "assert_eq" (func $assert_eq (param i32) (param i32)))
 (func $main
	i32.const 0xfe00dc00
	i32.const 4
	i32.rotl
	i32.const 0xe00dc00f
	call $assert_eq)
 (start $main))
