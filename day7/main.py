
# rust trees are hard, and I don't know what i'm doing with Rc<RefCell<T>>'s,
# .. so here's a hacky intermediate python solution until my elegant rust
# .. solution stops being screamed at by the borrow checker

with open('instructions.txt', 'r') as f:
    instructions = [line.strip() for line in f]

'''
fs = {"/": [("asdf.txt", 1234), ("qwer.txt", 7890)],
      "/Users": [..],
      "/Users/pemulis0x": [..],
      ...
      }
'''
fs = {"/": []}


def sum_dir(target: str) -> int:
    total = 0
    for k, v in fs.items():
        if target in k:
            for f in v:
                total += int(f[0])

    return total


# returns the key of directory specified by params
def add_dir_if_new(current_dir: str, new_dir: str) -> str:
    if current_dir == "/":
        new_dir = current_dir + new_dir
    else:
        new_dir = current_dir + "/" + new_dir
    if fs.get(new_dir) is None:
        fs.update({new_dir: []})

    return new_dir


# assumes dir is valid
def add_file_if_new(parent_dir: str, file: tuple):
    fs.get(parent_dir).append(file)


# only works for adjacent dirs
def change_dir(current_dir: str, dest_dir: str) -> str:
    if dest_dir == "/":
        return "/"

    if current_dir == "/":
        return current_dir + dest_dir

    if dest_dir == "..":
        return current_dir.rsplit("/", 1)[0]

    return current_dir + "/" + dest_dir


working_dir = "/"
for instruction in instructions:
    # print(instruction[0:3])
    if instruction[0:4] == '$ cd':
        dest_dir = instruction.split(' ')[-1]
        working_dir = change_dir(working_dir, dest_dir)

    elif instruction[0:4] == '$ ls':
        pass

    elif instruction[0:3] == 'dir':
        dest_dir = instruction.split(' ')[-1]
        add_dir_if_new(working_dir, dest_dir)

    else:
        # (size, name)
        file = tuple(instruction.split(' '))
        add_file_if_new(working_dir, file)


'''
pprint(fs)
for d in fs:
    print(f"dir {d} has size {sum_dir(d)}")
'''

pt_1 = sum([sum_dir(d) for d in fs if sum_dir(d) < 100_000])

min_delete = 30_000_000 - (70_000_000 - sum_dir("/"))
pt_2 = min([sum_dir(d) for d in fs if sum_dir(d) > min_delete])

print(pt_1)
print(pt_2)
