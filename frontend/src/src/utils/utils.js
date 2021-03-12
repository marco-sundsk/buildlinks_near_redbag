/*
 * @Author: your name
 * @Date: 2021-01-08 17:10:49
 * @LastEditTime: 2021-03-09 14:58:38
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: /swap/src/utils/utils.js
 */
import { connect, Contract, keyStores } from 'near-api-js'
import getConfig from './config'
import * as nearApi from 'near-api-js'

const nearConfig = getConfig(process.env.VUE_APP_NODE_ENV || 'development')

// Initialize contract & set global variables
export async function initContract () {
  // Initialize connection to the NEAR testnet
  const near = await connect(Object.assign({ deps: { keyStore: new keyStores.BrowserLocalStorageKeyStore() } }, nearConfig))

  // Initializing Wallet based Account. It can work with NEAR testnet wallet that
  // is hosted at https://wallet.testnet.near.org
  // window.walletConnection = new WalletConnection(near)
  // window.wallet = new nearApi.WalletConnection(window.near)
  window.getCurrentUser = async () => {
    // Needed to access wallet
    window.walletConnection = new nearApi.WalletConnection(near)
    window.walletAccount = new nearApi.WalletAccount(near)
    if (window.walletConnection.getAccountId()) {
      const accountId = window.walletConnection.getAccountId()
      window.currentUser = {
        accountId,
        account_id: accountId,
        balance: (await window.walletConnection.account().state()).amount
      }
    }
  }
  await window.getCurrentUser()

  // Getting the Account ID. If still unauthorized, it's just empty string
  window.accountId = window.walletConnection.getAccountId()

  // Initializing our contract APIs by contract name and configuration
  window.contract = await new Contract(window.walletConnection.account(), nearConfig.contractName, {
    // View methods are read only. They don't modify the state, but usually return some value.
    viewMethods: [
      'show_send_list',
      'show_recv_list',
      'show_redbag_brief',
      'show_redbag_detail',
      'show_statistic',
      'get_key_balance'
    ],
    // Change methods can modify the state. But you don't receive the returned value when called.
    changeMethods: [
      'send_redbag',
      'claim',
      'revoke'
    ]
  })
  window.baseUrl = process.env.VUE_APP_BASE_URL || 'http://47.242.35.20/redbag/'
  window.nearConfig = getConfig(process.env.VUE_APP_NODE_ENV || 'development')
  window.near = near
}

export function logout () {
  window.walletConnection.signOut()
  // reload page
  window.location.replace(window.location.origin + window.location.pathname)
}

export function login (query = 'active') {
  let url = null
  if (query === 'active') {
    url = `${window.baseUrl}?active=${query}`
  } else {
    url = null
  }
  // Allow the current app to make calls to the specified contract on the
  // user's behalf.
  // This works by creating a new access key for the user's account and storing
  // the private key in localStorage.
  window.walletConnection.requestSignIn(nearConfig.contractName, 'NEAR redbag', url)
}
