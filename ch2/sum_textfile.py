import sys
total = 0

for i, v in enumerate(sys.argv):
    if i == 0: continue

    with open(v, "rt", encoding="cp949") as fp: # ?? edcode
        text = fp.read()

        for line in text.split("\n"):
            try:
                total += float(line)
            except ValueError:
                pass;

print(total)