(module
 (import "env" "assert_eq" (func $assert_eq (param i32) (param i32)))
 (func $main
	i32.const -1
	i32.ctz
	i32.const 0
	call $assert_eq)
 (start $main))
