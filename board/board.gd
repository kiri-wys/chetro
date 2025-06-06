class_name Board extends Sprite2D

const PIECE_SCENE = preload("res://board/pieces/piece.tscn")
const INDIC_SCENE = preload("res://board/pieces/placeable.tscn")

@onready var sprite_size: Vector2 = self.texture.get_size()
var rows: int
var columns: int
@onready var cursor: Cursor = $Cursor

enum SquareState {
	FREE,
	BLACK,
	WHITE,
	}
var selected_piece: Piece
var squares_state: Dictionary[Vector2i, SquareState]

signal gizmo_deletion

func in_bounds(grid_position: Vector2i) -> bool:
	return grid_position.x >= 1 and grid_position.x <= rows and \
		   grid_position.y >= 1 and grid_position.y <= columns

func show_move_gizmos(piece: Piece):
	piece.toggle_select()
	var moves = piece.info._valid_moves(piece, self)
	for mov in moves:
		var gizmo: Placeable = INDIC_SCENE.instantiate()
		gizmo.info = PieceInfo.load_or_fallback("move_gizmo")
		gizmo.grid_position = mov
		gizmo.board_size = sprite_size
		add_child(gizmo)

		gizmo_deletion.connect(gizmo.queue_free)

func _ready() -> void:
	instantiate_piece(PieceInfo.load_or_fallback("pawn"), Vector2i(1,2), Piece.PieceColor.WHITE)
	instantiate_piece(PieceInfo.load_or_fallback("rook"), Vector2i(1,1), Piece.PieceColor.WHITE)
	instantiate_piece(PieceInfo.load_or_fallback("bishop"), Vector2i(2,1), Piece.PieceColor.WHITE)
	instantiate_piece(PieceInfo.load_or_fallback("knight"), Vector2i(3,1), Piece.PieceColor.WHITE)
	instantiate_piece(PieceInfo.load_or_fallback("queen"), Vector2i(4,1), Piece.PieceColor.WHITE)

	instantiate_piece(PieceInfo.load_or_fallback("pawn"), Vector2i(1,7), Piece.PieceColor.BLACK)
	instantiate_piece(PieceInfo.load_or_fallback("rook"), Vector2i(4,3), Piece.PieceColor.BLACK)
	
	rows = (sprite_size.y / Globals.square_size) as int
	columns = (sprite_size.x / Globals.square_size) as int

	cursor.board = self
	cursor.square_clicked.connect(func():
		print(Cursor.SelectionState.keys()[cursor.get_state(selected_piece)])
		match cursor.get_state(selected_piece):
			Cursor.SelectionState.SAME_PIECE: pass
			Cursor.SelectionState.DIFFERENT_PIECE:
				gizmo_deletion.emit()

				selected_piece.toggle_select()
				if cursor.piece_under.color != Piece.PieceColor.WHITE: 
					var moves = selected_piece.info._valid_moves(selected_piece, self)
					if cursor.piece_under.grid_position in moves:
						squares_state[selected_piece.grid_position] = SquareState.FREE
						selected_piece.grid_position = cursor.piece_under.grid_position
						squares_state[selected_piece.grid_position] = 1 + selected_piece.color as SquareState
						selected_piece.move_to_grid()
						cursor.piece_under.capture()
					else:
						selected_piece.toggle_select()
					selected_piece = null
					return
				show_move_gizmos(cursor.piece_under)
			Cursor.SelectionState.NEW_PIECE: 
				gizmo_deletion.emit()

				if cursor.piece_under.color != Piece.PieceColor.WHITE: return
				show_move_gizmos(cursor.piece_under)

			Cursor.SelectionState.MOVE_PIECE: 
				gizmo_deletion.emit()

				var moves = selected_piece.info._valid_moves(selected_piece, self)
				selected_piece.toggle_select()
				if cursor.grid_position in moves:
					squares_state[selected_piece.grid_position] = SquareState.FREE
					selected_piece.grid_position = cursor.grid_position
					squares_state[selected_piece.grid_position] = 1 + selected_piece.color as SquareState
					selected_piece.move_to_grid()
			Cursor.SelectionState.NOTHING: 
				gizmo_deletion.emit()

		selected_piece = cursor.piece_under
	)

func instantiate_piece(piece_info: PieceInfo, grid_position: Vector2i, color: Piece.PieceColor) -> void:
	squares_state[grid_position] = 1 + color as SquareState
	var piece: Piece = PIECE_SCENE.instantiate()
	piece.front = Vector2(0, -1) if color == Piece.PieceColor.BLACK else Vector2(0, 1)
	piece.color = color
	piece.info = piece_info
	piece.grid_position = grid_position 
	piece.board_size = sprite_size
	self.add_child(piece)
