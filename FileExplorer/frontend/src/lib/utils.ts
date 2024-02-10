export function getHexValue(byteArray: number[]){
    let byteString: string = "0x"; 
    byteArray.forEach(element => {
        if(element == 0){
            byteString = byteString + "00"
        }
        else{
            if(element < 15){
                byteString = byteString + "0"
            }
            byteString = byteString + element.toString(16);
        }
    });
    byteString = byteString.toUpperCase()
    return byteString;
}