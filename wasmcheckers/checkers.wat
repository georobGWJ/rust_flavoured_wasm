(module
    (memory $mem 1)  ;; Assign at least one 64KB page of memory to $mem
    ;; GLOBALS
    (global $WHITE i32 (i32.const 2)) ;; bitmask for white 00000010
    (global $BLACK i32 (i32.const 1)) ;; bitmask for white 00000001
    (global $CROWN i32 (i32.const 4)) ;; bitmask for white 00000100
    ;; mut makes the Global var below mutable
    (global $currentTurn (mut i32) (i32.const 0))  ;; 1 = black turn, 2 = white turn

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

    ;; Set a piece on the board
    (func $setPiece (param $x i32) (param $y i32) (param $piece i32)
        (i32.store  ;; i32.store stores an i32 in a memory address
            (call $offsetForPosition
                (get_local $x)
                (get_local $y)
            )
            (get_local $piece)
        )
    )

    ;; Gets a piece from the board.
    ;; Out of range causes a trap
    (func $getPiece (param $x i32) (param $y i32) (result i32)
        (if (result i32)
            (block (result i32)
                (i32.and
                    (call $inRange
                        (i32.const 0)
                        (i32.const 7)
                        (get_local $x)
                    )
                    (call $inRange
                        (i32.const 0)
                        (i32.const 7)
                        (get_local $y)
                    )
                )
            )
        (then
            (i32.load
                (call $offsetForPosition
                    (get_local $x)
                    (get_local $y)
                )
            )
        )
        (else
            (unreachable)
        )
        )
    )

    ;; Detect if values are within range (inclusive high and low)
    ;; wasm has no built in protections from trying to access out-of-bounds memory
    (func $inRange (param $low i32) (param $high i32) (param $value i32) (result i32)
        (i32.and
            (i32.ge_s (get_local $value) (get_local $low))
            (i32.le_s (get_local $value) (get_local $high))
        )
    )

    ;; Get the current turn owner
    (func $getTurnOwner (result i32)
        (get_global $currentTurn)
    )

    ;; At the end of a turn, switch turn owner to other player
    (func $toggleTurnOwner
        (if (i32.eq (call $getTurnOwner) (i32.const 1))
            (then (call $setTurnOwner (i32.const 2)))
            (else (call $setTurnOwner (i32.const 1)))
        )
    )

    ;; Set the current turn owner
    (func $setTurnOwner (param $piece i32)
        (set_global $currentTurn (get_local $piece))
    )

    ;; Check if it's a player's turn
    (func $isPlayersTurn (param $player i32) (result i32)
        (i32.gt_s
            (i32.and (get_local $player) (call $getTurnOwner))
            (i32.const 0)
        )
    )

    ;; Should this piece get crowned?
    ;; Black pieces are crowned on tow 0, white pieces on row 7
    (func $shouldCrown (param $pieceY i32) (param $piece i32) (result i32)
        (i32.or(i32.and
            (i32.eq
                (get_local $pieceY)
                (i32.const 0)
            )
            (call $isBlack (get_local $piece))
        )

        (i32.and
            (i32.eq
                (get_local $pieceY)
                (i32.const 7)
            )
            (call $isWhite (get_local $piece))
        )
        )
    )

    ;; Convert a piece into a crowned piece and invoke a host notifier
    (func $crownPiece (param $x i32) (param $y i32) 
        (local $piece i32)  ;; locally scoped var that goes away when function exits
        (set_local $piece (call $getPiece (get_local $x) (get_local $y)))
        (call $setPiece (get_local $x) (get_local $y)
            (call $withCrown (get_local $piece)))
        ;; notify_piececrowned function will be imported by the checkers
        ;; module but implemented by the host
        (call $notify_piececrowned (get_local $x) (get_local $y))
    )

    ;; Distance function
    (func $distance (param $x i32) (param $y i32) (result i32)
        (i32.sub (get_local $x) (get_local $y))
    )

    ;; Determine if a move is valid by chesking that:
    ;;    * The "moce distance" from current pos to target pos is valid
    ;;    * The target space is Unoccupied
    ;;    * The piece being moved belongs to the current player
    (func $isValidMove (param $fromX i32) (param $fromY i32)
                       (param $toX i32) (param $toY i32) (result i32)
        (local $player i32)
        (local $target i32)

        (set_local $player (call $getPiece (get_local $fromX) (get_local $fromY)))
        (set_local $target (call $getPiece (get_local $toX) (get_local $toY)))

        (if (result i32)
            (block (result i32)
                (i32.and
                    (call $validJumpDistance (get_local $fromY) (get_local $toY))
                    (i32.and
                        (call $isPlayersTurn (get_local $player))
                        ;; Target must be unoccupied
                        (i32.eq (get_local $target) (i32.const 0))
                    )
                )
            )
            (then
                (i32.const 1)
            )
            (else
                (i32.const 0)
            )
        )
    )

    ;; Ensures reavel is 1 or 2 Squares
    (func $validJumpDistance (param $from i32) (param $to i32) (result i32)
        (local $d i32)
        (set_local $d
            (if (result i32)
                (i32.gt_s (get_local $to) (get_local $from))
                (then
                    (call $distance (get_local $to) (get_local $from))
                )
                (else
                    (call $distance (get_local $from) (get_local $to))
                )
            )
        )
        (i32.le_u
            (get_local $d)
            (i32.const 2)
        )
    )

    ;; Exported move function to be called by the game host
    (func $move (param $fromX i32) (param $fromY i32)
                (param $toX i32) (param $toY i32) (result i32)

        (if (result i32)
            (block (result i32)
                (call $isValidMove (get_local $fromX) (get_local $fromY)
                                   (get_local $toX)   (get_local $toY)
                )
            )
            (then
                (call $do_move (get_local $fromX) (get_local $fromY)
                               (get_local $toX)   (get_local $toY))
            )
            (else
                (i32.const 0)
            )
        )
    )

    ;; Internal move helper function. This performs actual move post validation of target.
    ;; Currently not handled:
    ;;    * removing opponent piece during a jump
    ;;    * detecting a Win condition
    (func $do_move (param $fromX i32) (param $fromY i32)
                   (param $toX i32) (param $toY i32) (result i32)
        (local $curpiece i32)
        (set_local $curpiece (call $getPiece (get_local $fromX) (get_local $fromY)))

        (call $toggleTurnOwner)
        (call $setPiece (get_local $toX) (get_local $toY) (get_local $curpiece))
        (call $setPiece (get_local $fromX) (get_local $fromY) (i32.const 0))

        (if (call $shouldCrown (get_local $toY) (get_local $curpiece))
            (then (call $crownPiece (get_local $toX) (get_local $toY)))
        )
        (call $notify_piecemoved (get_local $fromX) (get_local $fromY)
                                 (get_local $toX)   (get_local $toY)
        )
        (i32.const 1)
    )

    ;; Board initialization for testing purposes
    ;; Manually place each piece
    (func $initBoard
        ;; Place White pieces at the top of the board
        (call $setPiece (i32.const 1) (i32.const 0) (i32.const 2))
        (call $setPiece (i32.const 3) (i32.const 0) (i32.const 2))
        (call $setPiece (i32.const 5) (i32.const 0) (i32.const 2))
        (call $setPiece (i32.const 7) (i32.const 0) (i32.const 2))

        (call $setPiece (i32.const 0) (i32.const 1) (i32.const 2))
        (call $setPiece (i32.const 2) (i32.const 1) (i32.const 2))
        (call $setPiece (i32.const 4) (i32.const 1) (i32.const 2))
        (call $setPiece (i32.const 6) (i32.const 1) (i32.const 2))

        (call $setPiece (i32.const 1) (i32.const 2) (i32.const 2))
        (call $setPiece (i32.const 3) (i32.const 2) (i32.const 2))
        (call $setPiece (i32.const 5) (i32.const 2) (i32.const 2))
        (call $setPiece (i32.const 7) (i32.const 2) (i32.const 2))

        ;; Place Black pieces at the top of the board
        (call $setPiece (i32.const 1) (i32.const 5) (i32.const 1))
        (call $setPiece (i32.const 3) (i32.const 5) (i32.const 1))
        (call $setPiece (i32.const 5) (i32.const 5) (i32.const 1))
        (call $setPiece (i32.const 7) (i32.const 5) (i32.const 1))

        (call $setPiece (i32.const 0) (i32.const 6) (i32.const 1))
        (call $setPiece (i32.const 2) (i32.const 6) (i32.const 1))
        (call $setPiece (i32.const 4) (i32.const 6) (i32.const 1))
        (call $setPiece (i32.const 6) (i32.const 6) (i32.const 1))

        (call $setPiece (i32.const 1) (i32.const 7) (i32.const 1))
        (call $setPiece (i32.const 3) (i32.const 7) (i32.const 1))
        (call $setPiece (i32.const 5) (i32.const 7) (i32.const 1))
        (call $setPiece (i32.const 7) (i32.const 7) (i32.const 1))

        (call $setTurnOwner (i32.const 1))  ;; Black goes first
    )


    ;; Exports
    (export "offsetForPosition" (func $offsetForPosition))
    (export "isCrowned" (func $isCrowned))
    (export "isWhite" (func $isWhite))
    (export "isBlack" (func $isBlack))
    (export "withCrown" (func $withCrown))
    (export "withoutCrown" (func $withoutCrown))

)