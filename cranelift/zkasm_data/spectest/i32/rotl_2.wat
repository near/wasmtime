(module
 (import "env" "assert_eq" (func $assert_eq (param i32) (param i32)))
 (func $main
	i32.const 1
	i32.const 0
	i32.rotl
	i32.const 1
	call $assert_eq)
 (start $main))
