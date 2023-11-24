(module
 (import "env" "assert_eq" (func $assert_eq (param i32) (param i32)))
 (func $main
	i32.const 0x80000000
	i32.const -1
	i32.gt_u
	i32.const 0
	call $assert_eq)
 (start $main))
