(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i32.const 0x01234567
	i32.const 0x76543210
	i32.mul
	i32.const 0x358e7470
	call $assert_eq_i32)
 (start $main))
