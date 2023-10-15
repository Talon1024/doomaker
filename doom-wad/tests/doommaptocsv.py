import struct
import csv
from collections import namedtuple
from itertools import cycle

Linedef = namedtuple("Linedef", "a b flags special tag front back")
linedef_struct = struct.Struct("<6Hh")

Sidedef = namedtuple("Sidedef", "x y upper lower middle sec")
sidedef_struct = struct.Struct("<2h8s8s8sH")

Thing = namedtuple("Thing", "x y angle ednum flags")
thing_struct = struct.Struct("<5h")

Sector = namedtuple("Sector", "florh ceilh flort ceilt light special flags")
sector_struct = struct.Struct("<2h8s8s3h")

Vertex = namedtuple("Vertex", "x y")
vertex_struct = struct.Struct("<2h")

def convert_map_data_to_csv(datafile, datacsv, tupletype, structtype):
    with    open(datafile, "rb") as mapdatafile, \
            open(datacsv, "w", newline="") as mapdatacsv:
        csvwriter = csv.DictWriter(mapdatacsv, tupletype._fields)
        csvwriter.writeheader()
        for x in cycle([structtype.size]):
            datum = mapdatafile.read(x)
            if len(datum) < x:  # EOF
                break
            datum = tupletype(*structtype.unpack(datum))
            csvwriter.writerow(datum._asdict())

# Export the LINEDEFS, SIDEDEFS, THINGS, SECTORS, and VERTEXES lumps of a Doom map using SLADE
convert_map_data_to_csv("/tmp/LINEDEFS.lmp", "/tmp/linedefs.csv", Linedef, linedef_struct)
convert_map_data_to_csv("/tmp/SIDEDEFS.lmp", "/tmp/sidedefs.csv", Sidedef, sidedef_struct)
convert_map_data_to_csv("/tmp/THINGS.lmp", "/tmp/things.csv", Thing, thing_struct)
convert_map_data_to_csv("/tmp/SECTORS.lmp", "/tmp/sectors.csv", Sector, sector_struct)
convert_map_data_to_csv("/tmp/VERTEXES.lmp", "/tmp/vertices.csv", Vertex, vertex_struct)
