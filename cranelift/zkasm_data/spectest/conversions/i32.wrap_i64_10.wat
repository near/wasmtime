(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i64.const 0x00000000ffffffff
	i32.wrap_i64
	i32.const 0xffffffff
	call $assert_eq_i32)
 (export "main" (func $main)))
