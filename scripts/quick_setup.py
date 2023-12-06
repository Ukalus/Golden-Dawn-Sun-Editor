import os, platform
import re 

# For Linux 
system_os = platform.system()

cbmdl_file_paths = []
narc_file_paths = []
cbarc_file_paths = []
unpacked_rom_path = "unpacked_rom"

def unpack_rom():
    os.system(f'ndstool -x "ROM/{os.listdir("ROM")[0]}" -d {unpacked_rom_path}')
def decompress(file_path:str):
    os.system(f'./third_party/Nintendo_DS_Compressors/lzx -d {file_path}')
def remove_bytes_until(path: str):
    # file_name = path.split("/")[-1]
    # new_file_path = path.split(file_name)[0]
    # print(new_file_path) 
    try:
        new_file: bytes
        with open(path, 'rb') as f:
            s = f.read()
            bmd0_index = s.find(b'\x42\x4D\x44\x30')
            if bmd0_index != -1:
                print(f"header found at byte: {bmd0_index}")
                new_file = s[bmd0_index:-1]
            else:
                print("No BMD0 Header found...")
        
        print("creating new file")
        with open(path, "wb") as f:
            f.write(new_file)
        print("new file created")
    except:
        print("something went wrong")

if system_os == 'Linux':
    unpack_rom()
else:
    print(f"platform ({system_os}) might not be supported, trying anyway...")
    try:
        unpack_rom()
    except:
        print(f"error, platform {system_os}")

for root, dirs, files in os.walk(unpacked_rom_path):
    for file in files:
        # if file.endswith(".cbmdl"):
        #     print(os.path.join(root, file))
        #     decompress(os.path.join(root, file))
        #     remove_bytes_until((os.path.join(root, file)))
        if file.endswith(".narc"):
            print(os.path.join(root,file))
            narc_file_paths.append(file)
        if file.endswith(".cbarc"):
            cbarc_file_paths.append(file)

print(cbmdl_file_paths)


            