
const { LCDClient, MnemonicKey , MsgExecuteContract,Fee } = require('@terra-money/terra.js');
const fs = require('fs')

require("dotenv").config();
require('dotenv').config({ path: '../.env' })

const whitelist = require("./whitelist.json");
const token_id_list = require("./token_id.json")

//testnet
//get nft contract address
const nft_contract_address = "terra1alskwhl7x6gteuqkw7z9pexw4v9hr78mh0r6da";

const terra = new LCDClient({
    URL: 'https://terra.stakesystems.io',
    chainID: 'columbus-5'
});


async function main() {

    const result = await queryNFTInfo(nft_contract_address)
  //  console.log(result)

}

main()

async function queryNFTInfo(contract_address) {
    const result = await terra.wasm.contractQuery(
        contract_address,
        {
            "all_tokens": {
                "limit" : 10000
            }
        }
    )
    console.log(result.tokens.length)
    // for(token_id in result.token) {
    //     console.log(result.tokens[])
    // }
    return result;
}

