
const { LCDClient, MsgStoreCode, MnemonicKey, isTxError, MsgInstantiateContract, MsgExecuteContract, } = require('@terra-money/terra.js');
const fs = require('fs')

require("dotenv").config();
require('dotenv').config({ path: '../.env' })

const whitelist = require("./whitelist.json");

const nft_contract_address = "terra1alskwhl7x6gteuqkw7z9pexw4v9hr78mh0r6da";

//testnet
const mnemonic = process.env.MNEMONIC;
const terra = new LCDClient({
    URL: 'https://terra.stakesystems.io',
    chainID: 'columbus-5'
});


const mk = new MnemonicKey({
    mnemonic: mnemonic
})


async function main() {


    for(i = 1; i< 3334; i++) {
        let token_id = `${i}`;
        let nft_info = await queryContractInfo(token_id, nft_contract_address);
        console.log(`Owner of token id ${i} ` + nft_info.owner)
    }

   

}

main()

async function queryContractInfo(token_id, contract_address) {
    const result = await terra.wasm.contractQuery(
        contract_address,
        {
            "owner_of": {
                "token_id" : token_id,
            }
        }
    )
    return result;
}