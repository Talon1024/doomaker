#!/usr/bin/env python3
import argparse
from sys import exit
from struct import unpack_from
import zlib

parser = argparse.ArgumentParser()
parser.add_argument("pngf", help="The PNG file")
parser.add_argument("--alpha", action="store_true", help="IndexedAlpha format")
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
idat = bytearray()
idatc = pngdata.find(b"IDAT")
while idatc != -1:
    idat_len = unpack_from(">I", pngdata, idatc - 4)[0]
    idat += pngdata[idatc + 4:idatc + 4 + idat_len]
    idatc += idat_len
    idatc = pngdata.find(b"IDAT", idatc)
if len(idat) == 0:
    print("No IDAT chunks! WTF?!")
    exit(1)
rawdata = bytearray(zlib.decompress(idat, bufsize=(width+1)*height))
for rowstart in reversed(range(0, (width+1)*height, width+1)):
    # print(rawdata[rowstart], rowstart)
    del rawdata[rowstart]

with open(rawf, "wb") as rawf:
    for byte in rawdata:
        bite = byte
        if alphas[bite] == 0: bite = 0
        rawf.write(bytes([bite]))
        if args.alpha:
            rawf.write(bytes([alphas[byte]]))
