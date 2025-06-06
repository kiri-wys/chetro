class_name Pawn extends PieceInfo

func _valid_moves(piece: Piece, board: Board) -> Array[Vector2i]:
	var res: Array[Vector2i] = []
	var f = piece.grid_position + piece.front
	if board.squares_state.get(f, Board.SquareState.FREE) == Board.SquareState.FREE:
		if board.in_bounds(f):
			res.append(f)
	for d in [Vector2i(1, piece.front.y), Vector2i(-1, piece.front.y)]:
		f = piece.grid_position + d
		var s = board.squares_state.get(f, Board.SquareState.FREE)
		var is_free = s == Board.SquareState.FREE
		var is_same_color = s == 1 + piece.color as Board.SquareState
		if !is_free and !is_same_color and board.in_bounds(f): res.append(f)

	return res
