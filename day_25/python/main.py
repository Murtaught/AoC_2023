class Node:
    def __init__(self, name):
        self.name = name
        self.edges = set()

    def __repr__(self):
        return f"Node{{{self.name}, {self.edges}}}"

    def __str__(self):
        return self.__repr__()

nodes = {}
edge_count = 0

def get_node(name):
    global nodes
    if name not in nodes:
        nodes[name] = Node(name)
    return nodes[name]

INPUT_FILE = 'input0'

with open('../' + INPUT_FILE, 'r') as file:
    for line in file:
        name, *edges = line.rstrip().split()
        name = name[:len(name) - 1]
        edge_count += len(edges)

        node = get_node(name)
        for e in edges:
            node.edges.add(e)
            get_node(e).edges.add(name)


print(f"{len(nodes)} nodes, {edge_count} edges.")

# Graphviz generation:
with open(INPUT_FILE.replace('input', 'graph') + '.dot', 'wt') as out:
    write = lambda *a, **k: print(*a, **k, file = out)
    write("strict graph G {")
    write("  rankdir = LR;")
    for node in nodes.values():
        for e in node.edges:
            write(f"  {node.name} -- {e};")
    write("}")

