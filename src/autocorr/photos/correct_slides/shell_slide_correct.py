from photoshop import Session
import subprocess
import argparse

# Running this requires having photoshop installed on the worker
def correct_slide(from_path: str, to_dir: str):
    file_name = from_path.split("\\")[-1]
    to_path = f"{to_dir}\\{file_name}"

    # Get color in top left corner
    get_color_cmd = "magick {} -format \"%[hex:u.p{{2,2}}]\\n\" info:".format(from_path)
    p = subprocess.Popen(get_color_cmd, stdout=subprocess.PIPE)
    lines = []
    for line in p.stdout:
        lines.append(line)
    hex = str(lines[0])[2:8]

    base_rotateandcrop_cmd = f"magick {from_path} -fuzz 30% -set option:angle \"%[minimum-bounding-box:unrotate]\" -background \"#{hex}\" -rotate \"%[angle]\" -trim {to_path}"
    process = subprocess.Popen(base_rotateandcrop_cmd, shell=True, stdout=subprocess.PIPE)
    process.wait()
    
    with Session() as ps:
        ps.app.load(to_path)
        ps.app.doAction('AutoColor')
        doc = ps.active_document
        doc.saveAs(to_path, ps.JPEGSaveOptions(10))
        doc.close()

    return 0

parser = argparse.ArgumentParser()
parser.add_argument("from_path", type=str, help="path to uncorrected slide image file")
parser.add_argument("to_folder", type=str, help="path to folder to which corrected image file will be stored")
args = parser.parse_args()
correct_slide(args.from_path, args.to_folder)