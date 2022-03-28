
const { LCDClient, MsgStoreCode, MnemonicKey, isTxError, MsgInstantiateContract, MsgExecuteContract, } = require('@terra-money/terra.js');
const fs = require('fs').promises

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

async function main() {

    let a = []

    let nft_info = await queryContractInfo(address, nft_contract_address);

   console.log(nft_info)

}

main()

async function queryContractInfo(address, contract_address) {
    const result = await terra.wasm.contractQuery(
        contract_address,
        {
            "tokens": {
                "owner" : address,
            }
        }
    )
    return result;
}