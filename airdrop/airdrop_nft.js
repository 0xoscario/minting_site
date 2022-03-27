
const { LCDClient, MnemonicKey , MsgExecuteContract,Fee } = require('@terra-money/terra.js');
const fs = require('fs')

require("dotenv").config();
require('dotenv').config({ path: '../.env' })

const whitelist = require("./whitelist.json");
const token_id_list = require("./token_id.json")

//testnet
const mnemonic = process.env.MNEMONIC;
//get nft contract address
const nft_contract_address = process.env.NFT_CONTRACT_ADDRESS;

const terra = new LCDClient({
    URL: 'https://bombay.stakesystems.io',
    chainID: 'bombay-12'
});


const mk = new MnemonicKey({
    mnemonic: mnemonic
})

const wallet = terra.wallet(mk);


async function main() {

    console.log("Airdrop NFT start" )

    const length_tokenid = token_id_list.token_id.length; 
    const length_address = whitelist.address.length; 

    if(length_address != length_tokenid ) {throw new Error("Must same length between token id and whitelist")};
    for (i = 0; i < length_address; i++) {
        let transfer_nft_msg = {
            "transfer_nft" : {
                "recipient" : whitelist.address[i],
                "token_id" : token_id_list.token_id[i]
            }
        }
        let result = await execute_airdrop(transfer_nft_msg, nft_contract_address);
        console.log(`${i}. Transfer token id ${token_id_list.token_id[i]} for address ${whitelist.address[i]} at tx ${result.txhash} `)

    }

}

main()

async function execute_airdrop(message, contract_address, coin) {
    const msg = new MsgExecuteContract(
        wallet.key.accAddress,
        contract_address.toString(),
        message,
        coin

    )
    const result = await wallet.createAndSignTx({
        msgs: [msg],
        gasPrices: { uusd: 0.15 },
    }).then(tx => terra.tx.broadcastSync(tx))
        .then(async result => {
            while (true) {
                const data = await terra.tx.txInfo(result.txhash)
                    .catch(() => { })
                // if hash is onchain return data
                if (data) return data
                // else wait 250ms and then repeat
                await new Promise(resolve => setTimeout(resolve, 250))
            }
        })

    return result
}



async function checkBalance() {
    
}
