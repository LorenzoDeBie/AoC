#!/usr/bin/env python3


template = 'PPFCHPFNCKOKOSBVCFPP'

input = '''VC -> N
SC -> H
CK -> P
OK -> O
KV -> O
HS -> B
OH -> O
VN -> F
FS -> S
ON -> B
OS -> H
PC -> B
BP -> O
OO -> N
BF -> K
CN -> B
FK -> F
NP -> K
KK -> H
CB -> S
CV -> K
VS -> F
SF -> N
KB -> H
KN -> F
CP -> V
BO -> N
SS -> O
HF -> H
NN -> F
PP -> O
VP -> H
BB -> K
VB -> N
OF -> N
SH -> S
PO -> F
OC -> S
NS -> C
FH -> N
FP -> C
SO -> P
VK -> C
HP -> O
PV -> S
HN -> K
NB -> C
NV -> K
NK -> B
FN -> C
VV -> N
BN -> N
BH -> S
FO -> V
PK -> N
PS -> O
CO -> K
NO -> K
SV -> C
KO -> V
HC -> B
BC -> N
PB -> C
SK -> S
FV -> K
HO -> O
CF -> O
HB -> P
SP -> N
VH -> P
NC -> K
KC -> B
OV -> P
BK -> F
FB -> F
FF -> V
CS -> F
CC -> H
SB -> C
VO -> V
VF -> O
KP -> N
HV -> H
PF -> H
KH -> P
KS -> S
BS -> H
PH -> S
SN -> K
HK -> P
FC -> N
PN -> S
HH -> N
OB -> P
BV -> S
KF -> N
OP -> H
NF -> V
CH -> K
NH -> P'''.splitlines()


# template = 'NNCB'

# input = '''CH -> B
# HH -> N
# CB -> H
# NH -> C
# HB -> C
# HC -> B
# HN -> C
# NN -> C
# BH -> H
# NC -> B
# NB -> B
# BN -> B
# BB -> N
# BC -> B
# CC -> N
# CN -> C'''.splitlines()

rules = [line.split(' -> ') for line in input]
rules = {k: v for k, v in rules}

from collections import defaultdict

def main():
    polymer = {}
    for i in range(0, len(template), 1):
        if len(template[i:i+2]) == 2:
            if template[i:i+2] not in polymer:
                polymer[template[i:i+2]] = 1
            else:
                polymer[template[i:i+2]] += 1
    print(polymer)
    steps = 40
    for _ in range(steps):
        new_polymer = dict(polymer)
        for pair in polymer:
            if pair in rules:
                n = polymer[pair]
                new_polymer[pair] -= n
                new_pair1 = pair[0] + rules[pair]
                if new_pair1 not in new_polymer:
                    new_polymer[new_pair1] = n
                else:
                    new_polymer[new_pair1] += n
                new_pair2 = rules[pair] + pair[1]
                if new_pair2 not in new_polymer:
                    new_polymer[new_pair2] = n
                else:
                    new_polymer[new_pair2] += n
        polymer = new_polymer
        print('step', _ + 1)
    print(polymer)
    # make freq table
    freq_table = defaultdict(lambda: 0)
    for pair in polymer:
        n = polymer[pair]
        freq_table[pair[0]] += n
        freq_table[pair[1]] += n
    # for c in freq_table:
    #     freq_table[c] /= 2
    min_c = min(freq_table, key=freq_table.get)
    max_c = max(freq_table, key=freq_table.get)
    print(min_c, freq_table[min_c], max_c, freq_table[max_c], freq_table[max_c] - freq_table[min_c])

    print(min_c, freq_table[min_c], max_c, freq_table[max_c], (freq_table[max_c] / 2 - freq_table[min_c] / 2))
    

if __name__ == '__main__':
    main()