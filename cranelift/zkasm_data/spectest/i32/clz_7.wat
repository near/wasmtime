(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i32.const 2
	i32.clz
	i32.const 30
	call $assert_eq_i32)
 (export "main" (func $main)))
