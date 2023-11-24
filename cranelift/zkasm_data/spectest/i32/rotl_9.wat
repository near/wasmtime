(module
 (import "env" "assert_eq" (func $assert_eq (param i32) (param i32)))
 (func $main
	i32.const 0xb0c1d2e3
	i32.const 0xff05
	i32.rotl
	i32.const 0x183a5c76
	call $assert_eq)
 (start $main))
