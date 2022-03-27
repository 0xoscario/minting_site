const { RawKey, LCDClient, BankAPI } = require('@terra-money/terra.js');

const privateKey = "8005175d8a09354e46f27925bdab42b0bef66d1ae58597f20e2453726e14d5c5TMVEcqD"
const CryptoJS = require("crypto-js")

// const rk = new RawKey(Buffer);

// console.log(rk.privateKey);

const terra = new LCDClient({
    chainID: "columbus-5",
    URL: "https://lcd.terra.dev",
  });


  const exportedWallet = JSON.parse(
    Buffer.from(
      "eyJuYW1lIjoibG9jYWx0ZXJyYTEiLCJhZGRyZXNzIjoidGVycmExZGNlZ3lyZWtsdHN3dnl5MHh5Njl5ZGd4bjl4OHgzMnpkdGFwZDgiLCJlbmNyeXB0ZWRfa2V5IjoiYThjMDg1ZTIyZjYwZmU5ZDljYzU5NGZkMWM5NjEwMjlmZDJhOGY2Y2NkYjA0MjQ5NWY3MmE2MDI1YjA3MmYyZjlJWXgybHhPcTU5Q3lJNDBEa1dXMEtkRzNLZjlnM3NrM3M5eFkwZStVRnZ0Z3VMYnAzd2QyYTFuUWVyS1hOcndveFlMQ29EUU1lTlk5NktHdSszNXh6alA3NUE3TnBTYVhRWUFqK1JNY0lBPSJ9",
      "base64"
    ).toString("utf8")
  );


  const decryptedKey = decrypt(exportedWallet.encrypted_key, "localterra");
   console.log(Buffer.from(decryptedKey, "hex"))
   console.log(Buffer.from("8005175d8a09354e46f27925bdab42b0bef66d1ae58597f20e2453726e14d5c5TMVEcqD", "hex"))

  const wallet2 = terra.wallet(new RawKey(Buffer.from(decryptedKey, "hex")));
    

  async function main(){

let balance = await terra.bank.balance(wallet2.key.accAddress);
console.log(balance[0]._coins)
  }

  main()
  wallet2
  console.log(decryptedKey);




function decrypt(transitmessage, pass) {
    const salt = CryptoJS.enc.Hex.parse(transitmessage.substr(0, 32));
    const iv = CryptoJS.enc.Hex.parse(transitmessage.substr(32, 32));
    const encrypted = transitmessage.substring(64);
  
    const keySize = 256;
    const iterations = 100;
    const key = CryptoJS.PBKDF2(pass, salt, {
      keySize: keySize / 32,
      iterations: iterations,
    });
  
    return CryptoJS.AES.decrypt(encrypted, key, {
      iv: iv,
      padding: CryptoJS.pad.Pkcs7,
      mode: CryptoJS.mode.CBC,
    }).toString(CryptoJS.enc.Utf8);
  }