import os
import time

playername = "Lucas24"

def findIndex():
    
    with open("golden_sun_save_beginning.sav", "rb") as file:
        while True:
            
            byte = file.read(1)
            
            number = int.from_bytes(byte)
            
            if chr(number) == playername[0]:
                print("First Letter was found \""+ chr(number)+" \":" + str(file.tell()))
                file.seek(-1, 1)
                outputStr = ""
                for i in range(0,14) :
                    _charArr = file.read(1)
                    
                    for c in _charArr:
                        print(int(c))
                        if c != b"0x00":
                            outputStr = outputStr + chr(c)
                print(outputStr)
                
                
                if str(outputStr) == str(playername):
                    print("matched PlayerName!")
                        
                    
                
                
                
                input()
        

findIndex()
