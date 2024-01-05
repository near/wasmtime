(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i32.const 0xff
	i32.extend8_s
	i32.const -1
	call $assert_eq_i32)
 (start $main))
