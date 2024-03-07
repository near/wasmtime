(module
    (import "env" "assert_eq_i32" (func $assert_eq_i32 (param i32) (param i32)))
    (memory 1)
    (data (i32.const 0) "abcdefghijklmnopqrstuvwxyz")
    (func $main
        i32.const 0
        i32.load8_u offset=0
        i32.const 97
        call $assert_eq_i32
        i32.const 0
        i32.load8_u align=1
        i32.const 97
        call $assert_eq_i32
        i32.const 0
        i32.load8_u offset=1 align=1
        i32.const 98
        call $assert_eq_i32
        i32.const 0
        i32.load8_u offset=2 align=1
        i32.const 99
        call $assert_eq_i32
        i32.const 0
        i32.load8_u offset=25 align=1
        i32.const 122
        call $assert_eq_i32
        i32.const 0
        i32.load8_s offset=0
        i32.const 97
        call $assert_eq_i32
        i32.const 0
        i32.load8_s align=1
        i32.const 97
        call $assert_eq_i32
        i32.const 0
        i32.load8_s offset=1 align=1
        i32.const 98
        call $assert_eq_i32
        i32.const 0
        i32.load8_s offset=2 align=1
        i32.const 99
        call $assert_eq_i32
        i32.const 0
        i32.load8_s offset=25 align=1
        i32.const 122
        call $assert_eq_i32
        i32.const 0
        i32.load16_u offset=0
        i32.const 25185
        call $assert_eq_i32
        i32.const 0
        i32.load16_u align=1
        i32.const 25185
        call $assert_eq_i32
        i32.const 0
        i32.load16_u offset=1 align=1
        i32.const 25442
        call $assert_eq_i32
        i32.const 0
        i32.load16_u offset=2 align=2
        i32.const 25699
        call $assert_eq_i32
        i32.const 0
        i32.load16_u offset=25 align=2    
        i32.const 122
        call $assert_eq_i32
        i32.const 0
        i32.load16_s offset=0
        i32.const 25185
        call $assert_eq_i32
        i32.const 0
        i32.load16_s align=1
        i32.const 25185
        call $assert_eq_i32
        i32.const 0
        i32.load16_s offset=1 align=1
        i32.const 25442
        call $assert_eq_i32
        i32.const 0
        i32.load16_s offset=2 align=2
        i32.const 25699
        call $assert_eq_i32
        i32.const 0
        i32.load16_s offset=25 align=2
        i32.const 122
        call $assert_eq_i32
        i32.const 0
        i32.load offset=0
        i32.const 1684234849
        call $assert_eq_i32
        i32.const 0
        i32.load align=1
        i32.const 1684234849
        call $assert_eq_i32
        i32.const 0
        i32.load offset=1 align=1 
        i32.const 1701077858
        call $assert_eq_i32
        i32.const 0
        i32.load offset=2 align=2
        i32.const 1717920867
        call $assert_eq_i32
        i32.const 0
        i32.load offset=25 align=4       
        i32.const 122
        call $assert_eq_i32)
    (export "main" (func $main))
)