#!/usr/bin/env python3
import argparse
from sys import exit
from struct import unpack_from
import zlib

parser = argparse.ArgumentParser()
parser.add_argument("pngf")
parser.add_argument("--alpha", action="store_true")
args = parser.parse_args()

with open(args.pngf, "rb") as pngf:
    pngdata = pngf.read()

rawf = args.pngf.rfind(".")
rawf = args.pngf[:rawf] + ".raw"
head = pngdata.find(b"IHDR")
width = unpack_from(">I", pngdata, head+4)[0]
height = unpack_from(">I", pngdata, head+8)[0]
bitdepth = pngdata[head+12] == 8
paletted = pngdata[head+13] == 3
if not paletted:
    print("not paletted")
    exit(1)
if not bitdepth:
    print("unsupported bit depth")
    exit(1)
alphas = [255] * 256
trns = pngdata.find(b"tRNS")
if trns > 0:
    trns_len = unpack_from(">I", pngdata, trns - 4)[0]
    trns = pngdata[trns+4:trns+4+trns_len]
    alphas[:trns_len] = trns
idat = pngdata.find(b"IDAT")
if idat < 0:
    print("no IDAT chunks! WTF!?")
    exit(1)
idat_len = unpack_from(">I", pngdata, idat - 4)[0]
idat = pngdata[idat + 4:idat + 4 + idat_len]
rawdata = bytearray(zlib.decompress(idat, bufsize=(width+1)*height))
for feet in reversed(range(0, (width+1)*height, width+1)):
    # print(rawdata[feet], feet)
    del rawdata[feet]

with open(rawf, "wb") as rawf:
    for byte in rawdata:
        bite = byte
        if alphas[bite] == 0: bite = 0
        rawf.write(bytes([bite]))
        if args.alpha:
            rawf.write(bytes([alphas[byte]]))
