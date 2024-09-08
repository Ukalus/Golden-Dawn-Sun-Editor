![Frame1.png](Frame1.png)

A Editor for Golden Sun: Dark Dawn 


## Apicula

![Apicula](https://github.com/scurest/apicula) is an Application for viewing NDS models written in Rust. The Backend of this Application is written in Rust aswell, so maybe i could take some parts of it to display files in my editor (If its OpenSource, have to check).
Apicula displays most of the Models, Textures and Sprites and Animations correctly, there is currently a bug where some models are rendered weirdly when a certain Draw command is used. Here is the Issue: 

https://github.com/scurest/apicula/issues/22

## Tinke

Tinke is a Tool for unpacking a ROM written in C#. Some Folders and Files inside the rom are "archived" multiple times in the NARC format.
By unpacking the folder multiple times it is possible to extract the currently biggest list of files as far as i know ( current highest number of viewable models: 4270).

![Tinke](https://github.com/pleonex/tinke)

## LZX

Some Files are compressed using LZ11 this tool can decompress these files 

## NDS Tool

https://github.com/PeterLemon/Nintendo_DS_Compressors

A collection of decompression tools, we need LZX to decompress some files that have are LZ40 compressed. you can identify these files by their first two bytes "0x40" 

# NDS HEADER

```mermaid
packet-beta
title NDS HEADER
0-11: "GT"
12-15: "GC"
16-17: "MC"
18: "UC"
19: "ESS"
20: "DVC"
21-28: "Reserved"
29: "REG"
30: "VER"
31: "AS"
32-35: "ARM9-offset"
36-39: "ARM9-entry"
40-43: "ARM9-RAM"
44-47: "ARM9-size"
48-51: "ARM7-offset"
52-55: "ARM7-entry"
56-59: "ARM7-RAM"
60-63: "ARM7-size"
64-67: "FNT-offset"
68-71: "FNT-size"
72-75: "FAT-offset"
76-79: "FAT-size"
80-83: "ARM9-OV-offset"
84-87: "ARM9-OV-size"
88-91: "ARM7-OV-offset"
92-95: "ARM7-OV-size"
96-99: "Normal commands"
100-103: "KEY1 commands"
104-107: "Icon offset"
108-109: "SAC"
110-111: "SAD"
112-115: "ARM9 Auto Load"
116-119: "ARM7 Auto Load"
120-127: "Secure Area Disable"
128-131: "Total Used ROM size"
132-135: "ROM Header Size"
136-139: "Unknown"
140-147: "Reserved"
148-149: "NEORA"
150-151: "NSORWA"
152-175: "Reserved"
176-191: "Reserved"
192-347: "Nintendo Logo (compressed bitmap, same as in GBA Headers)"
348-349: "NLCSM"
350-351: "HDCSM"
352-355: "DBG-offset"
356-359: "DBG-size"
360-363: "DBG-ram-addr"
364-367: "Reserved"
368-511: "Reserved (zero filled) (transferred, but not stored in RAM)"
512-543: "Reserved (zero filled) (usually not transferred) length: 512-4095"
```
Special Thanks to Martin Korth for all the information availiable at: http://problemkaputt.de


This Diagram is just for general overview more info ![here](http://problemkaputt.de/gbatek-ds-cartridge-header.htm)

## FAT Table
```mermaid
---
title: "FAT Table"
---
packet-beta
    0-3: "Start address"
    4-7: "Address address"
    8-11: "Start address"
    12-15: "Address address"
    16-19: "Start address"
    20-23: "Address address"
    24-27: "Start address"
    28-31: "Address address"
    32-35: "Start address"
    36-39: "Address address"
    40-43: "..."
    44-47: "..."
```
## FNT Table

```mermaid
---
title: "FNT Table"
---
packet-beta
    0-3: "offset_table_entry"
    4-5: "Id_1st_File"
    6-7: "t_num_file"
    8-11: "offset_table_entry"
    12-13: "Id_1st_File"
    14-15: "Id_parent"
    16-19: "offset_table_entry"
    20-21: "Id_1st_File"
    22-23: "Id_parent"
    24-27: "offset_table_entry"
    28-29: "Id_1st_File"
    30-31: "Id_parent"
    32-35: "offset_table_entry"
    36-37: "Id_1st_File"
    38-39: "Id_parent"
    40-43: "..."
    44-45: "..."
    46-47: "..."
```
## How to Read The Entire File System?

```mermaid
flowchart LR
    A[(FNT Table)]
    B[(FAT Table)]
    C[(FNT Subtable)]

    d[FNT Entry]
    e[FNT SubEntry]
    f[FAT Entry]

    D{Is File?}

    A --> d --> C
    C --> e --> D

    D -->|No, its a Directory| A
    D -->|Yes, its a file| B --> f

```

## Helpful Commands

Unpacking a NDS rom 

    ndstool -x file.nds -d ./unpacked

Removing bytes from file (1131 bytes in example)

    dd bs=1 skip=1131 if=filtered.dump of=trimmed.dump

Viewing (most) nintendo 3D/Texture/Animation files

    apicula view <filename,foldername>
View all files in a directory tree using apicula 

    apicula view $(find ./unpacked_rom -type f)

Decompress .cbmdl files (LZX using compression)

    lzx -D filename.cbmdl

Get all file extensions

    find ./data -type f | perl -ne 'print $1 if m/\.([^.\/]+)$/' | sort -u

Get all paths of file with certain extension

    find ./unpacked -type f -name \*.narc
## File Types


### All file Types

|        |        |        |        |        |
|--------|--------|--------|--------|--------|
|.bin    |.cam    |.cbarc  |.cbin   |.cbmdl  |
|.col    |.chr    |.dat    |.dat1   |.dat2   |
|.dat3   |.dat4   |.dat5   |.dat6   |.dat7   |
|.inc    |.inst   |.mdlr   |.narc   |.nbb    |
|.nsbca  |.nsbma  |.nsbmd  |.nsbsd  |.nsbta  |
|.nsbtp  |.nsbtx  |.nsbva  |.sdat   |.sim    |
|.spr    |.xsadl  |

## File Glossary


### .cbmdl

- Compressed file containing Models Textures (maybe Animations?)
- Compressed with LZX (lzx decompression tool works here).
- After decompressing All bytes at the beginning of the file until BMD0 have to be removed 
- deleting the bytes might lead to weird deformations of the model but still works (apicula recognizes it)s
- Can then be viewed/exported with apicula  

### .cbin

- are often paired up with .cbmdl files
- maybe additional data for rendering? 
- maybe stands for compressed binary?
- might be data on where objects are spawned? ./unpacked_rom/flmodel/djinn02.narc/djinn02.narc/djinn02.narc
./unpacked_rom/flmodel/chibi_col_04.narc/niru.narc/niru.narc

### .narc

- Widely used nintendo format
- Stands for Nintendo-archive (files have to be extracted)
- can be extracted with Apicula?! (is nowhere mentioned in the docs i think)
- It needs to be checked if Apicula actually exports everything or only parts (.cbark files are skipped for example)
- can also be extracted with tinke

### .cbark

- Its a NARC Archive file that has compressed (some) files inside!
- some of the files have the same compression used in .cbmdl files (LZ40)
- Some Models have weird Proportions (like very a small head) <- still has to be figured out why
