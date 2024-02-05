(module
  (type (;0;) (func (param i64 i64)))
  (type (;1;) (func))
  (type (;2;) (func (param i32 i32)))
  (type (;3;) (func (param i32)))
  (type (;4;) (func (param i32 i32 i32)))
  (type (;5;) (func (param i32 i32 i32) (result i32)))
  (import "env" "assert_eq_i64" (func (;0;) (type 0)))
  (func (;1;) (type 1)
    (local i32 i64 i64 i64 i64)
    global.get 0
    i32.const 352
    i32.sub
    local.tee 0
    global.set 0
    local.get 0
    i32.const 8
    i32.add
    i32.const 0
    i32.const 192
    call 10
    drop
    local.get 0
    i32.const 1
    i32.store8 offset=344
    local.get 0
    i32.const 88
    i32.store8 offset=208
    local.get 0
    i32.const 24
    i32.store offset=200
    local.get 0
    i32.const 210
    i32.add
    i32.const 0
    i32.const 133
    call 10
    drop
    local.get 0
    i32.const 343
    i32.add
    i32.const 128
    i32.store8
    local.get 0
    i32.const 1
    i32.store8 offset=209
    local.get 0
    local.get 0
    i64.load offset=208
    i64.store
    local.get 0
    local.get 0
    i32.const 336
    i32.add
    i64.load
    i64.store offset=128
    local.get 0
    i32.const 24
    call 3
    local.get 0
    i64.load offset=24
    local.set 1
    local.get 0
    i64.load offset=16
    local.set 2
    local.get 0
    i64.load offset=8
    local.set 3
    i64.const 85
    local.get 0
    i64.load
    local.tee 4
    i64.const 255
    i64.and
    call 0
    i64.const 12
    local.get 4
    i64.const 8
    i64.shr_u
    i64.const 255
    i64.and
    call 0
    i64.const 100
    local.get 4
    i64.const 16
    i64.shr_u
    i64.const 255
    i64.and
    call 0
    i64.const 161
    local.get 4
    i64.const 24
    i64.shr_u
    i64.const 255
    i64.and
    call 0
    i64.const 80
    local.get 4
    i64.const 32
    i64.shr_u
    i64.const 255
    i64.and
    call 0
    i64.const 49
    local.get 4
    i64.const 40
    i64.shr_u
    i64.const 255
    i64.and
    call 0
    i64.const 195
    local.get 4
    i64.const 48
    i64.shr_u
    i64.const 255
    i64.and
    call 0
    i64.const 6
    local.get 4
    i64.const 56
    i64.shr_u
    call 0
    i64.const 68
    local.get 3
    i64.const 255
    i64.and
    call 0
    i64.const 84
    local.get 3
    i64.const 8
    i64.shr_u
    i64.const 255
    i64.and
    call 0
    i64.const 193
    local.get 3
    i64.const 16
    i64.shr_u
    i64.const 255
    i64.and
    call 0
    i64.const 154
    local.get 3
    i64.const 24
    i64.shr_u
    i64.const 255
    i64.and
    call 0
    i64.const 220
    local.get 3
    i64.const 32
    i64.shr_u
    i64.const 255
    i64.and
    call 0
    i64.const 98
    local.get 3
    i64.const 40
    i64.shr_u
    i64.const 255
    i64.and
    call 0
    i64.const 67
    local.get 3
    i64.const 48
    i64.shr_u
    i64.const 255
    i64.and
    call 0
    i64.const 166
    local.get 3
    i64.const 56
    i64.shr_u
    call 0
    i64.const 18
    local.get 2
    i64.const 255
    i64.and
    call 0
    i64.const 44
    local.get 2
    i64.const 8
    i64.shr_u
    i64.const 255
    i64.and
    call 0
    i64.const 19
    local.get 2
    i64.const 16
    i64.shr_u
    i64.const 255
    i64.and
    call 0
    i64.const 138
    local.get 2
    i64.const 24
    i64.shr_u
    i64.const 255
    i64.and
    call 0
    i64.const 36
    local.get 2
    i64.const 32
    i64.shr_u
    i64.const 255
    i64.and
    call 0
    i64.const 46
    local.get 2
    i64.const 40
    i64.shr_u
    i64.const 255
    i64.and
    call 0
    i64.const 170
    local.get 2
    i64.const 48
    i64.shr_u
    i64.const 255
    i64.and
    call 0
    i64.const 9
    local.get 2
    i64.const 56
    i64.shr_u
    call 0
    i64.const 141
    local.get 1
    i64.const 255
    i64.and
    call 0
    i64.const 165
    local.get 1
    i64.const 8
    i64.shr_u
    i64.const 255
    i64.and
    call 0
    i64.const 11
    local.get 1
    i64.const 16
    i64.shr_u
    i64.const 255
    i64.and
    call 0
    i64.const 177
    local.get 1
    i64.const 24
    i64.shr_u
    i64.const 255
    i64.and
    call 0
    i64.const 20
    local.get 1
    i64.const 32
    i64.shr_u
    i64.const 255
    i64.and
    call 0
    i64.const 252
    local.get 1
    i64.const 40
    i64.shr_u
    i64.const 255
    i64.and
    call 0
    i64.const 141
    local.get 1
    i64.const 48
    i64.shr_u
    i64.const 255
    i64.and
    call 0
    i64.const 86
    local.get 1
    i64.const 56
    i64.shr_u
    call 0
    local.get 0
    i32.const 352
    i32.add
    global.set 0)
  (func (;2;) (type 2) (param i32 i32)
    (local i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64 i64)
    block  ;; label = @1
      local.get 1
      i32.const 24
      i32.gt_u
      br_if 0 (;@1;)
      block  ;; label = @2
        i32.const 24
        local.get 1
        i32.sub
        i32.const 3
        i32.shl
        i32.const 2048
        i32.add
        i32.const 2240
        i32.eq
        br_if 0 (;@2;)
        i32.const 0
        local.get 1
        i32.const 3
        i32.shl
        i32.sub
        local.set 1
        local.get 0
        i64.load offset=192
        local.set 2
        local.get 0
        i64.load offset=152
        local.set 3
        local.get 0
        i64.load offset=112
        local.set 4
        local.get 0
        i64.load offset=72
        local.set 5
        local.get 0
        i64.load offset=32
        local.set 6
        local.get 0
        i64.load offset=184
        local.set 7
        local.get 0
        i64.load offset=144
        local.set 8
        local.get 0
        i64.load offset=104
        local.set 9
        local.get 0
        i64.load offset=64
        local.set 10
        local.get 0
        i64.load offset=24
        local.set 11
        local.get 0
        i64.load offset=176
        local.set 12
        local.get 0
        i64.load offset=136
        local.set 13
        local.get 0
        i64.load offset=96
        local.set 14
        local.get 0
        i64.load offset=56
        local.set 15
        local.get 0
        i64.load offset=16
        local.set 16
        local.get 0
        i64.load offset=168
        local.set 17
        local.get 0
        i64.load offset=128
        local.set 18
        local.get 0
        i64.load offset=88
        local.set 19
        local.get 0
        i64.load offset=48
        local.set 20
        local.get 0
        i64.load offset=8
        local.set 21
        local.get 0
        i64.load offset=160
        local.set 22
        local.get 0
        i64.load offset=120
        local.set 23
        local.get 0
        i64.load offset=80
        local.set 24
        local.get 0
        i64.load offset=40
        local.set 25
        local.get 0
        i64.load
        local.set 26
        loop  ;; label = @3
          local.get 23
          local.get 22
          i64.xor
          local.get 24
          i64.xor
          local.get 25
          i64.xor
          local.get 26
          i64.xor
          local.tee 27
          local.get 13
          local.get 12
          i64.xor
          local.get 14
          i64.xor
          local.get 15
          i64.xor
          local.get 16
          i64.xor
          local.tee 28
          i64.const 1
          i64.rotl
          i64.xor
          local.tee 29
          local.get 20
          i64.xor
          local.set 30
          local.get 27
          i64.const 1
          i64.rotl
          local.get 8
          local.get 7
          i64.xor
          local.get 9
          i64.xor
          local.get 10
          i64.xor
          local.get 11
          i64.xor
          local.tee 31
          i64.xor
          local.tee 27
          local.get 2
          i64.xor
          local.set 32
          local.get 29
          local.get 17
          i64.xor
          i64.const 2
          i64.rotl
          local.tee 33
          local.get 28
          local.get 3
          local.get 2
          i64.xor
          local.get 4
          i64.xor
          local.get 5
          i64.xor
          local.get 6
          i64.xor
          local.tee 34
          i64.const 1
          i64.rotl
          i64.xor
          local.tee 28
          local.get 10
          i64.xor
          i64.const 55
          i64.rotl
          local.tee 35
          local.get 18
          local.get 17
          i64.xor
          local.get 19
          i64.xor
          local.get 20
          i64.xor
          local.get 21
          i64.xor
          local.tee 10
          local.get 31
          i64.const 1
          i64.rotl
          i64.xor
          local.tee 31
          local.get 16
          i64.xor
          i64.const 62
          i64.rotl
          local.tee 36
          i64.const -1
          i64.xor
          i64.and
          i64.xor
          local.set 2
          local.get 10
          i64.const 1
          i64.rotl
          local.get 34
          i64.xor
          local.tee 16
          local.get 23
          i64.xor
          i64.const 41
          i64.rotl
          local.tee 34
          local.get 27
          local.get 4
          i64.xor
          i64.const 39
          i64.rotl
          local.tee 37
          i64.const -1
          i64.xor
          i64.and
          local.get 35
          i64.xor
          local.set 17
          local.get 29
          local.get 19
          i64.xor
          i64.const 10
          i64.rotl
          local.tee 38
          local.get 28
          local.get 7
          i64.xor
          i64.const 56
          i64.rotl
          local.tee 39
          local.get 31
          local.get 13
          i64.xor
          i64.const 15
          i64.rotl
          local.tee 40
          i64.const -1
          i64.xor
          i64.and
          i64.xor
          local.set 13
          local.get 38
          local.get 16
          local.get 25
          i64.xor
          i64.const 36
          i64.rotl
          local.tee 41
          i64.const -1
          i64.xor
          i64.and
          local.get 27
          local.get 6
          i64.xor
          i64.const 27
          i64.rotl
          local.tee 42
          i64.xor
          local.set 23
          local.get 31
          local.get 15
          i64.xor
          i64.const 6
          i64.rotl
          local.tee 43
          local.get 29
          local.get 21
          i64.xor
          i64.const 1
          i64.rotl
          local.tee 44
          i64.const -1
          i64.xor
          i64.and
          local.get 16
          local.get 22
          i64.xor
          i64.const 18
          i64.rotl
          local.tee 22
          i64.xor
          local.set 4
          local.get 27
          local.get 3
          i64.xor
          i64.const 8
          i64.rotl
          local.tee 45
          local.get 28
          local.get 9
          i64.xor
          i64.const 25
          i64.rotl
          local.tee 46
          i64.const -1
          i64.xor
          i64.and
          local.get 43
          i64.xor
          local.set 19
          local.get 27
          local.get 5
          i64.xor
          i64.const 20
          i64.rotl
          local.tee 27
          local.get 28
          local.get 11
          i64.xor
          i64.const 28
          i64.rotl
          local.tee 11
          i64.const -1
          i64.xor
          i64.and
          local.get 31
          local.get 12
          i64.xor
          i64.const 61
          i64.rotl
          local.tee 15
          i64.xor
          local.set 5
          local.get 29
          local.get 18
          i64.xor
          i64.const 45
          i64.rotl
          local.tee 29
          local.get 11
          local.get 15
          i64.const -1
          i64.xor
          i64.and
          i64.xor
          local.set 10
          local.get 15
          local.get 29
          i64.const -1
          i64.xor
          i64.and
          local.get 16
          local.get 24
          i64.xor
          i64.const 3
          i64.rotl
          local.tee 21
          i64.xor
          local.set 15
          local.get 29
          local.get 21
          i64.const -1
          i64.xor
          i64.and
          local.get 27
          i64.xor
          local.set 20
          local.get 21
          local.get 27
          i64.const -1
          i64.xor
          i64.and
          local.get 11
          i64.xor
          local.set 25
          local.get 16
          local.get 26
          i64.xor
          local.tee 29
          local.get 32
          i64.const 14
          i64.rotl
          local.tee 27
          i64.const -1
          i64.xor
          i64.and
          local.get 28
          local.get 8
          i64.xor
          i64.const 21
          i64.rotl
          local.tee 28
          i64.xor
          local.set 11
          local.get 27
          local.get 28
          i64.const -1
          i64.xor
          i64.and
          local.get 31
          local.get 14
          i64.xor
          i64.const 43
          i64.rotl
          local.tee 31
          i64.xor
          local.set 16
          local.get 30
          i64.const 44
          i64.rotl
          local.tee 6
          local.get 28
          local.get 31
          i64.const -1
          i64.xor
          i64.and
          i64.xor
          local.set 21
          local.get 1
          i32.const 2240
          i32.add
          i64.load
          local.get 31
          local.get 6
          i64.const -1
          i64.xor
          i64.and
          i64.xor
          local.get 29
          i64.xor
          local.set 26
          local.get 41
          local.get 42
          i64.const -1
          i64.xor
          i64.and
          local.get 39
          i64.xor
          local.tee 28
          local.set 3
          local.get 6
          local.get 29
          i64.const -1
          i64.xor
          i64.and
          local.get 27
          i64.xor
          local.tee 29
          local.set 6
          local.get 36
          local.get 33
          i64.const -1
          i64.xor
          i64.and
          local.get 34
          i64.xor
          local.tee 27
          local.set 7
          local.get 42
          local.get 39
          i64.const -1
          i64.xor
          i64.and
          local.get 40
          i64.xor
          local.tee 31
          local.set 8
          local.get 44
          local.get 22
          i64.const -1
          i64.xor
          i64.and
          local.get 45
          i64.xor
          local.tee 39
          local.set 9
          local.get 33
          local.get 34
          i64.const -1
          i64.xor
          i64.and
          local.get 37
          i64.xor
          local.tee 33
          local.set 12
          local.get 22
          local.get 45
          i64.const -1
          i64.xor
          i64.and
          local.get 46
          i64.xor
          local.tee 34
          local.set 14
          local.get 40
          local.get 38
          i64.const -1
          i64.xor
          i64.and
          local.get 41
          i64.xor
          local.tee 38
          local.set 18
          local.get 37
          local.get 35
          i64.const -1
          i64.xor
          i64.and
          local.get 36
          i64.xor
          local.tee 35
          local.set 22
          local.get 44
          local.get 46
          local.get 43
          i64.const -1
          i64.xor
          i64.and
          i64.xor
          local.tee 36
          local.set 24
          local.get 1
          i32.const 8
          i32.add
          local.tee 1
          br_if 0 (;@3;)
        end
        local.get 0
        local.get 35
        i64.store offset=160
        local.get 0
        local.get 23
        i64.store offset=120
        local.get 0
        local.get 36
        i64.store offset=80
        local.get 0
        local.get 25
        i64.store offset=40
        local.get 0
        local.get 17
        i64.store offset=168
        local.get 0
        local.get 38
        i64.store offset=128
        local.get 0
        local.get 19
        i64.store offset=88
        local.get 0
        local.get 20
        i64.store offset=48
        local.get 0
        local.get 21
        i64.store offset=8
        local.get 0
        local.get 33
        i64.store offset=176
        local.get 0
        local.get 13
        i64.store offset=136
        local.get 0
        local.get 34
        i64.store offset=96
        local.get 0
        local.get 15
        i64.store offset=56
        local.get 0
        local.get 16
        i64.store offset=16
        local.get 0
        local.get 27
        i64.store offset=184
        local.get 0
        local.get 31
        i64.store offset=144
        local.get 0
        local.get 39
        i64.store offset=104
        local.get 0
        local.get 10
        i64.store offset=64
        local.get 0
        local.get 11
        i64.store offset=24
        local.get 0
        local.get 2
        i64.store offset=192
        local.get 0
        local.get 28
        i64.store offset=152
        local.get 0
        local.get 4
        i64.store offset=112
        local.get 0
        local.get 5
        i64.store offset=72
        local.get 0
        local.get 29
        i64.store offset=32
        local.get 0
        local.get 26
        i64.store
      end
      return
    end
    i32.const 2331
    i32.const 65
    i32.const 2396
    call 7
    unreachable)
  (func (;3;) (type 2) (param i32 i32)
    local.get 0
    local.get 1
    call 2)
  (func (;4;) (type 3) (param i32)
    loop  ;; label = @1
      br 0 (;@1;)
    end)
  (func (;5;) (type 3) (param i32))
  (func (;6;) (type 2) (param i32 i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 2
    global.set 0
    local.get 2
    i32.const 1
    i32.store16 offset=28
    local.get 2
    local.get 1
    i32.store offset=24
    local.get 2
    local.get 0
    i32.store offset=20
    local.get 2
    i32.const 2412
    i32.store offset=16
    local.get 2
    i32.const 2412
    i32.store offset=12
    local.get 2
    i32.const 12
    i32.add
    call 4
    unreachable)
  (func (;7;) (type 4) (param i32 i32 i32)
    (local i32)
    global.get 0
    i32.const 32
    i32.sub
    local.tee 3
    global.set 0
    local.get 3
    i32.const 12
    i32.add
    i64.const 0
    i64.store align=4
    local.get 3
    i32.const 1
    i32.store offset=4
    local.get 3
    i32.const 2412
    i32.store offset=8
    local.get 3
    local.get 1
    i32.store offset=28
    local.get 3
    local.get 0
    i32.store offset=24
    local.get 3
    local.get 3
    i32.const 24
    i32.add
    i32.store
    local.get 3
    local.get 2
    call 6
    unreachable)
  (func (;8;) (type 2) (param i32 i32)
    local.get 0
    i64.const -3777529136054271931
    i64.store offset=8
    local.get 0
    i64.const 2295361781758797333
    i64.store)
  (func (;9;) (type 5) (param i32 i32 i32) (result i32)
    (local i32 i32 i32)
    block  ;; label = @1
      block  ;; label = @2
        local.get 2
        i32.const 16
        i32.ge_u
        br_if 0 (;@2;)
        local.get 0
        local.set 3
        br 1 (;@1;)
      end
      local.get 0
      i32.const 0
      local.get 0
      i32.sub
      i32.const 3
      i32.and
      local.tee 4
      i32.add
      local.set 5
      block  ;; label = @2
        local.get 4
        i32.eqz
        br_if 0 (;@2;)
        local.get 0
        local.set 3
        loop  ;; label = @3
          local.get 3
          local.get 1
          i32.store8
          local.get 3
          i32.const 1
          i32.add
          local.tee 3
          local.get 5
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 5
      local.get 2
      local.get 4
      i32.sub
      local.tee 4
      i32.const -4
      i32.and
      local.tee 2
      i32.add
      local.set 3
      block  ;; label = @2
        local.get 2
        i32.const 1
        i32.lt_s
        br_if 0 (;@2;)
        local.get 1
        i32.const 255
        i32.and
        i32.const 16843009
        i32.mul
        local.set 2
        loop  ;; label = @3
          local.get 5
          local.get 2
          i32.store
          local.get 5
          i32.const 4
          i32.add
          local.tee 5
          local.get 3
          i32.lt_u
          br_if 0 (;@3;)
        end
      end
      local.get 4
      i32.const 3
      i32.and
      local.set 2
    end
    block  ;; label = @1
      local.get 2
      i32.eqz
      br_if 0 (;@1;)
      local.get 3
      local.get 2
      i32.add
      local.set 5
      loop  ;; label = @2
        local.get 3
        local.get 1
        i32.store8
        local.get 3
        i32.const 1
        i32.add
        local.tee 3
        local.get 5
        i32.lt_u
        br_if 0 (;@2;)
      end
    end
    local.get 0)
  (func (;10;) (type 5) (param i32 i32 i32) (result i32)
    local.get 0
    local.get 1
    local.get 2
    call 9)
	(start 1)
  (table (;0;) 3 3 funcref)
  (memory (;0;) 1)
  (global (;0;) (mut i32) (i32.const 2048))
  (global (;1;) i32 (i32.const 2428))
  (global (;2;) i32 (i32.const 2432))
  (export "memory" (memory 0))
  (export "main" (func 1))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (elem (;0;) (i32.const 1) func 5 8)
  (data (;0;) (i32.const 2048) "\01\00\00\00\00\00\00\00\82\80\00\00\00\00\00\00\8a\80\00\00\00\00\00\80\00\80\00\80\00\00\00\80\8b\80\00\00\00\00\00\00\01\00\00\80\00\00\00\00\81\80\00\80\00\00\00\80\09\80\00\00\00\00\00\80\8a\00\00\00\00\00\00\00\88\00\00\00\00\00\00\00\09\80\00\80\00\00\00\00\0a\00\00\80\00\00\00\00\8b\80\00\80\00\00\00\00\8b\00\00\00\00\00\00\80\89\80\00\00\00\00\00\80\03\80\00\00\00\00\00\80\02\80\00\00\00\00\00\80\80\00\00\00\00\00\00\80\0a\80\00\00\00\00\00\00\0a\00\00\80\00\00\00\80\81\80\00\80\00\00\00\80\80\80\00\00\00\00\00\80\01\00\00\80\00\00\00\00\08\80\00\80\00\00\00\80/Users/akashin/.cargo/registry/src/index.crates.io-6f17d22bba15001f/keccak-0.1.5/src/lib.rsA round_count greater than KECCAK_F_ROUND_COUNT is not supported!\c0\08\00\00[\00\00\00\ee\00\00\00\09\00\00\00\01\00\00\00\00\00\00\00\01\00\00\00\02\00\00\00"))
