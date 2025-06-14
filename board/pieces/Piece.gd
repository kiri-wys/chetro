class_name Piece extends Placeable

signal disconect_piece()

enum PieceColor {
	BLACK,
	WHITE,
	}

var color: PieceColor
var selected: bool = false
var front: Vector2i

func set_select(value: bool):
	selected = value
	if selected:
		modulate = Color.RED
	else:
		modulate = Color.WHITE

func toggle_select():
	selected = !selected
	if selected:
		modulate = Color.RED
	else:
		modulate = Color.WHITE

func capture() -> void:
	disconect_piece.emit()
	queue_free()
