#!/usr/bin/env python3

n_paths = 0


# Python program to print all paths from a source to destination.

from collections import defaultdict

# This class represents a directed graph
# using adjacency list representation
class Graph:

    def __init__(self, vertices):
        # No. of vertices
        self.V = len(vertices)
        
        # default dictionary to store graph
        self.graph = defaultdict(list)
        for v in vertices:
            self.graph[v] = []

    # function to add an edge to graph
    def addEdge(self, u, v):
        self.graph[u].append(v)

    '''A recursive function to print all paths from 'u' to 'd'.
    visited[] keeps track of vertices in current path.
    path[] stores actual vertices and path_index is current
    index in path[]'''
    def printAllPathsUtil(self, u: str, d, visited, path):

        # Mark the current node as visited and store in path
        if not u.isupper():
            visited[u] += 1
        path.append(u)

        # If current vertex is same as destination, then print
        # current path[]
        if u == d:
            print(path)
            global n_paths
            n_paths += 1
        else:
            # If current vertex is not destination
            # Recur for all the vertices adjacent to this vertex
            for i in self.graph[u]:
                # see which lowercase we are doing twice
                lower_twice = [k for k, v in visited.items() if not k.isupper() and v == 2 ]
                if visited[i] == 0 or i.isupper() or (len(lower_twice) == 0 and i != 'start' and i != 'end'):
                    self.printAllPathsUtil(i, d, visited, path)
                    
        # Remove current vertex from path[] and mark it as unvisited
        path.pop()
        visited[u] -= 1


    # Prints all paths from 's' to 'd'
    def printAllPaths(self, s, d):

        # Mark all the vertices as not visited
        visited = [False]*(self.V)
        visited = {k: 0 for k in self.graph.keys()}
        print(visited)

        # Create an array to store paths
        path = []

        # Call the recursive helper function to print all paths
        self.printAllPathsUtil(s, d, visited, path)



input = '''start-A
start-b
A-c
A-b
b-d
A-end
b-end'''.splitlines()

input = '''dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc'''.splitlines()

input = '''fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW'''.splitlines()

input = '''YW-end
DK-la
la-XG
end-gy
zq-ci
XG-gz
TF-la
xm-la
gy-gz
ci-start
YW-ci
TF-zq
ci-DK
la-TS
zq-YW
gz-YW
zq-gz
end-gz
ci-TF
DK-zq
gy-YW
start-DK
gz-DK
zq-la
start-TF'''.splitlines()


def main():
    vertices = set()
    for line in input:
        for vertex in line.split('-'):
            vertices.add(vertex)
    vertices = list(vertices)
    print(vertices)
    g = Graph(vertices)
    for line in input:
        points = line.split('-')
        start = points[0]
        end = points[1]
        g.addEdge(start, end)
        g.addEdge(end, start)
    print(g.graph)
    s = vertices.index('start')
    d = vertices.index('end')

    s = 'start'
    d = 'end'
    # Create a graph given in the above diagram
    # g = Graph(4)
    # g.addEdge(0, 1)
    # g.addEdge(0, 2)
    # g.addEdge(0, 3)
    # g.addEdge(2, 0)
    # g.addEdge(2, 1)
    # g.addEdge(1, 3)

    # s = 2 ; d = 3
    print ("Following are all different paths from %s to %s :" %(s, d))
    g.printAllPaths(s, d)
    # This code is contributed by Neelam Yadav
    print(n_paths)

def main2():
    pass

if __name__ == '__main__':
    main()
    main2()