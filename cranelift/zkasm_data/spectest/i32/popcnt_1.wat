(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i32.const -1
	i32.popcnt
	i32.const 32
	call $assert_eq_i32)
 (start $main))
