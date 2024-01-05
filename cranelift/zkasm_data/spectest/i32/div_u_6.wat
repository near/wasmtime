(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i32.const 0x8ff00ff0
	i32.const 0x10001
	i32.div_u
	i32.const 0x8fef
	call $assert_eq_i32)
 (start $main))
