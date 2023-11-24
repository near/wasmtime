(module
 (import "env" "assert_eq" (func $assert_eq (param i32) (param i32)))
 (func $main
	i32.const 0x769abcdf
	i32.const 0xffffffed
	i32.rotl
	i32.const 0x579beed3
	call $assert_eq)
 (start $main))
