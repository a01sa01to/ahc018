import os

res = []
for i in range(0, 101):
  print(i, len(res))
  with open(f".{os.sep}tools{os.sep}in{os.sep}{i}.txt") as f:
    lines = f.readlines()
    cnt = 0
    for line in lines:
      cnt += 1
      if 2 <= cnt and cnt <= 201:
        res.append(line.split(" "))

print(len(res))
with open("res.txt", "w") as f:
  f.write("".join(["\t".join(x) for x in res]))
