(module
 (import "env" "assert_eq" (func $assert_eq (param i32) (param i32)))
 (func $main
	i32.const 2
	i32.clz
	i32.const 30
	call $assert_eq)
 (start $main))
