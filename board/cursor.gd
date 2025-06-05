class_name Cursor extends Area2D

enum SelectionState {
	SAME_PIECE,
	DIFFERENT_PIECE,
	NEW_PIECE,
	MOVE_PIECE,
	NOTHING,
}
func get_state(selected_piece) -> SelectionState:
	if piece_under != null:
		if selected_piece != null:
			if selected_piece == piece_under:
				return SelectionState.SAME_PIECE
			else:
				return SelectionState.DIFFERENT_PIECE
		else: return SelectionState.NEW_PIECE
	else:
		if selected_piece != null: return SelectionState.MOVE_PIECE
		else: return SelectionState.NOTHING

var board: Board 
var grid_position: Vector2

var piece_under: Piece 

signal square_clicked()

func _ready() -> void:
	area_entered.connect(func(area: Area2D): 
		if area.owner is Piece:
			piece_under = area.owner
	)
	area_exited.connect(func(area: Area2D):
		if area.owner is Piece:
			piece_under = null
	)

func _physics_process(_delta: float) -> void:
	var m = board.get_local_mouse_position()
	self.position.x = ceilf(m.x / Globals.square_size) * Globals.square_size - Globals.half_square_size
	self.position.y = ceilf(m.y / Globals.square_size) * Globals.square_size - Globals.half_square_size

	# board_size / size_cell = number of cells
	# num_cells / 2.0 = half the number of cells, offset so 0,0 is at bottom left
	# x / a / b = (x/a) * 1/b = x/(a*b)
	grid_position.x = ceilf(m.x / Globals.square_size) + (board.sprite_size.x / Globals.double_square_size)
	grid_position.y = absf(ceilf(m.y / Globals.square_size) - (board.sprite_size.y / Globals.double_square_size) - 1.0)

	if Input.is_action_just_pressed("square_click"):
		square_clicked.emit()
