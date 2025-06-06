class_name Knight extends PieceInfo

func _valid_moves(piece: Piece, board: Board) -> Array[Vector2i]:
	var res: Array[Vector2i] = []
	build_square(piece, board, 2, 1, res)
	build_square(piece, board, 1, 2, res)
	return res
