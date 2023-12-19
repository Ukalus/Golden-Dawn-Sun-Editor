import struct 
import sys

# WORK IN PROGRESS

# Implemented based on http://problemkaputt.de/gbatek-ds-cartridge-nitrorom-and-nitroarc-file-systems.htm
# Previous helpful implementation https://github.com/VendorPC/NARCTool/blob/master/0.4/backend/NARC.py
# http://llref.emutalk.net/docs/?file=xml/narc.xml#xml-doc
class Narc_container():
    magic_num: bytes
    byter_order: bytes
    version: bytes
    file_size: bytes
    chunk_size: bytes
    no_following_chunks: bytes
    chunk_name: bytes
    chunk_size2: bytes 
    no_of_files: bytes
    reserved: bytes
    FAT: bytes
    FNT_magic_num: bytes
    FNT: list[bytes] = []
    chunk_size3: bytes 
    FIMG_magic_num: bytes
    FIMG_chunk_size: bytes

    def print_general_info(self):
        print(f"Type of file: {self.magic_num}")
        print(f"Byte order: {self.byter_order}")
        print(f"Byte version: {self.version}")
        print(f"File size: {self.file_size}")
        print(f"Chunk size: {int.from_bytes(self.chunk_size,'little')}")
        print(f"Number of following chunks: {self.no_following_chunks}")
        print(f"Chunk name: {self.chunk_name}")
        print(f"Chunk 2 size: {int.from_bytes(self.chunk_size2,'little')}")
        print(f"Number of files: {int.from_bytes(self.no_of_files,'little')}")
        print(f"Reserved bytes: ${self.reserved}")
        print(f"FNT magic number: ${self.FNT_magic_num}")
        print(f"File name table: ${self.FNT}")
        print(f"File image magic number: ${self.FIMG_magic_num}")
        print(f"File image chunk size: ${int.from_bytes(self.FIMG_chunk_size,'little')}")
       


def unpack_narc(path:str):
    container = Narc_container()
    btnf_index = find_btnf(path)
    with open(path, "rb") as file:
        container.magic_num = file.read(4)
        container.byter_order = file.read(2)
        container.version = file.read(2)
        container.file_size = file.read(4)
        container.chunk_size = file.read(2)
        container.no_following_chunks = file.read(2)
        container.chunk_name = file.read(4)
        container.reserved = file.read(2)

        # Note about the File Allocation table: Adresses are probably relative to the entire rom not just the NARC file so it can be ignored
        # FNT Block should contain everything we need to unpack NARC
        

        # Calculate the offset to the FNT-Block
        offset_btnf2 = int.from_bytes(container.chunk_size2,"little") - int.from_bytes(container.chunk_size,"little") + 4
        file.read(offset_btnf2)
        container.FNT_magic_num = file.read(4)
        # what are these bytes?
        file.read(12)
        
        
        for i in range(int.from_bytes(container.no_of_files,"little")):
            entry_size = file.read(1)
            container.FNT.append(file.read(int.from_bytes(entry_size,"little")))
        file.read(1)

        # this skips the possible padding that is here before the File IMG-block 
        if file.tell() % 4 != 0:
            file.read(4 - (file.tell() % 4))
        
        container.FIMG_magic_num = file.read(4)
        container.FIMG_chunk_size = file.read(4)
        
        container.file = file.read(131)
        
        
       
        
        
        


        

       
    container.print_general_info()

def find_btnf(path: str):
    with open(path, "rb") as file:
        stream = file.read()
       
        index = stream.find(b'\x42\x54\x4E\x46')
        
        if index:
            print(f"FOUND FNTB at {index}")
    return index


unpack_narc(sys.argv[1])