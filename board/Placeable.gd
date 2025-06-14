class_name Placeable extends Node2D

signal moved()

@export var info: PieceInfo

var grid_position: Vector2i
var board_size: Vector2

func _ready() -> void:
	$Sprite2D.region_rect = info.region()
	move_to_grid(Vector2i(0,0))

func move_to_grid(old_pos: Vector2i):
	var half = board_size / 2.0
	var root = Vector2(-half.x, half.y)

	self.position = root

	self.position.x -= Globals.half_square_size
	self.position.y += Globals.half_square_size

	self.position.x += grid_position.x * Globals.square_size
	self.position.y -= grid_position.y * Globals.square_size

	moved.emit(old_pos)
