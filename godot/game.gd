extends Node2D
const size = 5
const tile_size = 64

var current_position= Vector2i(2,5)
var current_maze: Maze
var maze_pos = Vector2i(2,4)
var super_maze: SuperMaze

var last_maze_pos: Vector2i
var last_maze_entrance: Vector2i
var regenerate_last = false

var animation = true

signal win(score: int)
var score = 0

func _ready():
	super_maze = SuperMaze.create()
	$Map/Escape.global_position = (Vector2i(2,-3) + Vector2i(2,0)*size)*tile_size + Vector2i(tile_size/2,tile_size/2)
	$Map/Maze.set_cell(abs_pos(), 0, Vector2i(5,2))
	$Map/Maze.set_cell(Vector2i(2,0) + Vector2i(2,0)*size + Vector2i.UP, 0, Vector2i(5,0))
	for y in range(size):
		for x in range(size):
			var v = Vector2i(x,y)
			build_maze(v)
			build_fog(v)
	current_maze = super_maze.retrieve_maze(maze_pos)
	position = DisplayServer.window_get_size()/2.0
	position -= Vector2((abs_pos())*tile_size) + Vector2(tile_size/2,tile_size/2)
	$Player.global_position = DisplayServer.window_get_size()/2.0
	$Map/Fog.set_cell(maze_pos*size + maze_pos, -1)
	play_animation()

func play_animation():
	var tmp = position
	position += Vector2(0,tile_size * size * size-1)
	await get_tree().create_timer(1.0).timeout
	var tween = create_tween() 
	tween.tween_property(self, "position", tmp, 4)
	tween.set_trans(Tween.TRANS_SINE).set_ease(Tween.EASE_IN_OUT)
	tween.connect("finished", Callable(self, "_on_animation_finished"))
	

func abs_pos():
	return current_position + maze_pos*size
	
func build_maze(maze_pos: Vector2i):
	var maze = super_maze.retrieve_maze(maze_pos)
	for y in range(size):
		for x in range(size):
			var v = Vector2i(x,y)
			$Map/Maze.set_cell(maze_pos*size + v, 0, maze.get(v))

func build_fog(maze_pos: Vector2i):
	var rnd_num = randi_range(0,27)
	var choice = Vector2i(rnd_num % 8, rnd_num/8)*size
	for y in range(size):
		for x in range(size):
			var v = Vector2i(x,y)
			$Map/Fog.set_cell(maze_pos*size + v, 0, choice+v)

func reload_maze(maze_pos: Vector2i):
	super_maze.reload(maze_pos)
	build_maze(maze_pos)

func _process(delta: float) -> void:
	movement_logic(delta)
	if Input.is_action_just_released("ui_accept"):
		var rnd_vec = Vector2i(0,0)
		reload_maze(rnd_vec)

func movement_logic(delta: float) -> void:
	if !animation:
		if current_position == Vector2i(2,-1):
			finish_game()
		var move = Vector2i.ZERO
		if Input.is_action_pressed("move_left"):
			move = Vector2i.LEFT
		elif Input.is_action_pressed("move_up"):
			move = Vector2i.UP
		elif Input.is_action_pressed("move_right"):
			move = Vector2i.RIGHT
		elif Input.is_action_pressed("move_down"):
			move = Vector2i.DOWN
		
		if move != Vector2i.ZERO:
			$Player.set_direction(move)
			if !is_start_or_end_pos() or is_start_or_end_pos() and move==Vector2i.UP:
				if !current_maze.get_wall_g(current_position, move):
					manage_move(move)
					score += 1
					if !is_start_or_end_pos():
						update_fog()
				
				
func update_fog():
	for v in [Vector2i.RIGHT, Vector2i.LEFT,Vector2i.DOWN, Vector2i.UP]:
		if !current_maze.get_wall_g(current_position, v):
			v = v + current_position + maze_pos*size
			$Map/Fog.set_cell(v, -1)

func manage_move(move: Vector2i):
	var new_pos = move + current_position
	if regenerate_last && abs_pos().distance_squared_to(last_maze_entrance+last_maze_pos*size) > 3:
		reload_maze(last_maze_pos)
		build_fog(last_maze_pos)
		regenerate_last = false
	if !is_start_or_end_pos(new_pos) and (new_pos.x < 0 || new_pos.x > size - 1 || new_pos.y < 0 || new_pos.y > size - 1):
		regenerate_last = true
		last_maze_entrance = current_position
		last_maze_pos = maze_pos
		if new_pos.x < 0:
			new_pos.x = size - 1
			maze_pos.x -= 1
		if new_pos.x > size - 1:
			new_pos.x = 0
			maze_pos.x += 1
		if new_pos.y < 0:
			new_pos.y = size - 1
			maze_pos.y -= 1
		if new_pos.y > size - 1:
			new_pos.y = 0
			maze_pos.y += 1
		current_maze = super_maze.retrieve_maze(maze_pos)
	animation = true
	current_position = new_pos
	$Player.start_animation()
	var tween = create_tween() 
	var new_pixel_pos = $Map.global_position + Vector2(-move * tile_size)
	tween.tween_property($Map, "global_position", new_pixel_pos, 0.3)
	tween.set_trans(Tween.TRANS_SINE).set_ease(Tween.EASE_IN_OUT)
	tween.connect("finished", Callable(self, "_on_animation_finished"))

func is_start_or_end_pos(pos=current_position):
	return (pos==Vector2i(2,5) and maze_pos==Vector2i(2,4)) or (pos==Vector2i(2,-1) and maze_pos==Vector2i(2,0))

func _on_animation_finished():
	animation = false
	$Player.stop_animation()
	
func finish_game():
	animation = true
	win.emit(score)
