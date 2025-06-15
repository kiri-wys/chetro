class_name Board extends Sprite2D

const PIECE_SCENE = preload("res://board/pieces/piece.tscn")
const INDIC_SCENE = preload("res://board/placeable.tscn")
const LABEL_SCENE = preload("res://board/board_label.tscn")

@onready var sprite_size: Vector2 = self.texture.get_size()
var rows: int
var columns: int
@onready var cursor: Cursor = $Cursor

class Square:
	var piece: Piece
	var attacking_pieces: Array[Piece]
	var num_black_attacks: int
	var num_white_attacks: int
	var board: Board

	func _init(b: Board) -> void:
		board = b

	func register_attack(atk_piece: Piece):
		if atk_piece in attacking_pieces: return
		attacking_pieces.append(atk_piece)
		atk_piece.moved.connect(func(old_pos): 
			var old_sqr: Square = board.squares_state.get(old_pos, Square.new(board))
			var new_sqr: Square = board.squares_state.get(atk_piece.grid_position, Square.new(board))

			for p in old_sqr.attacking_pieces:
				p.info._pseudo_valid_moves(p, board)
			for p in new_sqr.attacking_pieces:
				p.disconect_piece.emit()
				p.info._pseudo_valid_moves(p, board)

			board.squares_state[old_pos] = old_sqr
			board.squares_state[atk_piece.grid_position] = new_sqr
			unregister_attack(atk_piece), ConnectFlags.CONNECT_ONE_SHOT)
		atk_piece.disconect_piece.connect(func():
			unregister_attack(atk_piece), ConnectFlags.CONNECT_ONE_SHOT)

		if atk_piece.color == Piece.PieceColor.WHITE:
			num_white_attacks += 1
		else:
			num_black_attacks += 1
	
	func unregister_attack(atk_piece: Piece):
		if !(atk_piece in attacking_pieces): return
		attacking_pieces.erase(atk_piece)
		if atk_piece.color == Piece.PieceColor.WHITE:
			num_white_attacks -= 1
		else:
			num_black_attacks -= 1

var selected_piece: Piece
var squares_state: Dictionary[Vector2i, Square]
var labels: Dictionary[Vector2i, BoardLabel]

signal gizmo_deletion

func in_bounds(grid_position: Vector2i) -> bool:
	return grid_position.x >= 1 and grid_position.x <= rows and \
		   grid_position.y >= 1 and grid_position.y <= columns

func show_move_gizmos(piece: Piece):
	piece.toggle_select()
	var moves = piece.info._pseudo_valid_moves(piece, self)
	for mov in moves:
		var gizmo: Placeable = INDIC_SCENE.instantiate()
		gizmo.info = PieceInfo.load_or_fallback("move_gizmo")
		gizmo.grid_position = mov
		gizmo.board_size = sprite_size
		add_child(gizmo)

		gizmo_deletion.connect(gizmo.queue_free)

func _after_ready() -> void:
	for pos in squares_state:
		var pic = squares_state[pos]
		var piece = pic.piece
		if piece != null:
			piece.info._pseudo_valid_moves(piece, self)
func _ready() -> void:
	#instantiate_piece(PieceInfo.load_or_fallback("pawn"), Vector2i(1,2), Piece.PieceColor.WHITE)
	instantiate_piece(PieceInfo.load_or_fallback("rook"), Vector2i(1,1), Piece.PieceColor.WHITE)
	instantiate_piece(PieceInfo.load_or_fallback("bishop"), Vector2i(2,1), Piece.PieceColor.WHITE)
	#instantiate_piece(PieceInfo.load_or_fallback("knight"), Vector2i(3,1), Piece.PieceColor.WHITE)
	#instantiate_piece(PieceInfo.load_or_fallback("queen"), Vector2i(4,1), Piece.PieceColor.WHITE)

	#instantiate_piece(PieceInfo.load_or_fallback("pawn"), Vector2i(1,7), Piece.PieceColor.BLACK)
	#instantiate_piece(PieceInfo.load_or_fallback("rook"), Vector2i(4,3), Piece.PieceColor.BLACK)
	
	rows = (sprite_size.y / Globals.square_size) as int
	columns = (sprite_size.x / Globals.square_size) as int
	for y in range(1, rows + 1):
		for x in range(1, columns + 1):
			var p = Vector2i(x, y)
			var l = LABEL_SCENE.instantiate()
			l.board_size = sprite_size
			l.grid_position = p
			labels[p] = l
			add_child(l)

	cursor.board = self
	cursor.square_clicked.connect(func():
		print(Cursor.SelectionState.keys()[cursor.get_state(selected_piece)])
		match cursor.get_state(selected_piece):
			Cursor.SelectionState.SAME_PIECE: pass
			Cursor.SelectionState.DIFFERENT_PIECE:
				gizmo_deletion.emit()

				selected_piece.toggle_select()
				if cursor.piece_under.color != Piece.PieceColor.WHITE: 
					if try_move_selected(cursor.piece_under.grid_position):
						cursor.piece_under.capture()
					else:
						#selected_piece.toggle_select()
						pass
					selected_piece = null
					return
				show_move_gizmos(cursor.piece_under)
			Cursor.SelectionState.NEW_PIECE: 
				gizmo_deletion.emit()

				if cursor.piece_under.color != Piece.PieceColor.WHITE: return
				show_move_gizmos(cursor.piece_under)

			Cursor.SelectionState.MOVE_PIECE: 
				gizmo_deletion.emit()

				selected_piece.toggle_select()
				try_move_selected(cursor.grid_position)
			Cursor.SelectionState.NOTHING: 
				gizmo_deletion.emit()

		selected_piece = cursor.piece_under
	)

	call_deferred("_after_ready")

func _process(delta: float) -> void:
	for y in range(1, 9):
		for x in range(1,9):
			var p = Vector2i(x, y)
			var cell = squares_state.get(p, Square.new(self))
			var label = labels.get(p)
			label.label.text = "%s:%s" % [cell.num_white_attacks, cell.num_black_attacks]


func instantiate_piece(piece_info: PieceInfo, grid_position: Vector2i, color: Piece.PieceColor) -> void:
	squares_state[grid_position] = Square.new(self)
	var piece: Piece = PIECE_SCENE.instantiate()
	piece.front = Vector2(0, -1) if color == Piece.PieceColor.BLACK else Vector2(0, 1)
	piece.color = color
	piece.info = piece_info
	piece.grid_position = grid_position 
	piece.board_size = sprite_size
	squares_state[grid_position].piece = piece
	self.add_child(piece)

func try_move_selected(grid_position: Vector2i) -> bool:
	var moves = selected_piece.info._pseudo_valid_moves(selected_piece, self)
	if grid_position not in moves: return false
	
	var sq = squares_state.get(selected_piece.grid_position, Square.new(self))
	sq.piece = null
	squares_state[selected_piece.grid_position] = sq
	
	var old_pos = selected_piece.grid_position
	selected_piece.grid_position = grid_position
	sq = squares_state.get(selected_piece.grid_position, Square.new(self))
	sq.piece = selected_piece
	squares_state[selected_piece.grid_position] = sq
	
	selected_piece.move_to_grid(old_pos)
	# HACK: Tun again to register attack squares at new position.
	# This could be a separate function to optimize for pieces whose
	# attack patterns are different than their move patterns i.e; Pawns
	selected_piece.info._pseudo_valid_moves(selected_piece, self)
	return true
	
