(module
 (import "env" "assert_eq" (func $assert_eq (param i32) (param i32)))
 (func $main
	i32.const 0x80000000
	i32.const 0x7fffffff
	i32.ge_s
	i32.const 0
	call $assert_eq)
 (start $main))
