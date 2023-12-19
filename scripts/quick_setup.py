import os, platform
import re 

# For Linux 
# For now this code is awful until i have my own narc export script because:
# 1. Tinke is giving weird file endings when unpacking 
# 3. i should check the files not by their file ending but by their magic number (why did i do this, am i some kind of windows user?)

class Project():
    system_os = platform.system()
    """
    This is a list of files that are commonly compressed in this ROM,
    It has to be checked if really all of the files need to be decompressed
    """
    compressed_file_endings = [
        "col",
        "inst",
        "nbb",
        "nsbca",
        "nsbma",
        "nsbmd",
        "nsbta",
        "nsbtx",
        "cbmdl",
        "nsbva",
    
    ]

    """
    Some files have strange header bits that are attached to the actual file,
    So we use a list of the first actual bytes of the files( Also called file signatures or magic numbers)
    """
    remove_stamp = {
       
        "cbmdl" : b'\x42\x4D\x44\x30',
        "nsbmd" : b'\x42\x4D\x44\x30',
        "nsbma" : b'\x42\x4D\x41\x30',
        "nsbca" : b'\x42\x43\x41\x30',
        "nsbta" : b'\x42\x43\x54\x30',
        "nsbtx" : b'\x42\x43\x58\x30',

        # ugly solution for tinke export additional "bin" file ending
        "cbmdlbin" : b'\x42\x4D\x44\x30',
        "nsbmdbin" : b'\x42\x4D\x44\x30',
        "nsbmabin" : b'\x42\x4D\x41\x30',
        "nsbcabin" : b'\x42\x43\x41\x30',
        "nsbtabin" : b'\x42\x43\x54\x30',
        "nsbtxbin" : b'\x42\x43\x58\x30',
        # some files are just bin?!?!? for now, just get the models
        "bin": b'\x42\x4D\x44\x30',
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
    def unpack_rom(self, path:str = "ROM"):
        os.system(f'ndstool -x "ROM/{os.listdir("ROM")[0]}" -d unpacked_rom')
    # eventually i probably should stop using os.system() altogether if possible
    def decompress(self,file_path:str):
        os.system(f'./third_party/Nintendo_DS_Compressors/lzx -d {file_path}')
        
    def remove_bytes_until(self,path: str, until: bytes):
        new_file: bytes
        with open(path, 'rb') as f:
            s = f.read()
            bmd0_index = s.find(until)
            # -1 means no header 0 means bytes have already been removed 
            if bmd0_index != -1 and bmd0_index != 0:
                print(f"file signature {until.decode(encoding='UTF-8')} found at offset: {bmd0_index}")
                new_file = s[bmd0_index:-1]
                with open(path, "wb") as f:
                    f.write(new_file)
                     
    def process_file(self,filepath: str):
        file_ending = next((file_ending for file_ending in self.compressed_file_endings if filepath.endswith(file_ending)), None)
        if file_ending != None:
            self.decompress(filepath)    
        if file_ending in self.remove_stamp:
            self.remove_bytes_until(filepath,self.remove_stamp[file_ending])
            self.counts["formated"][file_ending] = self.counts["formated"][file_ending] + 1   
    def format_files(self, path: str = "unpacked_rom"):
        for root, dirs, files in os.walk(path):
            for file in files:
                self.process_file(os.path.join(root, file))

    # expand this function, make a more fancy display of all the stats!
    def summary(self):
        print(self.counts)
    
    def run_setup(self):
        if self.system_os == 'Linux':
            # for now this is disabled because i use Tinke manually instead to unpack the files
            self.unpack_rom()
            self.format_files()
        else:
            print(f"platform ({self.system_os}) might not be supported, trying anyway...")
            
        self.summary()
    





Project().run_setup()



