(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i32.const 0x80000000
	i32.const 2
	i32.div_s
	i32.const 0xc0000000
	call $assert_eq_i32)
 (export "main" (func $main)))
