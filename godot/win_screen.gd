extends Control

func on_win(score: int):
	$MarginContainer/VBoxContainer/Score.text = "Steps Taken: " + String.num_uint64(score)
	visible = true


func _on_menu_pressed() -> void:
	get_tree().change_scene_to_file("res://start_screen.tscn")
