class_name Pawn extends PieceInfo

func region_offset() -> Vector2i:
	return Vector2i(1.0, 0.0)

func _valid_moves(piece: Piece, _board: Board) -> PackedVector2Array:
	var deltas = PackedVector2Array([Vector2(0, 1), Vector2(0, -1)])
	for i in deltas.size():
		deltas[i] = piece.grid_position + deltas[i]

	return PackedVector2Array(deltas)
