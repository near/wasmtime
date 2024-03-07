(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (global $a i32 (i32.const -2))
 (global $b (mut i32) (i32.const 5))
 (func $main
	(global.set $b (i32.const 3))
	(global.get $a)
	(global.get $b)
	i32.add
	i32.const 1
	call $assert_eq_i32)
 (export "main" (func $main)))
