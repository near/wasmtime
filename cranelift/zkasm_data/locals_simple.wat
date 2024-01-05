(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	(local $x i32)
	(local.set $x (i32.const 2))
	(local.get $x)
	(i32.const 2)
	call $assert_eq_i32)
(start $main))
