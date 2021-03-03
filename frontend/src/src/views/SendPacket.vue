<!--
 * @Author: your name
 * @Date: 2021-02-26 13:47:40
 * @LastEditTime: 2021-03-02 19:28:46
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit<
 * @FilePath: /buildlinks-near-redbag/src/views/SendPacket.vue
-->
<template>
  <div class="near-container">
    <div class="near-dapp loading" v-if="loading"></div>
      <div class="near-dapp near-dapp-redpacket" v-else>
        <div class="near-redpacket-header" :style="animationHeader">
          <img class="redpacket-cover" src="../assets/img/redpacket-cover.svg" alt="" />
          <button class="redpacket-btn" :style="animationBtn">拆封</button>
          <div class="redpacket-login" v-if="!isLogin">
            <div class="login active" @click="login">已有账户登录</div>
            <div class="register" @click="register">注册Near账户</div>
          </div>
        </div>
        <div class="near-redpacket-body">
          <div class="redpacket-content">
            <!-- <a v-if="registerBtn" class="btn btn-gold btn-block btn-lg" @click="goIndexToReceived" target="_blank">查看已收红包</a> -->
            <div class="redpacket-content-title">感谢您的关注</div>
            <div class="redpacket-content-subtitle">来自 NEAR 团队的祝福</div>
          </div>
          <div class="redpacket-card" :style="animationCard">
            <img class="redpacket-cover" src="../assets/img/redpacket-cover.svg" alt="" />
            <div class="redpacket-card-header">
              <div class="h2">感谢您的关注</div></div>
            <div class="redpacket-card-body">
              <div class="">金额</div>
                <div v-if="yesOrNo" class="h1">{{drawMoney}}<small>Ⓝ</small></div>
                <div v-else class="h2">{{errInfo}}</div>
            </div>
            <div class="redpacket-card-footer">
                <a v-if="true" class="btn btn-gold btn-block btn-lg" @click="goIndex" target="_blank">返回首页</a>
                <a v-else class="btn btn-gold btn-block btn-lg disabled" href="#">无法领取 NEAR</a>
            </div>
          </div>
        </div>
      </div>
      <div class="toast-wechat">
        推荐在浏览器中打开领取 ↗
      </div>
  </div>
</template>

<script>
import { initContract, login } from '../utils/utils.js'
import { KeyPair, Account, Contract } from 'near-api-js'
export default {
  name: 'SendPacket',
  data () {
    return {
      isLogin: false,
      isActive: 'active',
      drawMoney: '',
      loading: true,
      yesOrNo: false,
      registerBtn: false,
      errInfo: ''
    }
  },
  computed: {
    animationBtn () {
      return this.isLogin ? '-webkit-animation: re-flip-btn 3s ease-in-out 1;' : ''
    },
    animationHeader () {
      return this.isLogin ? '-webkit-animation: re-slide-up 3s ease-in-out 1 forwards;' : ''
    },
    animationCard () {
      return this.isLogin ? '-webkit-animation: re-slide-down 3s ease-in-out 1 forwards;' : ''
    }
  },
  methods: {
    getQueryVariable (name) {
      var reg = new RegExp('(^|&)' + name + '=([^&]*)(&|$)', 'i')
      var r = window.location.search.substr(1).match(reg)
      if (r != null) return unescape(r[2])
      return null
    },
    getSecretKeyByUrl () {
      const key = this.getQueryVariable('secretKey')
      return key
    },
    async claim () {
      try {
        this.yesOrNo = true
        const account = this.getAccount()
        const key = this.getSecretKeyByUrl()
        // if (!key) {
        //   this.$router.push('/')
        //   return
        // }
        await window.walletConnection._keyStore.setKey(
          window.nearConfig.networkId,
          window.nearConfig.contractName,
          KeyPair.fromString(key)
        )
        const contract = await new Contract(
          account,
          window.nearConfig.contractName,
          {
            changeMethods: [
              'claim'
            ],
            sender: window.nearConfig.contractName
          }
        )
        const drawMoney = await contract.claim({
          account_id: window.accountId
        })
        this.drawMoney = (drawMoney / 1e24).toFixed(2)
      } catch (err) {
        this.yesOrNo = false
        const errString = err.toString()
        if (errString.indexOf('before') !== -1) {
          this.errInfo = '已领过'
        } else {
          this.errInfo = '已领完'
        }
      }
    },
    async getWalletLink (secretkey) {
      const key = decodeURIComponent(secretkey)
      return `${window.nearConfig.walletUrl}/create/${window.nearConfig.contractName}/${key}?create=1`
    },
    getAccount () {
      return new Account(window.near.connection, window.nearConfig.contractName)
    },
    login () {
      this.isActive = 'active'
      login()
    },
    async register () {
      try {
        this.isActive = 'register'
        this.registerBtn = true
        const secreKey = this.getSecretKeyByUrl()
        const walletUrl = await this.getWalletLink(secreKey)
        window.open(walletUrl)
      } catch (err) {
        console.error(err)
      }
    },
    goIndex () {
      this.$router.push('/')
    },
    goIndexToReceived () {
      this.$router.push({
        name: 'Index',
        query: {
          active: 'claimed'
        }
      })
    }
  },
  created () {
    const that = this
    initContract()
      .then(async () => {
        if (window.walletConnection.isSignedIn()) {
          that.isLogin = false
          await this.claim()
          that.loading = false
        } else {
          that.isLogin = false
          that.loading = false
        }
      })
      .catch(console.error)
  }
}
</script>
