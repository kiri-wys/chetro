class_name Bishop extends PieceInfo


func _pseudo_valid_moves(piece: Piece, board: Board) -> Array[Vector2i]:
	var res: Array[Vector2i] = []
	build_diagonal_cross(piece, board, res)
	return res
