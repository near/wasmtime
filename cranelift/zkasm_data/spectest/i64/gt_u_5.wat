(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i64.const 0x7fffffffffffffff
	i64.const 0x7fffffffffffffff
	i64.gt_u
	i32.const 0
	call $assert_eq_i32)
 (export "main" (func $main)))
