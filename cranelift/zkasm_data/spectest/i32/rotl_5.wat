(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i32.const 0xabcd9876
	i32.const 1
	i32.rotl
	i32.const 0x579b30ed
	call $assert_eq_i32)
 (start $main))
