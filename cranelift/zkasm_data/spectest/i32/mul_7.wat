(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i32.const 0x7fffffff
	i32.const -1
	i32.mul
	i32.const 0x80000001
	call $assert_eq_i32)
 (export "main" (func $main)))
