#!/usr/bin/env python3

from typing import Tuple
from math import ceil, prod
from functools import reduce

sample0 = "D2FE28"
sample1 = "38006F45291200"
sample2 = "EE00D40C823060"
sample3 = "8A004A801A8002F478"
sample4 = "620080001611562C8802118E34"
sample5 = "C0015000016115A2E0802F182340"
sample6 = "A0016C880162017C3686B18A3D4780"

samples = [sample0, sample1, sample2, sample3, sample4, sample5, sample6]

class Packet:
    # return number of bits read
    def __init__(self, encoded: str, base=16) -> None:
        # convert encoded string to base2 if needed
        if base == 16:
            bin_packet = Packet.__hex_to_binary(encoded)
        elif base == 2:
            bin_packet = encoded
        else:
            raise ValueError('Wrong base!')
        # parse version and type id
        self.version = int(bin_packet[:3], 2)
        self.type_id = int(bin_packet[3:6], 2)
        # keep track of how many bits this packet is long
        # for now this is 6 (the version and type id)
        self.length = 6

        # now we start parsing the body of the packet
        # literal value
        if self.type_id == 4:
            value, n_groups = Packet.__parse_literal(bin_packet[6:])
            self.value = value
            self.length += n_groups * 5
            # self.length = 4 * ceil(self.length/4)
        # operator -> and subpackets
        else:
            # next bit gives type length id
            self.length_type_id = int(bin_packet[6])
            # packet is now one bit longer
            self.length += 1
            # two different ways to encode the length of embedded packets
            # 0 -> 15 bits that represent the length in bits of all the subpackets
            if self.length_type_id == 0:
                # pase the length in bits of the subpacket
                embedded_packets_length = int(bin_packet[7:22], 2)
                # increase the length of this packet by 15
                self.length += 15
                # keep track of length of subpackets
                n_bits_read = 0
                # keep all subpackets in list
                self.value = []
                # keep parsing subpackets while we can
                while n_bits_read < embedded_packets_length:
                    # parse next packet
                    next_packet = Packet(bin_packet[22 + n_bits_read:], 2)
                    # keep track of the length
                    n_bits_read += len(next_packet)
                    # add subpacket to the list of values
                    self.value.append(next_packet)
                # make sure we have read and parsed the correct amount
                assert embedded_packets_length == n_bits_read, 'embedded packets length != n_bits_read'
                # add the length to this packet as well
                self.length += n_bits_read
            # 1 -> 11 bits represent the number of subpackets
            elif self.length_type_id == 1:
                # parse number of subpackets
                n_subpackets = int(bin_packet[7:18], 2)
                # add 11 bits to our length
                self.length += 11
                # keep track of length of subpackets
                n_bits_read = 0
                # keep all subpackets in list
                self.value = []
                # parse n subpackets
                for _ in range(n_subpackets):
                    # parse the next subpacket
                    next_packet = Packet(bin_packet[18 + n_bits_read:], 2)
                    # keep track of the length
                    n_bits_read += len(next_packet)
                    # add subpacket to list of values
                    self.value.append(next_packet)
                # add the length to this packet as well
                self.length += n_bits_read

    @staticmethod
    def __hex_to_binary(hex_packet: str) -> str:            
        # ''.join([bin(int(c, 16))[2:] for c in hex_packet])
        return ''.join([format(int(c, 16), '04b') for c in hex_packet])
    
    @staticmethod
    def __parse_literal(encoded_literal: str) -> Tuple[int, int]:
        n_groups = 0
        literal_bits = ''
        group = encoded_literal[:5]
        while group[0] == '1':
            n_groups += 1
            literal_bits += group[1:]
            group = encoded_literal[5 * n_groups:5 * (n_groups + 1)]
        # last group
        n_groups += 1
        literal_bits += group[1:]
        return (int(literal_bits, 2), n_groups)

    def get_sum_of_version_numbers(self) -> int:
        if self.type_id == 4:
            return self.version
        else:
            res = self.version
            for packet in self.value:
                res += packet.get_sum_of_version_numbers()
            return res
    
    def calculate_expression(self) -> int:
        # litteral value
        if self.type_id == 4:
            return self.value
        # no literal value -> get all child expressions
        values = [x.calculate_expression() for x in self.value]
        # sum
        if self.type_id == 0:
            return sum(values)
        # product
        elif self.type_id == 1:
            return prod(values)
        # min
        elif self.type_id == 2:
            return min(values)
        # max
        elif self.type_id == 3:
            return max(values)
        # gt
        elif self.type_id == 5:
            return int(values[0] > values[1])
        # lt
        elif self.type_id == 6:
            return int(values[0] < values[1])
        # eq
        elif self.type_id == 7:
            return int(values[0] == values[1])
        

    def __len__(self):
        return self.length

