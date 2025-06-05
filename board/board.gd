class_name Board extends Sprite2D

const PIECE_SCENE = preload("res://board/pieces/piece.tscn")
const INDIC_SCENE = preload("res://board/pieces/placeable.tscn")

@onready var sprite_size: Vector2 = self.texture.get_size()
@onready var cursor: Cursor = $Cursor

var selected_piece: Piece
signal gizmo_deletion

func _ready() -> void:
	instantiate_piece(PieceInfo.load_or_fallback("pawn"), Vector2i(1,3))
	instantiate_piece(PieceInfo.load_or_fallback("pawn"), Vector2i(2,3))

	cursor.board = self
	cursor.square_clicked.connect(func():
		print(Cursor.SelectionState.keys()[cursor.get_state(selected_piece)])
		match cursor.get_state(selected_piece):
			Cursor.SelectionState.SAME_PIECE: pass
			Cursor.SelectionState.DIFFERENT_PIECE:
				gizmo_deletion.emit()

				selected_piece.toggle_select()

				cursor.piece_under.toggle_select()
				var moves = cursor.piece_under.info._valid_moves(cursor.piece_under, self)
				for mov in moves:
					var gizmo: Placeable = INDIC_SCENE.instantiate()
					gizmo.info = MoveGizmo.new()
					gizmo.grid_position = mov
					gizmo.board_size = sprite_size
					add_child(gizmo)

					gizmo_deletion.connect(gizmo.queue_free)
			Cursor.SelectionState.NEW_PIECE: 
				gizmo_deletion.emit()

				cursor.piece_under.toggle_select()
				var moves = cursor.piece_under.info._valid_moves(cursor.piece_under, self)
				for mov in moves:
					var gizmo: Placeable = INDIC_SCENE.instantiate()
					gizmo.info = MoveGizmo.new()
					gizmo.grid_position = mov
					gizmo.board_size = sprite_size
					add_child(gizmo)

					gizmo_deletion.connect(gizmo.queue_free)
			Cursor.SelectionState.MOVE_PIECE: 
				gizmo_deletion.emit()

				var moves = selected_piece.info._valid_moves(selected_piece, self)
				selected_piece.toggle_select()
				if cursor.grid_position in moves:
					selected_piece.grid_position = cursor.grid_position
					selected_piece.move_to_grid()
			Cursor.SelectionState.NOTHING: 
				gizmo_deletion.emit()

		selected_piece = cursor.piece_under
	)

func instantiate_piece(piece_info: PieceInfo, grid_position: Vector2i = Vector2i.ZERO) -> void:
	var piece: Piece = PIECE_SCENE.instantiate()
	piece.info = piece_info
	piece.grid_position = grid_position * 1.0
	piece.board_size = sprite_size
	self.add_child(piece)
