(module
 (import "env" "assert_eq" (func $assert_eq (param i32) (param i32)))
 (func $main
	i32.const 999999999
	i32.const 1000000000
	i32.add
	i32.const 1999999999
	call $assert_eq)
 (start $main))
