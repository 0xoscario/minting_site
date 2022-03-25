
const { LCDClient, MsgStoreCode, MnemonicKey, isTxError, MsgInstantiateContract, MsgExecuteContract, } = require('@terra-money/terra.js');
const fs = require('fs')

require("dotenv").config();
require('dotenv').config({ path: '../.env' })

const whitelist = require("./whitelist.json");


//testnet
const mnemonic = process.env.MNEMONIC;
const terra = new LCDClient({
    URL: 'https://bombay.stakesystems.io',
    chainID: 'bombay-12'
});

const mk = new MnemonicKey({
    mnemonic: mnemonic
})

const wallet = terra.wallet(mk);


async function main() {


    const code_id_nft = await storeCode('astro_nft_base');


    //init message
    const init_nft_msg =
    {
        "name": "Astro Hero",
        "symbol": "Astro",
        "minter": wallet.key.accAddress,
        "token_supply": 10000
    }

    const nft_contract_address = await initContract(code_id_nft, init_nft_msg)

    console.log("Deloy contract nft at address " + nft_contract_address)


    const result = await execute_mint(nft_contract_address)
    
    console.log(result);

    // const query_msg = {
    //     "nft_info": {
    //         "token_id": "astro_test" ,
    //     }
    // }

    // let nft_info = await query(query_msg, nft_contract_address);
    // console.log(nft_info)




}

main()

async function storeCode(path) {
    const storeCode = new MsgStoreCode(
        wallet.key.accAddress,
        fs.readFileSync(`../artifacts/${path}-aarch64.wasm`).toString('base64')
    );
    const storeCodeTx = await wallet.createAndSignTx({
        msgs: [storeCode],
    });
    const storeCodeTxResult = await terra.tx.broadcast(storeCodeTx);

    if (isTxError(storeCodeTxResult)) {
        throw new Error(
            `store code failed. code: ${storeCodeTxResult.code}, codespace: ${storeCodeTxResult.codespace}, raw_log: ${storeCodeTxResult.raw_log}`
        );
    }

    const {
        store_code: { code_id },
    } = storeCodeTxResult.logs[0].eventsByType;
    console.log(`Store contract ${path} with code id = ${code_id}`)

    return code_id;

}

async function initContract(code_id, init_msg) {
    //init 
    const instantiate = new MsgInstantiateContract(
        wallet.key.accAddress,
        wallet.key.accAddress,
        code_id[0], // code ID
        init_msg
        // { uluna: 10000000, ukrw: 1000000 }
    );

    const instantiateTx = await wallet.createAndSignTx({
        msgs: [instantiate],
    });

    const instantiateTxResult = await terra.tx.broadcast(instantiateTx);

    const {
        instantiate_contract: { contract_address },
    } = instantiateTxResult.logs[0].eventsByType;

    return contract_address
}


async function query(message, contract_address) {
    const result = await terra.wasm.contractQuery(
        contract_address,
        {
            "nft_info": {
                "token_id": "test_token_id"
            }
        }
    )
    return result;
}

async function execute_mint(contract_address, coin) {

    const mint_msg = {
        "mint": {
            "owner": "terra1zp7h2vjvqaj37a5ttp9j09dsp6xuudqkfh533h",
            "token_id": "astro_test",
            "token_uri": "ipfs::/astrohero_1" ,
            "extension": {
                    "name": "AH 0002",
                    "faction": "Vandals",
                    "attributes": [
                        {
                            "trait_type": "Background",
                            "value": "Terra Planet"
                        },
                        {
                            "trait_type": "ShadowCharacter",
                            "value": ""
                        },
                        {
                            "trait_type": "Shoes",
                            "value": "Astronaut White Shoes"
                        },
                        {
                            "trait_type": "Right",
                            "value": "Arm Standard Purple"
                        },
                        {
                            "trait_type": "Trunk",
                            "value": "Cyborg Trunk (Orange v2)"
                        },
                        {
                            "trait_type": "Suit",
                            "value": "Pandora Pink suit"
                        },
                        {
                            "trait_type": "Left",
                            "value": "HERO Cyborg Arm"
                        },
                        {
                            "trait_type": "Weapon",
                            "value": "Cyber Baseball Bat"
                        },
                        {
                            "trait_type": "Weapon2",
                            "value": "Metal Shield"
                        },
                        {
                            "trait_type": "Head",
                            "value": "Hunter  Vandals (Gold - Black)"
                        },
                        {
                            "trait_type": "HAT",
                            "value": "Astronut Helmet White"
                        }
                    ]
                }
            },
        }

    const msg = new MsgExecuteContract(
        wallet.key.accAddress,
        contract_address.toString(),
        mint_msg,
        coin

    )
    const execute = await wallet.createAndSignTx({
        msgs: [msg]
    })
    const result = await terra.tx.broadcast(execute);
    return result.txhash
}