(module
 (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
 (func $main
	i32.const 3
	i32.const 3
	i32.ne
	i32.const 0
	call $assert_eq_i32
    i32.const 3
    i32.const 4
    i32.ne
    i32.const 1
    call $assert_eq_i32)
 (export "main" (func $main)))
