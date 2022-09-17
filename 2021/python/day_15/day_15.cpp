#include <vector>
#include <fstream>
#include <iostream>

class Graph {
    public:
        class Node {
            private:
                int riskLevel;
                std::vector<Node> neighbours;
        };

        Graph(int nNodes);
    private:
        std::vector<Node> nodes;
};

Graph::Graph(int nNodes) {
    this->nodes = std::vector<Node>(nNodes);
}

int main() {
    std::fstream input("day_15_sample.txt", std::ios_base::in);

    int x;
    while( input >> x) {
        printf("%d", x);
    };
    return 0;
}