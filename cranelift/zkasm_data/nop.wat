(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i32.const 0
	nop
    nop
    nop
	i32.const 0
	call $assert_eq_i32)
 (start $main))