def main():
    packets = []
    for sample in samples:
        packet = Packet(sample)
        packets.append(packet)
        print(sample, packet.get_sum_of_version_numbers())
    part1 = Packet("420D5A802122FD25C8CD7CC010B00564D0E4B76C7D5A59C8C014E007325F116C958F2C7D31EB4EDF90A9803B2EB5340924CA002761803317E2B4793006E28C2286440087C5682312D0024B9EF464DF37EFA0CD031802FA00B4B7ED2D6BD2109485E3F3791FDEB3AF0D8802A899E49370012A926A9F8193801531C84F5F573004F803571006A2C46B8280008645C8B91924AD3753002E512400CC170038400A002BCD80A445002440082021DD807C0201C510066670035C00940125D803E170030400B7003C0018660034E6F1801201042575880A5004D9372A520E735C876FD2C3008274D24CDE614A68626D94804D4929693F003531006A1A47C85000084C4586B10D802F5977E88D2DD2898D6F17A614CC0109E9CE97D02D006EC00086C648591740010C8AF14E0E180253673400AA48D15E468A2000ADCCED1A174218D6C017DCFAA4EB2C8C5FA7F21D3F9152012F6C01797FF3B4AE38C32FFE7695C719A6AB5E25080250EE7BB7FEF72E13980553CE932EB26C72A2D26372D69759CC014F005E7E9F4E9FA7D3653FCC879803E200CC678470EC0010E82B11E34080330D211C663004F00101911791179296E7F869F9C017998EF11A1BCA52989F5EA778866008D8023255DFBB7BD2A552B65A98ECFEC51D540209DFF2FF2B9C1B9FE5D6A469F81590079160094CD73D85FD2699C5C9DCF21F0700094A1AC9EDA64AE3D37D34200B7B401596D678A73AFB2D0B1B88057230A42B2BD88E7F9F0C94F1ECB7B0DD393489182F9802D3F875C00DC40010F8911C61F8002111BA1FC2E400BEA5AA0334F9359EA741892D81100B83337BD2DDB4E43B401A800021F19A09C1F1006229C3F8726009E002A12D71B96B8E49BB180273AA722468002CC7B818C01B04F77B39EFDF53973D95ADB5CD921802980199CF4ADAA7B67B3D9ACFBEC4F82D19A4F75DE78002007CD6D1A24455200A0E5C47801559BF58665D80")
    print(part1.get_sum_of_version_numbers())


def main2():
    part2_samples = ['C200B40A82', '04005AC33890', '880086C3E88112', 'CE00C43D881120', 'D8005AC2A8F0', 'F600BC2D8F', '9C005AC2F8F0', '9C0141080250320F1802104A08']
    for sample in part2_samples:
        print(sample, Packet(sample).calculate_expression())
    part2 = Packet("420D5A802122FD25C8CD7CC010B00564D0E4B76C7D5A59C8C014E007325F116C958F2C7D31EB4EDF90A9803B2EB5340924CA002761803317E2B4793006E28C2286440087C5682312D0024B9EF464DF37EFA0CD031802FA00B4B7ED2D6BD2109485E3F3791FDEB3AF0D8802A899E49370012A926A9F8193801531C84F5F573004F803571006A2C46B8280008645C8B91924AD3753002E512400CC170038400A002BCD80A445002440082021DD807C0201C510066670035C00940125D803E170030400B7003C0018660034E6F1801201042575880A5004D9372A520E735C876FD2C3008274D24CDE614A68626D94804D4929693F003531006A1A47C85000084C4586B10D802F5977E88D2DD2898D6F17A614CC0109E9CE97D02D006EC00086C648591740010C8AF14E0E180253673400AA48D15E468A2000ADCCED1A174218D6C017DCFAA4EB2C8C5FA7F21D3F9152012F6C01797FF3B4AE38C32FFE7695C719A6AB5E25080250EE7BB7FEF72E13980553CE932EB26C72A2D26372D69759CC014F005E7E9F4E9FA7D3653FCC879803E200CC678470EC0010E82B11E34080330D211C663004F00101911791179296E7F869F9C017998EF11A1BCA52989F5EA778866008D8023255DFBB7BD2A552B65A98ECFEC51D540209DFF2FF2B9C1B9FE5D6A469F81590079160094CD73D85FD2699C5C9DCF21F0700094A1AC9EDA64AE3D37D34200B7B401596D678A73AFB2D0B1B88057230A42B2BD88E7F9F0C94F1ECB7B0DD393489182F9802D3F875C00DC40010F8911C61F8002111BA1FC2E400BEA5AA0334F9359EA741892D81100B83337BD2DDB4E43B401A800021F19A09C1F1006229C3F8726009E002A12D71B96B8E49BB180273AA722468002CC7B818C01B04F77B39EFDF53973D95ADB5CD921802980199CF4ADAA7B67B3D9ACFBEC4F82D19A4F75DE78002007CD6D1A24455200A0E5C47801559BF58665D80")
    print(part2.calculate_expression())

if __name__ == '__main__':
    main()
    main2()