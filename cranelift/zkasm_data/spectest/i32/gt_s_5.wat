(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i32.const 0x7fffffff
	i32.const 0x7fffffff
	i32.gt_s
	i32.const 0
	call $assert_eq_i32)
 (start $main))