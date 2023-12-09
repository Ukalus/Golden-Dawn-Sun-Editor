import os, platform
import re 

# For Linux 

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
        "cbmdl" : b'\x42\x4D\x44\x30',
        ".nsbmd" : b'\x42\x4D\x44\x30',
        ".nsbma" : b'\x42\x4D\x41\x30',
        ".nsbca" : b'\x42\x43\x41\x30',
        ".nsbta" : b'\x42\x43\x54\x30',
        ".nsbtx" : b'\x42\x43\x58\x30'
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
        },
        "decompressed_files": 0,
        "unpacked_narc_files": 0,
        "bytes_removed_from_files": 0,
        "header_found": 0,
        "header_not_found": 0
    }
    def unpack_rom(self):
        os.system(f'ndstool -x "ROM/{os.listdir("ROM")[0]}" -d unpacked_rom')
    def decompress(self,file_path:str):
        os.system(f'./third_party/Nintendo_DS_Compressors/lzx -d {file_path}')
        
    def remove_bytes_until(self,path: str, until: bytes):
        new_file: bytes
        with open(path, 'rb') as f:
            s = f.read()
            bmd0_index = s.find(until)
                
            if bmd0_index != -1:
                print(f"file signature {until.decode(encoding='UTF-8')} found at offset: {bmd0_index}")
                new_file = s[bmd0_index:-1]
                with open(path, "wb") as f:
                    f.write(new_file) 
            else:
                print(f"No {until.decode(encoding='UTF-8')} Header found...")
    def handle_file(self,file:str, filetype:str,root:str):
        if file.endswith(file):
                print(os.path.join(root, file))
                self.decompress(os.path.join(root, file))
                self.remove_bytes_until(os.path.join(root, file),self.remove_stamp[filetype])
                self.counts["formated"][filetype] = self.counts["formated"][filetype] + 1 

    def format_files(self):
        for root, dirs, files in os.walk("unpacked_rom"):
            for file in files:
                self.handle_file(".cbmdl","cbmdl",root)
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
    def summary(self):
        print(self.counts)
    
    def run_setup(self):
        if self.system_os == 'Linux':
            self.unpack_rom()
            self.format_files()
        else:
            print(f"platform ({self.system_os}) might not be supported, trying anyway...")
            try:
                self.unpack_rom()
            except:
                print(f"error, platform {self.system_os}")
        self.summary()
    





Project().run_setup()



