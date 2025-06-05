class_name Piece extends Placeable

var selected: bool = false

func toggle_select():
	selected = !selected
	if selected:
		modulate = Color.RED
	else:
		modulate = Color.WHITE
