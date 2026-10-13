# modify_group_num_layers.aux2

[English](README.en.md) | [日本語](README.md)

[![AviUtl2 Catalog](https://aviutl2-catalog-badge.sevenc7c.workers.dev/badge/v/sevenc-nanashi.modify_group_num_layers-aux2)](https://aviutl2-catalog-badge.sevenc7c.workers.dev/package/sevenc-nanashi.modify_group_num_layers-aux2)

An AviUtl2 generic plugin that allows you to change the "Number of Target Layers" for group control, image composite object, etc. from shortcut keys.

## Usage

- The operation of this plugin basically targets the following objects:
  - When range is selected (Ctrl + Drag): Multiple selected objects.
  - When range is not selected: Right-clicked object or selected object (the target in the object settings screen).
- The operation of this plugin is basically added to the edit menu and the right-click menu of the object.
- "Increase/Decrease the number of target layers": Increases or decreases the number of target layers of the selected group control object.
- "Make the number of target layers unlimited": Makes the number of target layers of the selected group control object unlimited. Internally, 0 is set.
- "Set the number of target layers to 1": Sets the number of target layers of the selected group control object to 1.
- "Adjust the number of target layers to match the selected object": Adjusts the number of target layers of the selected group control object so that the lowest layer of the selected object is included.
  - This operation behaves the same as the standard "Adjust Target Layer Range", but unlike the standard, you can assign shortcut keys.

## Installation

Download `sevenc-nanashi.modify_group_num_layers-aux2-v{{version}}.au2pkg.zip` from [Releases](https://github.com/sevenc-nanashi/modify_group_num_layers.aux2/releases/latest) and drag and drop it into the preview of AviUtl2.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
