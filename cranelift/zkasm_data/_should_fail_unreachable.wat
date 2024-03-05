(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	unreachable
	call $assert_eq_i32)
 (export "main" (func $main)))
