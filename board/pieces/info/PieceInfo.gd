class_name PieceInfo extends Resource

@export var sprite: Texture2D

func region_offset() -> Vector2i:
	return Vector2i(0.0, 0.0)

func region() -> Rect2:
	var coords = region_offset()
	return Rect2(128.0 * coords.x, 128.0 * coords.y, 128.0, 128.0)

static func load_or_fallback(name: String) -> PieceInfo:
	var path = "res://board/pieces/info/%s.gd" % name
	var piece: PieceInfo = null

	# Best effort to avoid exceptions
	if FileAccess.file_exists(path):
		var fetched = ResourceLoader.load(path)
		# TOCTOU
		if fetched != null:
			var con = fetched.new()
			if con is PieceInfo:
				piece = con

	if piece != null:
		return piece
	else:
		print("WARNING: %s requested but known mapping not found, using fallback" % name)
		return PieceInfo.new()

func _valid_moves(_piece: Piece, _board: Board) -> PackedVector2Array:
	return PackedVector2Array([])
