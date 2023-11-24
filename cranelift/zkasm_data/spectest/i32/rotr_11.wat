(module
 (import "env" "assert_eq" (func $assert_eq (param i32) (param i32)))
 (func $main
	i32.const 0x769abcdf
	i32.const 0x8000000d
	i32.rotr
	i32.const 0xe6fbb4d5
	call $assert_eq)
 (start $main))
