extends Control

func _on_start_button_pressed() -> void:
	if Global.info_supressed:
		get_tree().change_scene_to_file("res://main.tscn")
	else:
		$InfoPopup.popup_centered()

func _on_quit_button_pressed() -> void:
	get_tree().quit()


func _on_info_popup_closed() -> void:
	get_tree().change_scene_to_file("res://main.tscn")


func _on_tutorial_button_pressed() -> void:
	$TutorialPopup.popup_centered()

func _on_h_slider_value_changed(value: float) -> void:
	Global.volume_db = value
	$AudioStreamPlayer.volume_db = value
