"""
https://adventofcode.com/2024/day/24
"""

OPS = {
    'AND': lambda x, y: x and y,
    'OR': lambda x, y: x or y,
    'XOR': lambda x, y: x ^ y,  # x != y
}


def main1() -> int:
    """part1"""
    with open('input.txt', 'r') as f:
        inits, gates = f.read().strip().split('\n\n')

    init_map = {}
    for line in inits.split("\n"):
        node, init = line.split(": ")
        init_map[node] = int(init)

    graph = {}
    output_layer = []
    for line in gates.split("\n"):
        nodes, target = line.split(" -> ")
        left, op, right = nodes.split()
        graph[target] = (left, OPS[op], right)
        if target.startswith("z"):
            output_layer.append(target)

    # end of process input
    stack = output_layer.copy()
    while stack:
        node = stack.pop()
        left, fcn, right = graph[node]
        if left not in init_map or right not in init_map:
            stack.append(node)
            if left not in init_map:
                stack.append(left)
            if right not in init_map:
                stack.append(right)
            continue

        predec1 = init_map[left]
        predec2 = init_map[right]
        init_map[node] = fcn(predec1, predec2)

    # print([(z, init_map[z]) for z in sorted(output_layer, reverse=True)])
    # return int(''.join([str(init_map[z]) for z in sorted(output_layer, reverse=True)]), 2)
    return sum([
        init_map[z] * pow(2, power) for power, z in enumerate(sorted(output_layer, reverse=False))
    ])


def main2() -> str:
    with open('input.txt', 'r') as f:
        inits, gates = f.read().strip().split('\n\n')

    init_map = {}
    for line in inits.split("\n"):
        node, init = line.split(": ")
        init_map[node] = int(init)

    graph = {}
    nodes = set()
    select = set()
    for line in gates.split("\n"):
        nodes_, target = line.split(" -> ")
        left, op, right = nodes_.split()
        graph[target] = (left, op, right)
        nodes.add((left, op))
        nodes.add((right, op))

    # initally solved in graph viz
    for key, val in graph.items():
        print(f"{val[0]} -> {key};")
        print(f"{val[2]} -> {key};")
    print()

    for key, (predec1, op, predec2) in graph.items():
        if op == 'AND':
            if predec1 != 'x00' and predec2 != 'x00' and (key, 'OR') not in nodes:
                select.add(key)

        elif op == 'OR':
            if key.startswith('z') and key != 'z45':
                select.add(key)
            if (key, 'OR') in nodes:
                select.add(key)

        elif op == 'XOR':
            if predec1.startswith('x') or predec2.startswith('x'):
                if predec1 != 'x00' and predec2 != 'x00' and (key, 'XOR') not in nodes:
                    select.add(key)
            elif not key.startswith('z'):
                select.add(key)
        else:
            raise ValueError(f'Unknown operation: {op}')

    intermed = graph['z18']
    graph['z18'] = graph['qgd']
    graph['qgd'] = intermed

    for key, val in graph.items():
        print(f"{val[0]} -> {key};")
        print(f"{val[2]} -> {key};")

    return ','.join(sorted(select))


if __name__ == '__main__':
    # print(main1())  # 45121475050728
    print(main2())  # gqp,hsw,jmh,mwk,qgd,z10,z18,z33
