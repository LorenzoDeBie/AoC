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


template = 'NNCB'

input = '''CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C'''.splitlines()

rules = [line.split(' -> ') for line in input]
rules = {k: v for k, v in rules}

def get_freq_table(s: str) -> dict[str, int]:
    # using naive method to get count 
    # of each element in string 
    all_freq = {}
    
    for i in s:
        if i in all_freq:
            all_freq[i] += 1
        else:
            all_freq[i] = 1
    return all_freq

steps = 10

def main():
    print(rules)
    polymer = str(template)
    for _ in range(steps):
        i = 0
        while i + 1 < len(polymer):
            pair = polymer[i:i+2]
            if pair in rules:
                polymer = polymer[:i + 1] + rules[pair] + polymer[i + 1:]
                i += 1
            i += 1
        print('step', _ + 1)
    freq_table = get_freq_table(polymer)
    min_c = min(freq_table, key=freq_table.get)
    max_c = max(freq_table, key=freq_table.get)
    print(min_c, max_c, freq_table[max_c] - freq_table[min_c])

steps2 = 40

def main2():
    print(rules)
    polymer = list(template)
    for _ in range(steps2):
        i = 0
        while i + 1 < len(polymer):
            pair = ''.join(polymer[i:i+2])
            if pair in rules:
                polymer.insert(i + 1, rules[pair])
                i += 1
            i += 1
        print('step', _ + 1, len(polymer))
    freq_table = get_freq_table(polymer)
    min_c = min(freq_table, key=freq_table.get)
    max_c = max(freq_table, key=freq_table.get)
    print(min_c, max_c, freq_table[max_c] - freq_table[min_c])

if __name__ == '__main__':
    main()
    main2()