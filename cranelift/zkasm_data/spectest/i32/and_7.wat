(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i32.const 0xf0f0ffff
	i32.const 0xfffff0f0
	i32.and
	i32.const 0xf0f0f0f0
	call $assert_eq_i32)
 (start $main))