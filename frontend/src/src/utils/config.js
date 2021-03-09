/*
 * @Author: your name
 * @Date: 2021-01-08 17:10:49
 * @LastEditTime: 2021-03-09 12:21:52
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: /near-swap-front/src/utils/config.js
 */
const CONTRACT_NAME = process.env.VUE_APP_CONTRACT_NAME || 'rb01.testnet'
const MAINNET_CONTRACT_NAME = process.env.VUE_APP_MAINNET_CONTRACT_NAME || 'biudrops.near'
const MAINNET_WALLET_URL = process.env.VUE_APP_MAINNET_WALLET_URL || 'https://near-wallet.buildlinks.org'
const MAINNET_NODE_URL = process.env.MAINNET_NODE_URL || 'https://rpc.mainnet.near.org'
const TESTNET_CONTRACT_NAME = process.env.VUE_APP_TESTNET_CONTRACT_NAME || 'rb01.testnet'
const TESTNET_WALLET_URL = process.env.VUE_APP_TESTNET_WALLET_URL || 'http://47.242.35.20'
const TESTNET_NODE_URL = process.env.VUE_APP_TESTNET_NODE_URL || 'https://rpc.testnet.near.org'

function getConfig (env) {
  switch (env) {
    case 'production':
    case 'mainnet':
      return {
        networkId: 'mainnet',
        nodeUrl: MAINNET_NODE_URL,
        contractName: MAINNET_CONTRACT_NAME,
        walletUrl: MAINNET_WALLET_URL,
        helperUrl: 'https://helper.mainnet.near.org',
        explorerUrl: 'https://explorer.mainnet.near.org'
      }
    case 'development':
    case 'testnet':
      return {
        networkId: 'testnet',
        nodeUrl: TESTNET_NODE_URL,
        contractName: TESTNET_CONTRACT_NAME,
        walletUrl: TESTNET_WALLET_URL,
        helperUrl: 'https://helper.testnet.near.org',
        explorerUrl: 'https://explorer.testnet.near.org'
      }
    case 'betanet':
      return {
        networkId: 'betanet',
        nodeUrl: 'https://rpc.betanet.near.org',
        contractName: CONTRACT_NAME,
        walletUrl: 'https://wallet.betanet.near.org',
        helperUrl: 'https://helper.betanet.near.org',
        explorerUrl: 'https://explorer.betanet.near.org'
      }
    case 'local':
      return {
        networkId: 'local',
        nodeUrl: 'http://localhost:3030',
        keyPath: `${process.env.HOME}/.near/validator_key.json`,
        walletUrl: 'http://localhost:4000/wallet',
        contractName: CONTRACT_NAME
      }
    case 'test':
    case 'ci':
      return {
        networkId: 'shared-test',
        nodeUrl: 'https://rpc.ci-testnet.near.org',
        contractName: CONTRACT_NAME,
        masterAccount: 'test.near'
      }
    case 'ci-betanet':
      return {
        networkId: 'shared-test-staging',
        nodeUrl: 'https://rpc.ci-betanet.near.org',
        contractName: CONTRACT_NAME,
        masterAccount: 'test.near'
      }
    default:
      throw Error(`Unconfigured environment '${env}'. Can be configured in src/config.js.`)
  }
}

module.exports = getConfig
