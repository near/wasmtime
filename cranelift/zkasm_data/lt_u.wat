;; 1 - true, 0 false
(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i32.const 2
	i32.const 3
	i32.lt_u
	i32.const 1
	call $assert_eq_i32
	
	i32.const 3
	i32.const 2
	i32.lt_u
	i32.const 0
	call $assert_eq_i32)
 (export "main" (func $main)))
