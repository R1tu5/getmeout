extends PopupPanel

signal ignore_info 
signal closed

func _on_done_button_pressed() -> void:
	hide()
	closed.emit()

func _on_check_button_toggled(toggled_on: bool) -> void:
	Global.info_supressed = toggled_on
