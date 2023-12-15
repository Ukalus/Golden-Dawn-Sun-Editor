import os, platform
import re 

# For Linux 
# For now this code is awful until i have my own narc export script because:
# 1. Tinke is giving weird file endings when unpacking 
# 2. I dont know how to but this structure inside a function without loosing reference to the filestream:
# if file.endswith(".cbmdl"):
#                     print(os.path.join(root, file))
#                     self.decompress(os.path.join(root, file))
#                     self.remove_bytes_until(os.path.join(root, file),self.remove_stamp[".cbmdl"])
#                     self.counts["formated"]["cbmdl"] = self.counts["formated"]["cbmdl"] + 1 
#                 if file.endswith(".nsbmd"):
# 3. i should check the files not by their file ending but by their magic number (why did i do this, am i some kind of windows user?)

class Project():
    system_os = platform.system()
    """
    This is a list of files that are commonly compressed in this ROM,
    It has to be checked if really all of the files need to be decompressed
    """
    compressed_file_endings = [
        ".col",
        ".inst",
        ".nbb",
        ".nsbca",
        ".nsbma",
        ".nsbmd",
        ".nsbta",
        ".cbmdl",
        ".nsbva",
    
    ]

    """
    Some files have strange header bits that are attached to the actual file,
    So we use a list of the first actual bytes of the files( Also called file signatures or magic numbers)
    """
    remove_stamp = {
       
        ".cbmdl" : b'\x42\x4D\x44\x30',
        ".nsbmd" : b'\x42\x4D\x44\x30',
        ".nsbma" : b'\x42\x4D\x41\x30',
        ".nsbca" : b'\x42\x43\x41\x30',
        ".nsbta" : b'\x42\x43\x54\x30',
        ".nsbtx" : b'\x42\x43\x58\x30',

        # ugly solution for tinke export additional "bin" file ending
        ".cbmdlbin" : b'\x42\x4D\x44\x30',
        ".nsbmdbin" : b'\x42\x4D\x44\x30',
        ".nsbmabin" : b'\x42\x4D\x41\x30',
        ".nsbcabin" : b'\x42\x43\x41\x30',
        ".nsbtabin" : b'\x42\x43\x54\x30',
        ".nsbtxbin" : b'\x42\x43\x58\x30',
        # some files are just bin?!?!? for now, just get the models
        ".bin": b'\x42\x4D\x44\x30',
    }

    counts = {
        "found": {
            "cbmdl": 0,
            "narc": 0,
            "cbarc": 0,
            "nsbmd": 0,
            "nsbma": 0,
            "nsbca": 0,
            "nsbta": 0,
            "nsbtx": 0,

            # ugly solution for tinke export additional "bin" file ending
            "cbmdlbin": 0,
            "narcbin": 0,
            "cbarcbin": 0,
            "nsbmdbin": 0,
            "nsbmabin": 0,
            "nsbcabin": 0,
            "nsbtabin": 0,
            "nsbtxbin": 0,
            "bin": 0
        },
        "formated":{
            "cbmdl": 0,
            "narc": 0,
            "cbarc": 0,
            "nsbmd": 0,
            "nsbma": 0,
            "nsbca": 0,
            "nsbta": 0,
            "nsbtx": 0,

            # ugly solution for tinke export additional "bin" file ending
            "cbmdlbin": 0,
            "narcbin": 0,
            "cbarcbin": 0,
            "nsbmdbin": 0,
            "nsbmabin": 0,
            "nsbcabin": 0,
            "nsbtabin": 0,
            "nsbtxbin": 0,
            "bin": 0 
        },
        "decompressed_files": 0,
        "unpacked_narc_files": 0,
        "bytes_removed_from_files": 0,
        "header_found": 0,
        "header_not_found": 0
    }
    # for now this is disabled because i use Tinke manually instead to unpack the files
    def unpack_rom(self):
        os.system(f'ndstool -x "ROM/{os.listdir("ROM")[0]}" -d unpacked_rom')
    # eventually i probably should stop using os.system() altogether if possible
    def decompress(self,file_path:str):
        os.system(f'./third_party/Nintendo_DS_Compressors/lzx -d {file_path}')
        
    def remove_bytes_until(self,path: str, until: bytes):
        new_file: bytes
        with open(path, 'rb') as f:
            s = f.read()
            bmd0_index = s.find(until)
            
            # should also probably skip when bmd0_index is 0, meaning that the magic number is at the start of the file, as it should be 
            # this would save one file operation for each of these occurences
            if bmd0_index != -1:
                print(f"file signature {until.decode(encoding='UTF-8')} found at offset: {bmd0_index}")
                new_file = s[bmd0_index:-1]
                with open(path, "wb") as f:
                    f.write(new_file) 
            else:
                print(f"No {until.decode(encoding='UTF-8')} Header found...")
    
    def format_files(self):
        for root, dirs, files in os.walk("unpacked_rom"):
            for file in files:
                if file.endswith(".cbmdl"):
                    print(os.path.join(root, file))
                    self.decompress(os.path.join(root, file))
                    self.remove_bytes_until(os.path.join(root, file),self.remove_stamp[".cbmdl"])
                    self.counts["formated"]["cbmdl"] = self.counts["formated"]["cbmdl"] + 1 
                if file.endswith(".nsbmd"):
                    print(os.path.join(root, file))
                    self.decompress(os.path.join(root, file))
                    self.remove_bytes_until(os.path.join(root, file),self.remove_stamp[".nsbmd"])
                    self.counts["formated"]["nsbmd"] = self.counts["formated"]["nsbmd"] + 1 
                if file.endswith(".nsbma"):
                    print(os.path.join(root, file))
                    self.decompress(os.path.join(root, file))
                    self.remove_bytes_until(os.path.join(root, file),self.remove_stamp[".nsbma"])
                    self.counts["formated"]["nsbma"] = self.counts["formated"]["nsbma"] + 1 
                if file.endswith(".nsbca"):
                    print(os.path.join(root, file))
                    self.decompress(os.path.join(root, file))
                    self.remove_bytes_until(os.path.join(root, file),self.remove_stamp[".nsbca"])
                    self.counts["formated"]["nsbca"] = self.counts["formated"]["nsbca"] + 1
                if file.endswith(".nsbta"):
                    print(os.path.join(root, file))
                    self.decompress(os.path.join(root, file))
                    self.remove_bytes_until(os.path.join(root, file),self.remove_stamp[".nsbta"])
                    self.counts["formated"]["nsbta"] = self.counts["formated"]["nsbta"] + 1
                if file.endswith(".nsbtx"):
                    print(os.path.join(root, file))
                    self.decompress(os.path.join(root, file))
                    self.remove_bytes_until(os.path.join(root, file),self.remove_stamp[".nsbtx"])
                    self.counts["formated"]["nsbtx"] = self.counts["formated"]["nsbtx"] + 1         
                
                # ugly solution for tinke export additional "bin" file ending
                # absolutely vile to copy that like this 
                if file.endswith(".cbmdlbin"):
                    print(os.path.join(root, file))
                    self.decompress(os.path.join(root, file))
                    self.remove_bytes_until(os.path.join(root, file),self.remove_stamp[".cbmdlbin"])
                    self.counts["formated"]["cbmdlbin"] = self.counts["formated"]["cbmdlbin"] + 1 
                if file.endswith(".nsbmdbin"):
                    print(os.path.join(root, file))
                    self.decompress(os.path.join(root, file))
                    self.remove_bytes_until(os.path.join(root, file),self.remove_stamp[".nsbmdbin"])
                    self.counts["formated"]["nsbmdbin"] = self.counts["formated"]["nsbmdbin"] + 1 
                if file.endswith(".nsbmabin"):
                    print(os.path.join(root, file))
                    self.decompress(os.path.join(root, file))
                    self.remove_bytes_until(os.path.join(root, file),self.remove_stamp[".nsbmabin"])
                    self.counts["formated"]["nsbmabin"] = self.counts["formated"]["nsbmabin"] + 1 
                if file.endswith(".nsbcabin"):
                    print(os.path.join(root, file))
                    self.decompress(os.path.join(root, file))
                    self.remove_bytes_until(os.path.join(root, file),self.remove_stamp[".nsbcabin"])
                    self.counts["formated"]["nsbcabin"] = self.counts["formated"]["nsbcabin"] + 1
                if file.endswith(".nsbtabin"):
                    print(os.path.join(root, file))
                    self.decompress(os.path.join(root, file))
                    self.remove_bytes_until(os.path.join(root, file),self.remove_stamp[".nsbtabin"])
                    self.counts["formated"]["nsbtabin"] = self.counts["formated"]["nsbtabin"] + 1
                if file.endswith(".nsbtxbin"):
                    print(os.path.join(root, file))
                    self.decompress(os.path.join(root, file))
                    self.remove_bytes_until(os.path.join(root, file),self.remove_stamp[".nsbtxbin"])
                    self.counts["formated"]["nsbtxbin"] = self.counts["formated"]["nsbtxbin"] + 1       
                if file.endswith(".bin"):
                    print(os.path.join(root, file))
                    self.decompress(os.path.join(root, file))
                    self.remove_bytes_until(os.path.join(root, file),self.remove_stamp[".bin"])
                    self.counts["formated"]["bin"] = self.counts["formated"]["bin"] + 1         

    # expand this function, make a more fancy display of all the stats!
    def summary(self):
        print(self.counts)
    
    def run_setup(self):
        if self.system_os == 'Linux':
            # for now this is disabled because i use Tinke manually instead to unpack the files
            # self.unpack_rom()
            self.format_files()
        else:
            print(f"platform ({self.system_os}) might not be supported, trying anyway...")
            
        self.summary()
    





Project().run_setup()



