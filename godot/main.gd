extends Node

func _ready() -> void:
	$AudioStreamPlayer.volume_db = Global.volume_db - 10
