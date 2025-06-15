class_name Pawn extends PieceInfo

func _pseudo_valid_moves(piece: Piece, board: Board) -> Array[Vector2i]:
	var res: Array[Vector2i] = []
	var f = piece.grid_position + piece.front
	var s: Board.Square = board.squares_state.get(f, Board.Square.new(board))
	if s.piece == null:
		if board.in_bounds(f):
			res.append(f)

	for d in [Vector2i(1, piece.front.y), Vector2i(-1, piece.front.y)]:
		f = piece.grid_position + d
		s = board.squares_state.get(f, Board.Square.new(board))
		var is_free = s.piece == null
		var is_same_color = s.piece != null and s.piece.color == piece.color
		s.register_attack(piece)
		board.squares_state[f] = s
		if !is_free and !is_same_color and board.in_bounds(f):
			res.append(f)

	return res
