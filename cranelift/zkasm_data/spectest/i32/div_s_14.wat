(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i32.const -7
	i32.const -3
	i32.div_s
	i32.const 2
	call $assert_eq_i32)
 (start $main))
