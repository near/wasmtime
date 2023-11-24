(module
 (import "env" "assert_eq" (func $assert_eq (param i32) (param i32)))
 (func $main
	i32.const 0x10000000
	i32.const 4096
	i32.mul
	i32.const 0
	call $assert_eq)
 (start $main))
