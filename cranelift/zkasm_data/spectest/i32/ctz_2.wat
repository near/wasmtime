(module
 (import "env" "assert_eq" (func $assert_eq (param i32) (param i32)))
 (func $main
	i32.const 0
	i32.ctz
	i32.const 32
	call $assert_eq)
 (start $main))
