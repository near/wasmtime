(module
 (import "env" "assert_eq" (func $assert_eq (param i32) (param i32)))
 (func $main
	i32.const 0x8000
	i32.extend16_s
	i32.const -32768
	call $assert_eq)
 (start $main))
