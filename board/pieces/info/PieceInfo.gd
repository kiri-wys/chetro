class_name PieceInfo extends Resource

@export var sprite: Texture2D
var region_offset: Vector2i = Vector2i(0, 0)

func region() -> Rect2:
	var coords = region_offset
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
		var mappings = FileAccess.get_file_as_string("res://assets/pieces.json")
		var json_mappings = JSON.parse_string(mappings)
		if json_mappings != null:
			if json_mappings.has(name):
				var entry = json_mappings[name]
				piece.region_offset.x = entry["x"]
				piece.region_offset.y = entry["y"]
			else:
				print("WARNING: %s requested but mappings to the atlas not found" % name)
		return piece
	else:
		print("WARNING: %s requested but known mapping not found, using fallback" % name)
		return PieceInfo.new()

func _valid_moves(_piece: Piece, _board: Board) -> PackedVector2Array:
	return PackedVector2Array([])
