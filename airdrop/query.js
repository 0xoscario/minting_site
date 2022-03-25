
const { LCDClient, MsgStoreCode, MnemonicKey, isTxError, MsgInstantiateContract, MsgExecuteContract, } = require('@terra-money/terra.js');
const fs = require('fs')

require("dotenv").config();
require('dotenv').config({ path: '../.env' })

const whitelist = require("./whitelist.json");

const nft_contract_address = "terra1zfmppqden5n7n5evumd6l58hdszjlsn88ysfz8";

//testnet
const mnemonic = process.env.MNEMONIC;
const terra = new LCDClient({
    URL: 'https://bombay.stakesystems.io',
    chainID: 'bombay-12'
});



const mk = new MnemonicKey({
    mnemonic: mnemonic
})


async function main() {


    // for(i = 1; i< 12; i++) {
    //     let token_id = "astro_" + `${i}`;
    //     let nft_info = await queryNFTInfo(token_id, nft_contract_address);
    //     console.log("Token id " + token_id)
    //     console.log(nft_info)
    // }

   let nft_info = await queryNFTInfo("astro_8", nft_contract_address)
   console.log(nft_info)
   

}

main()

async function queryNFTInfo(token_id, contract_address) {
    const result = await terra.wasm.contractQuery(
        contract_address,
        {
            "nft_info": {
                "token_id": token_id
            }
        }
    )
    return result;
}

async function queryContractInfo(contract_address) {
    const result = await terra.wasm.contractQuery(
        contract_address,
        {
            "contract_info": {
           
            }
        }
    )
    return result;
}