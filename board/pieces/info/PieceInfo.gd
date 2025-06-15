class_name PieceInfo extends Resource

var region_offset: Vector2i = Vector2i(0, 0)

func region() -> Rect2:
	var coords = region_offset
	return Rect2(Globals.square_size * coords.x, Globals.square_size * coords.y, Globals.square_size, Globals.square_size)

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

func _pseudo_valid_moves(_piece: Piece, _board: Board) -> Array[Vector2i]:
	return []


# Builds a line in the direction of the vector away from the piece
# until it hits a square that is not free, if it hits a piece 
# not of the same color, it is added to the result
func build_line(piece: Piece, board: Board, direction: Vector2i, res: Array[Vector2i]):
	var i = 1
	while board.in_bounds(piece.grid_position + direction * i):
		var square = piece.grid_position + direction * i
		var state = board.squares_state.get(square, Board.Square.new(board))
		var is_free = state.piece == null

		if is_free:
			res.append(square)
			state.register_attack(piece)
			board.squares_state[square] = state
		else:
			var is_same_color = state.piece.color == piece.color
			state.register_attack(piece)
			board.squares_state[square] = state
			if !is_same_color:
				res.append(square)
			break
		
		i += 1
# Builds all lines away from the piece
# effectively making a rook's move
func build_line_cross(piece: Piece, board: Board, res: Array[Vector2i]):
	build_line(piece, board, Vector2i(1, 0), res)
	build_line(piece, board, Vector2i(-1, 0), res)
	build_line(piece, board, Vector2i(0, 1), res)
	build_line(piece, board, Vector2i(0, -1), res)


# Builds a diagonal in the direction of the vector away from the piece
# until it hits a square that is not free, if it hits a piece 
# not of the same color, it is added to the result
func build_diagonal(piece: Piece, board: Board, direction: Vector2i, res: Array[Vector2i]):
	var i = 1
	while board.in_bounds(piece.grid_position + direction * i):
		var square = piece.grid_position + direction * i
		var state = board.squares_state.get(square, Board.Square.new(board))
		var is_free = state.piece == null
		var is_same_color = !is_free and state.piece.color == piece.color
		
		if is_free:
			res.append(square)
			state.register_attack(piece)
			board.squares_state[square] = state
		else:
			state.register_attack(piece)
			board.squares_state[square] = state
			if !is_same_color:
				res.append(square)
			break
		
		i += 1
# Builds all diagonals away from the piece
# effectively making a bishop's move
func build_diagonal_cross(piece: Piece, board: Board, res: Array[Vector2i]):
	build_diagonal(piece, board, Vector2i(1, 1), res)
	build_diagonal(piece, board, Vector2i(1, -1), res)
	build_diagonal(piece, board, Vector2i(-1, 1), res)
	build_diagonal(piece, board, Vector2i(-1, -1), res)

# Builds squares that are d1 and d2 away from the piece in all directions
# Effectively building a square of size 2*d1+1 x 2*d2+1
# when this function is called two times
# once with d1 = 1, d2 = 2
# once with d1 = 2, d2 = 1
# it builds a knight's move
func build_square(piece: Piece, board: Board, d1: int, d2: int, res: Array[Vector2i]):
	for m1 in [1, -1]:
		for m2 in [1, -1]:
			var square = piece.grid_position + Vector2i(d1 * m1, d2 * m2)
			var state = board.squares_state.get(square, Board.Square.new(board))
			var is_same_color = state.piece != null and state.piece.color == piece.color
			if not is_same_color and board.in_bounds(square):
				res.append(square)
			else:
				continue	
