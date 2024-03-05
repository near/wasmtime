(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i64.const 0x0000000100000000
	i32.wrap_i64
	i32.const 0x00000000
	call $assert_eq_i32)
 (export "main" (func $main)))
