extends AnimatedSprite2D

var is_in_animation = false
var direction = Vector2i.UP

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass
# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	var animation_string: String
	
	if direction == Vector2i.UP:
		animation_string = "up"
	elif direction == Vector2i.DOWN:
		animation_string = "down"
	elif direction == Vector2i.RIGHT:
		animation_string = "right"
	elif direction == Vector2i.LEFT:
		animation_string = "left"
	if is_in_animation: 	
		animation_string += "_walking"
	animation = animation_string
	play()
	
func set_direction(dir: Vector2i):
	direction = dir
	
func start_animation():
	is_in_animation = true
	
func stop_animation():
	is_in_animation = false
