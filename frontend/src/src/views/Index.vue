<!--
 * @Author: your name
 * @Date: 2021-02-26 11:37:19
 * @LastEditTime: 2021-03-02 18:26:57
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: /buildlinks-near-redbag/src/views/Home.vue
-->
<template>
  <div class="near-container">
    <div class="near-dapp">
      <div class="near-dapp-header">
        <div class="near-logo">
          <img class="near-logo" src="../assets/near-logo.svg" alt="NEAR logo" height="32" />
        </div>
        <div class="near-user">
            <div v-if="isLogin" class="dropdown dropdown-right" tabIndex="0">
                <div class="btn">
                  <img class="btn-icon" src="../assets/img/icon-account.svg" alt="NEAR user" height="40" />
                  <span class="text-ellipsis">{{accountId}}</span>
                </div>
                <ul class="menu">
                  <li class="menu-item">
                    <a href="#">
                      Backup NEAR Drops
                    </a>
                  </li>
                  <li class="divider"></li>
                  <li class="menu-item">
                    <a href={window.nearConfig.walletUrl} target="_blank">
                      NEAR Wallet
                    </a>
                  </li>
                  <li class="menu-item">
                    <a href="#" @click="requestSignOut">
                      Log Out
                    </a>
                  </li>
                </ul>
            </div>
        </div>
      </div>
      <div class="near-dapp-body">
        <div>
          <drop v-if="isLogin"></drop>
          <div v-else class="empty">
              <div class="empty-icon">🧧</div>
              <p class="empty-title h5">欢迎使用Near红包</p>
              <p class="empty-subtitle">Login and Send NEAR Redpackets.</p>
              <div class="empty-action">
                <div class="near-user">
                  <a class="btn" href="#" @click="requestSignIn">
                    <img class="btn-icon" src="../assets/img/icon-account.svg" alt="NEAR user" height="40" />
                    <span class="text-ellipsis">使用Near账户登录</span>
                  </a>
                </div>
              </div>
            </div>
        </div>
      </div>
      <send-redbag v-show="sendRedBag"></send-redbag>
      <q-r-code v-show="isQRCode" :url="url" ref="qrcode"></q-r-code>
      <redbag-info v-show="isRedbagInfo" :redbagInfo="redbagInfo" :redbagBrief="redbagBrief" :accountId="accountId"></redbag-info>
    </div>
  </div>
</template>

<script>
import Drop from '@/components/Drop.vue'
import SendRedbag from '@/components/SendRedbag.vue'
import QRCode from '@/components/QRCode.vue'
import RedbagInfo from '@/components/RedbagInfo.vue'
import { initContract, logout, login } from '../utils/utils.js'

export default {
  name: 'Index',
  components: {
    Drop,
    SendRedbag,
    QRCode,
    RedbagInfo
  },
  data () {
    return {
      sendRedBag: false,
      isLogin: false,
      accountId: '',
      isQRCode: false,
      url: '',
      redbagInfo: '',
      isRedbagInfo: false,
      redbagBrief: ''
    }
  },
  methods: {
    showSendRedBag () {
      this.sendRedBag = !this.sendRedBag
    },
    showQRCode (url) {
      this.isQRCode = true
      this.url = url
      this.$nextTick(() => {
        this.$refs.qrcode.createQrc()
      })
    },
    cancelQRCode () {
      this.isQRCode = false
    },
    requestSignIn () {
      if (this.$route.query.active) {
        console.log(this.$route.query.active)
        login(this.$route.query.active)
      } else {
        login()
      }
    },
    requestSignOut () {
      logout()
    },
    showRedbagInfo (info, item) {
      this.isRedbagInfo = true
      this.redbagInfo = info
      this.redbagBrief = item
    },
    cancelRedbagInfo () {
      this.isRedbagInfo = false
    }
  },
  created () {
    const that = this
    initContract()
      .then(async () => {
        if (window.walletConnection.isSignedIn()) {
          that.isLogin = true
          that.accountId = window.accountId
        } else {
          that.isLogin = false
        }
      })
      .catch(console.error)
  }
}
</script>
