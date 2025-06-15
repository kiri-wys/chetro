class_name BoardLabel extends Placeable

@onready var label: Label = $Label

func _ready() -> void:
	label.text = "%s:%s" % [grid_position.x, grid_position.y]
	move_to_grid(Vector2i(0,0))
