class_name Queen extends PieceInfo

func _pseudo_valid_moves(piece: Piece, board: Board) -> Array[Vector2i]:
	var res: Array[Vector2i] = []
	build_line_cross(piece, board, res)
	build_diagonal_cross(piece, board, res)
	return res
	