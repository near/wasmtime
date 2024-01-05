(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i32.const 0x769abcdf
	i32.const 0x8000000d
	i32.rotl
	i32.const 0x579beed3
	call $assert_eq_i32)
 (start $main))
