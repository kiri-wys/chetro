class_name Rook extends PieceInfo 


func _valid_moves(piece: Piece, board: Board) -> Array[Vector2i]:
	var res: Array[Vector2i] = []
	build_line_cross(piece, board, res)
	return res
