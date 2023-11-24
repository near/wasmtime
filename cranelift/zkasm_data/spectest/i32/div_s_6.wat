(module
 (import "env" "assert_eq" (func $assert_eq (param i32) (param i32)))
 (func $main
	i32.const 0x80000001
	i32.const 1000
	i32.div_s
	i32.const 0xffdf3b65
	call $assert_eq)
 (start $main))
