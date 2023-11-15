input_file  = open("styles_source.txt", "r")
output_file = open("styles_output.txt", "w")

for l in input_file.readlines():
    line = l[4:len(l) - 2]
    tokens = line.split(": ")
    # print("pub fn " + tokens[0] + "(&mut self, " + tokens[0] + ": " + tokens[1] + ") -> &mut Self {", end = "")
    print("pub fn ", end = "", file = output_file)
    print(tokens[0], end = "", file = output_file)
    print("(&mut self, ", end = "", file = output_file)
    print(tokens[0], end = "", file = output_file)
    print(": ", end = "", file = output_file)
    print(tokens[1], end = "", file = output_file)
    print(") -> &mut Self { self.style.", end = "", file = output_file)
    print(tokens[0], end = "", file = output_file)
    print(" = ", end = "", file = output_file)
    print(tokens[0], end = "", file = output_file)
    print("; self.mark_dirty() }", end = "", file = output_file)

    print("", file = output_file)
