# exports each selected object into its own file

import bpy
import os
from collections import namedtuple
from typing import NamedTuple

# export to blend file location
basedir = os.path.dirname(bpy.data.filepath)
if not basedir:
    raise Exception("Blend file is not saved")

view_layer = bpy.context.view_layer
class Hex(NamedTuple):
    name: str
    front: str
    back: str

hexes = [
        Hex("A001", "Bundle 1 A001 - A013\\A001 F.png", "Bundle 1 A001 - A013\\A001 B.png"),
        Hex("A002", "Bundle 1 A001 - A013\\A002 F.png", "Bundle 1 A001 - A013\\A002 B.png"),
        Hex("A003", "Bundle 1 A001 - A013\\A003 F.png", "Bundle 1 A001 - A013\\A003 B.png"),
        Hex("A004", "Bundle 1 A001 - A013\\A004 F.png", "Bundle 1 A001 - A013\\A004 B.png"),
        Hex("A005", "Bundle 1 A001 - A013\\A005 F.png", "Bundle 1 A001 - A013\\A005 B.png"),
        Hex("A006", "Bundle 1 A001 - A013\\A006 F.png", "Bundle 1 A001 - A013\\A006 B.png"),
        Hex("A007", "Bundle 1 A001 - A013\\A007 F.png", "Bundle 1 A001 - A013\\A007 B.png"),
        Hex("A008", "Bundle 1 A001 - A013\\A008 F.png", "Bundle 1 A001 - A013\\A008 B.png"),
        Hex("A009", "Bundle 1 A001 - A013\\A009 F.png", "Bundle 1 A001 - A013\\A009 B.png"),
        Hex("A010", "Bundle 1 A001 - A013\\A010 F.png", "Bundle 1 A001 - A013\\A010 B.png"),
        Hex("A011", "Bundle 1 A001 - A013\\A011 F.png", "Bundle 1 A001 - A013\\A011 B.png"),
        Hex("A012", "Bundle 1 A001 - A013\\A012 F.png", "Bundle 1 A001 - A013\\A012 B.png"),
        Hex("A013", "Bundle 1 A001 - A013\\A013 F.png", "Bundle 1 A001 - A013\\A013 B.png"),
        
        #Bundle 2
        Hex("A014", "Bundle 2 A014 - A026\\A014 F.png", "Bundle 2 A014 - A026\\A014 B.png"),
        Hex("A015", "Bundle 2 A014 - A026\\A015 F.png", "Bundle 2 A014 - A026\\A015 B.png"),
        Hex("A016", "Bundle 2 A014 - A026\\A016 F.png", "Bundle 2 A014 - A026\\A016 B.png"),
        Hex("A017", "Bundle 2 A014 - A026\\A017 F.png", "Bundle 2 A014 - A026\\A017 B.png"),
        Hex("A018", "Bundle 2 A014 - A026\\A018 F.png", "Bundle 2 A014 - A026\\A018 B.png"),
        Hex("A019", "Bundle 2 A014 - A026\\A019 F.png", "Bundle 2 A014 - A026\\A019 B.png"),
        Hex("A020", "Bundle 2 A014 - A026\\A020 F.png", "Bundle 2 A014 - A026\\A020 B.png"),
        Hex("A021", "Bundle 2 A014 - A026\\A021 F.png", "Bundle 2 A014 - A026\\A021 B.png"),
        Hex("A022", "Bundle 2 A014 - A026\\A022 F.png", "Bundle 2 A014 - A026\\A022 B.png"),
        Hex("A023", "Bundle 2 A014 - A026\\A023 F.png", "Bundle 2 A014 - A026\\A023 B.png"),
        Hex("A024", "Bundle 2 A014 - A026\\A024 F.png", "Bundle 2 A014 - A026\\A024 B.png"),
        Hex("A025", "Bundle 2 A014 - A026\\A025 F.png", "Bundle 2 A014 - A026\\A025 B.png"),
        Hex("A026", "Bundle 2 A014 - A026\\A026 F.png", "Bundle 2 A014 - A026\\A026 B.png"),

        #Bundle 3
        Hex("d001", "Bundle 3 d001 - d013\\d001 F.png", "Bundle 3 d001 - d013\\d001 B.png"),
        Hex("d002", "Bundle 3 d001 - d013\\d002 F.png", "Bundle 3 d001 - d013\\d002 B.png"),
        Hex("d003", "Bundle 3 d001 - d013\\d003 F.png", "Bundle 3 d001 - d013\\d003 B.png"),
        Hex("d004", "Bundle 3 d001 - d013\\d004 F.png", "Bundle 3 d001 - d013\\d004 B.png"),
        Hex("d005", "Bundle 3 d001 - d013\\d005 F.png", "Bundle 3 d001 - d013\\d005 B.png"),
        Hex("d006", "Bundle 3 d001 - d013\\d006 F.png", "Bundle 3 d001 - d013\\d006 B.png"),
        Hex("d007", "Bundle 3 d001 - d013\\d007 F.png", "Bundle 3 d001 - d013\\d007 B.png"),
        Hex("d008", "Bundle 3 d001 - d013\\d008 F.png", "Bundle 3 d001 - d013\\d008 B.png"),
        Hex("d009", "Bundle 3 d001 - d013\\d009 F.png", "Bundle 3 d001 - d013\\d009 B.png"),
        Hex("d010", "Bundle 3 d001 - d013\\d010 F.png", "Bundle 3 d001 - d013\\d010 B.png"),
        Hex("d011", "Bundle 3 d001 - d013\\d011 F.png", "Bundle 3 d001 - d013\\d011 B.png"),
        Hex("d012", "Bundle 3 d001 - d013\\d012 F.png", "Bundle 3 d001 - d013\\d012 B.png"),
        Hex("d013", "Bundle 3 d001 - d013\\d013 F.png", "Bundle 3 d001 - d013\\d013 B.png"),

        #Bundle 4
        Hex("M001", "Bundle 4 The peaks (mountains)\\M001 F.png", "Bundle 4 The peaks (mountains)\\M001 B.png"),
        Hex("M002", "Bundle 4 The peaks (mountains)\\M002 F.png", "Bundle 4 The peaks (mountains)\\M002 B.png"),
        Hex("M003", "Bundle 4 The peaks (mountains)\\M003 F.png", "Bundle 4 The peaks (mountains)\\M003 B.png"),
        Hex("M004", "Bundle 4 The peaks (mountains)\\M004 F.png", "Bundle 4 The peaks (mountains)\\M004 B.png"),
        Hex("M005", "Bundle 4 The peaks (mountains)\\M005 F.png", "Bundle 4 The peaks (mountains)\\M005 B.png"),
        Hex("M006", "Bundle 4 The peaks (mountains)\\M006 F.png", "Bundle 4 The peaks (mountains)\\M006 B.png"),
        Hex("M007", "Bundle 4 The peaks (mountains)\\M007 F.png", "Bundle 4 The peaks (mountains)\\M007 B.png"),
        Hex("M008", "Bundle 4 The peaks (mountains)\\M008 F.png", "Bundle 4 The peaks (mountains)\\M008 B.png"),
        Hex("M009", "Bundle 4 The peaks (mountains)\\M009 F.png", "Bundle 4 The peaks (mountains)\\M009 B.png"),
        Hex("M010", "Bundle 4 The peaks (mountains)\\M010 F.png", "Bundle 4 The peaks (mountains)\\M010 B.png"),

        #Bundle 5, city tiles, uses A001 as background
        Hex("C001", "city_tiles\\C001.png", "Bundle 1 A001 - A013\\A001 B.png"),
        Hex("C002", "city_tiles\\C002.png", "Bundle 1 A001 - A013\\A001 B.png"),
        Hex("C003", "city_tiles\\C003.png", "Bundle 1 A001 - A013\\A001 B.png"),
        Hex("C004", "city_tiles\\C004.png", "Bundle 1 A001 - A013\\A001 B.png"),
        Hex("C005", "city_tiles\\C005.png", "Bundle 1 A001 - A013\\A001 B.png"),
        Hex("C006", "city_tiles\\C006.png", "Bundle 1 A001 - A013\\A001 B.png"),
        Hex("C007", "city_tiles\\C007.png", "Bundle 1 A001 - A013\\A001 B.png"),
        Hex("C008", "city_tiles\\C008.png", "Bundle 1 A001 - A013\\A001 B.png"),


        ]
obj = view_layer.objects.active
print(obj)
name = bpy.path.clean_name(obj.name)
generated_folder = os.path.join(basedir, "generated")
try:
    os.mkdir(generated_folder)
except OSError as error:
    print(error)

for hex in hexes: 
    bpy.context.object.active_material_index = 0
    image = bpy.data.images.load(filepath = basedir + "\\" +  hex.front)
    obj.active_material.node_tree.nodes.get("Image Texture").image = image
    bpy.context.object.active_material_index = 1
    image = bpy.data.images.load(filepath = basedir + "\\" + hex.back)
    obj.active_material.node_tree.nodes.get("Image Texture").image = image
    fn = os.path.join(generated_folder, hex.name)
    bpy.ops.export_scene.gltf(filepath=fn + ".glb", use_selection=True)

    print("written:", fn)


