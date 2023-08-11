file = open("colors.txt", "r")
output = open("colors_result.txt", "w")

# color stuff
color_rgb_length = len("/// <div style=\"background-color:")
color_rgb = ""

# definition line stuff
def_length = len("pub const ")

for line in file:
    # do initial processing
    line = line.replace("\n", "").replace("\r", "").strip()

    # skip comments
    if line.startswith("#"): continue

    # handle color input
    if line.startswith("/// <div"):
        color_rgb = line[color_rgb_length:-1].split("; ")[0]
    # handle definition line
    elif line.startswith("pub const"):
        target = line[def_length:-1].split(": ")[0].lower()
        target = "                \"" + target + "\" => Color::" + target.upper() + ","
        print("                // " + color_rgb, file = output)
        print(target, file = output)

    # print(line)