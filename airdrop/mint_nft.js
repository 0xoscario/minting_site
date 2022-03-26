
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



//local terra
// const mnemonic = "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius"
// // test1 key from localterra accounts
// const terra = new LCDClient({
//     URL: "http://localhost:1317",
//     chainID: "localterra",
//   });


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

    for (i = 1; i < 13; i++) {
        let metadata = fs.readFileSync(`./metadata/${i}.json`);

        let extension = JSON.parse(metadata);
        const mint_msg = {
            "mint": {
                "owner": wallet.key.accAddress,
                "token_id": "astro_" + `${i}`,
                "token_uri": "ipfs::/astrohero_" + `${i}`,
                "extension": extension
            }
        }


        await execute(mint_msg, nft_contract_address)
        console.log("Mint for address " + i + " token id " + "astro_" + `${i}`);
    

    }

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

async function execute(message, contract_address, coin) {
    const msg = new MsgExecuteContract(
        wallet.key.accAddress,
        contract_address.toString(),
        message,
        coin

    )
    const execute = await wallet.createAndSignTx({
        msgs: [msg]
    })
    const result = await terra.tx.broadcastSync(execute);

    let timedOut = false
    const timeout = setTimeout(() => {
        timedOut = true
    }, 2*60*1000)

    let included = false
    while (true) {
        if (timedOut) break

        const data = await terra.tx.txInfo(result.txhash).catch(() => null)
        if (!data) {
            await new Promise(resolve => setTimeout(resolve, 1000))
            continue
        }

        included = true
        break
    }
    if (!included) throw new Error(`Transaction (${result.txhash}) not included in a block.`)
    clearTimeout(timeout)


    return result.txhash
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

