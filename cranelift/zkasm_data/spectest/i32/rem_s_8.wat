(module
 (import "env" "assert_eq" (func $assert_eq (param i32) (param i32)))
 (func $main
	i32.const 0x80000001
	i32.const 1000
	i32.rem_s
	i32.const -647
	call $assert_eq)
 (start $main))
