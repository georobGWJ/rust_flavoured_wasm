(module
    (memory $mem 1)  ;; Assign at least one 64KB page of memory to $mem

    ;; A function for indexing an 8x8 grid using linear memory
    (func $indexForPosition (param $x i32) (param $y i32) (result i32)
        (i32.add
            (i32.mul 
                (i32.const 8) ;; Index width of one row
                (get_local $y)
            )
            (get_local $x)
        )
    )

    ;; Offset = (x + y * 8) * 4
    ;;   The maths here boil down to:
    ;;      offsetForPosition(1,2)
    ;;      = (1 + 2 * 8) * 4
    ;;      = 68
    (func $offsetForPosition (param $x i32) (param $y i32) (result i32)
        (i32.mul
            (call $indexForPosition (get_local $x) (get_local $y))
            (i32.const 4)
        )
    )
)