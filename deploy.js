
const { LCDClient, MsgStoreCode, MnemonicKey, isTxError, MsgInstantiateContract, MsgExecuteContract } = require('@terra-money/terra.js');
const fs = require('fs')

// test1 key from localterra accounts
const mk = new MnemonicKey({
    mnemonic: 'notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius'
})

// connect to localterra
const terra = new LCDClient({
    URL: 'http://localhost:1317',
    chainID: 'localterra'
});
const wallet = terra.wallet(mk);

async function main() {


    const code_id_nft = await storeCode('rest_nft_base');

    const init_nft_msg =
    {
        "name": "Astro Herro",
        "symbol": "Astro",
        "minter": "terra1x46rqay4d3cssq8gxxvqz8xt6nwlz4td20k38v",
        "token_supply": 10000
    }

    const nft_contract_address = await initContract(code_id_nft, init_nft_msg)
    console.log("Deloy contract nft at address " + nft_contract_address)


    const code_id_minting_site = await storeCode('minting_site')

    const init_msg_minting_site =
    {
        "owner": wallet.key.accAddress.toString(),
        "treasury": wallet.key.accAddress.toString(),
        "nft_token_address": nft_contract_address.toString(),
        "collection_name": "Astro_hero",
        "collection_symbol": "Astro",
        "price": {
            "denom": "uluna",
            "amount": "10"
        }
    }
    const minting_site_address = await initContract(code_id_minting_site, init_msg_minting_site);
    console.log("Deploy contract minting site at address" + minting_site_address)



    const set_minter_msg = {
        "set_minter": {
            "minter": minting_site_address.toString()
        }
    }
    const execute_set_minter = await execute(set_minter_msg, nft_contract_address);
    console.log("Set minter at tx " + execute_set_minter)


    const mint_msg = {
        "mint_nft" : {
            "token_id": "test_token_id",
            "extension": {
                "name": "Astro #855",
                "image": "ipfs://QmSZeCiryPoUCjCgTyUxzrBF9sUheJivfAD4CsbBLW18q5",
                "attributes": [
                  {
                    "value": "dark spaceship",
                    "trait_type": "backgrounds"
                  },
                  {
                    "value": "commander",
                    "trait_type": "suits"
                  },
                  {
                    "value": "lunatic 6",
                    "trait_type": "species"
                  },
                  {
                    "value": "black eye",
                    "trait_type": "face"
                  },
                  {
                    "value": "brown mid",
                    "trait_type": "hair"
                  },
                  {
                    "value": "cool punk",
                    "trait_type": "glasses"
                  },
                  {
                    "value": "tough punk",
                    "trait_type": "headware"
                  }
                ],
                "description": "Astro are 10,921 randomly generated NFTs on the Terra blockchain."
              },
            "token_uri": "ipfs::/astrohero",
            "owner": wallet.key.accAddress
        }
    }

    const execute_mint = await execute(mint_msg, minting_site_address, { uluna: 10000000})

    console.log("Mint contract at address" + execute_mint)

}

main()

async function storeCode(path) {
    const storeCode = new MsgStoreCode(
        wallet.key.accAddress,
        fs.readFileSync(`./artifacts/${path}-aarch64.wasm`).toString('base64')
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
    const result = await terra.tx.broadcast(execute);
    return result.txhash
}