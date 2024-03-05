(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i64.const 0
	i32.wrap_i64
	i32.const 0
	call $assert_eq_i32)
 (export "main" (func $main)))
