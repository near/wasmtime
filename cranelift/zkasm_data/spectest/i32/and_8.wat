(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i32.const 0xffffffff
	i32.const 0xffffffff
	i32.and
	i32.const 0xffffffff
	call $assert_eq_i32)
 (export "main" (func $main)))
