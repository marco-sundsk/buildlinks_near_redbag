(function() {
    // const CONTRACT_NAME = 'buildlinks_pool'; /* TODO: fill this in! */
    // const CONTRACT_NAME = 'aaaabbbb1.betanet';
    // const CONTRACT_NAME = 'buildlinks';
    // const CONTRACT_NAME = 'blredbag.testnet';
    // const CONTRACT_NAME = 'buildlinksapp.near';
    const CONTRACT_NAME = 'redbag.near';
    // const CONTRACT_NAME = 'contract_user_contract_user_contract_user';
    const DEFAULT_ENV = 'mainnet';
    //15
    const GAS = Math.pow(10,13).toString();

    function getConfig(env) {
        switch (env) {

            case 'production':
            case 'development':
                return {
                    networkId: 'betanet',                                             // this can be any label to namespace user accounts
                    nodeUrl: "https://rpc.betanet.near.org",                          // this endpoint must point to the network you want to reach
                    walletUrl: "https://wallet.betanet.near.org",                      // this endpoint must exist for the wallet to work
                    contractName: CONTRACT_NAME,
                    deps: {
                        keyStore: new nearApi.keyStores.BrowserLocalStorageKeyStore() // keys are stored as plaintext in LocalStorage
                    }
                };
            case 'mainnet':
                return {
                    networkId: 'mainnet',                                             // this can be any label to namespace user accounts
                    nodeUrl: "https://rpc.mainnet.near.org",                          // this endpoint must point to the network you want to reach
                    // walletUrl: "http://47.242.35.20",                      // this endpoint must exist for the wallet to work
                    walletUrl: 'https://near-wallet.buildlinks.org/',
                    contractName: CONTRACT_NAME,
                    deps: {
                        keyStore: new nearApi.keyStores.BrowserLocalStorageKeyStore() // keys are stored as plaintext in LocalStorage
                    }
                };
            case 'testnet':
                return {
                    networkId: 'testnet',                                             // this can be any label to namespace user accounts
                    nodeUrl: "https://rpc.testnet.near.org",                          // this endpoint must point to the network you want to reach
                    // walletUrl: "https://wallet.testnet.near.org/",                      // this endpoint must exist for the wallet to work
                    walletUrl: "http://near-wallet.buildlinks.org",
                    contractName: CONTRACT_NAME,
                    deps: {
                        keyStore: new nearApi.keyStores.BrowserLocalStorageKeyStore() // keys are stored as plaintext in LocalStorage
                    }
                };
            case 'mine':
                return {
                    networkId: 'betanet',                                             // this can be any label to namespace user accounts
                    nodeUrl: "http://192.168.2.199:1234",                          // this endpoint must point to the network you want to reach
                    walletUrl: "http://192.168.2.199:1234",                      // this endpoint must exist for the wallet to work
                    contractName: CONTRACT_NAME,
                    deps: {
                        keyStore: new nearApi.keyStores.BrowserLocalStorageKeyStore() // keys are stored as plaintext in LocalStorage
                    }
                };
            case 'local':
                return {
                    networkId: 'local',
                    nodeUrl: 'https://rpc.testnet.near.org',
                    // keyPath: `${process.env.HOME}/.near/validator_key.json`,
                    walletUrl: 'http://localhost:1234',
                    contractName: CONTRACT_NAME,
                    deps: {
                        keyStore: new nearApi.keyStores.BrowserLocalStorageKeyStore() // keys are stored as plaintext in LocalStorage
                    }
                };
            default:
                throw Error(`Unconfigured environment '${env}'. Can be configured in src/config.js.`);
        }
    }

    window.nearConfig =  getConfig(DEFAULT_ENV);
    window.gas = GAS;
})();

const radix=Math.pow(10,24);
const base_url = 'http://47.242.35.20/redbag';
const create_url = 'http://47.242.35.20/';
// const create_url = 'http://18.166.189.212/';
// const create_url = 'http://near-wallet.buildlinks.org/';
// const base_url = 'http://18.166.189.212/redbag/';
// const base_url = 'http://near-wallet.buildlinks.org/redbag/';
// const base_url = 'http://192.168.1.3:8091/';

