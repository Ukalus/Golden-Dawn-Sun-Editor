export function getHexValue(byteArray: number[], decimal: boolean = false){
    let byteStr: string = "";
    byteArray.forEach(element => {
        if(element == 0){
            byteStr = byteStr + "00"
        }
        else{
            if(element < 15){
                byteStr = byteStr + "0"
            }
            byteStr = byteStr + element.toString(16);
        }
    });
    byteStr = "0x" + byteStr.toUpperCase()
    return decimal ? Number(byteStr): byteStr;
    
}