(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i32.const 0xfedcba_80
	i32.extend8_s
	i32.const -0x80
	call $assert_eq_i32)
 (export "main" (func $main)))
