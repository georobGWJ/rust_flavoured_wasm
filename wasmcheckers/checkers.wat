(module
    (memory $mem 1)  ;; Assign at least one 64KB page of memory to $mem
    ;; GLOBALS
    (global $WHITE i32 (i32.const 2))
    (global $BLACK i32 (i32.const 1))
    (global $CROWN i32 (i32.const 4))

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
            (i32.const 4)  ;; 4 is for 4 bytes in an i32 (32 bits)
        )
    )

    ;; Squares on the board willuse 8 bit bitmasks to store state
    ;; Binary       Decimal     State Meaning
    ;; 00000000     0           Unoccupied Square
    ;; 00000001     1           Black Piece
    ;; 00000010     2           White Piece
    ;; 00000100     4           Crowned Piece
    ;; For example, a Crowned Black Piece would be 00000101 (Decimal 5)

    ;; Determine if a piece has been Crowned
    (func $isCrowned (param $piece i32) (result i32)
        (i32.eq
            (i32.and (get_local $piece) (get_global $CROWN))
            (get_global $CROWN)
        )
    )

    ;; Determine if a piece is White
    (func $isWhite(param $piece i32) (result i32)
        (i32.eq
            (i32.and (get_local $piece) (get_global $WHITE))
            (get_global $WHITE)
        )
    )

        ;; Determine if a piece is Black
    (func $isBlack(param $piece i32) (result i32)
        (i32.eq
            (i32.and (get_local $piece) (get_global $BLACK))
            (get_global $BLACK)
        )
    )

    ;; Add a crown to a given piece (no mutation)
    (func $withCrown (param $piece i32) (result i32)
        (i32.or (get_local $piece) (get_global $CROWN))
    )

    ;; Removes a crown from a given piece (no muttion)
    (func $withoutCrown (param $piece i32) (result i32)
        (i32.and (get_local $piece) (i32.const 3))
    )

    ;; Exports
    (export "offsetForPosition" (func $offsetForPosition))
    (export "isCrowned" (func $isCrowned))
    (export "isWhite" (func $isWhite))
    (export "isBlack" (func $isBlack))
    (export "withCrown" (func $withCrown))
    (export "withoutCrown" (func $withoutCrown))

)