<!--
 * @Author: your name
 * @Date: 2021-02-28 10:24:19
 * @LastEditTime: 2021-03-04 11:54:51
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: /buildlinks-near-redbag/src/components/SendRedbag.vue
-->
<template>
  <transition name="el-fade-in">
    <div class="mask">
      <div class="send-redbag-wrap">
        <div class="cancel" @click="showSendRedBag"></div>
        <div class="title">
          <img class="near-logo" src="../assets/near-logo.svg" alt="NEAR logo"/>
        </div>
        <div class="main">
            <div class="inp-wrap">
              <div class="left">
                <img :src="redbagTypeImg" alt="">
                {{redbagType3}}
              </div>
              <div class="center">
                <input type="text" v-model="money" placeholder="0.00">
              </div>
              <div class="right">Ⓝ</div>
            </div>
            <div class="redbag-type">
              <span>当前为{{redbagType1}}</span>
              <span @click="changeRedbagType">改为{{redbagType2}}</span>
            </div>
            <div class="inp-wrap">
              <div class="left">
                红包个数
              </div>
              <div class="center">
                <input type="text" v-model="redbagCount" placeholder="填写个数">
              </div>
              <div class="right">个</div>
            </div>
            <div class="redbag-type">
              <span>手续费:</span>
              <span>0.01</span>
              <span>Ⓝ/个</span>
            </div>
            <div class="inp-wrap">
              <input type="text" v-model="redbagTitle" placeholder="万事如意，恭喜发财" :maxlength="32">
            </div>
          <div class="amount-money">
            <span>{{totalMoney}}</span> <small>Ⓝ</small>
          </div>
          <div class="btn-wrap">
            <button v-if="showButton" class="btn btn-primary" @click="sendRedbag">发{{redbagType1}}</button>
            <button v-else-if="showNoButton" class="btn btn-no">余额不足</button>
            <button v-else class="btn btn-no">发{{redbagType1}}</button>
          </div>
        </div>
      </div>
    </div>
  </transition>
</template>

<script>
import { KeyPair } from 'near-api-js'
import { initContract } from '../utils/utils.js'
export default {
  data () {
    return {
      redbagState: true, // flase代表普通红包 true代表拼手气红包
      money: '',
      redbagCount: '',
      redbagTitle: '',
      pin: require('../assets/img/icon07.png'),
      pu: require('../assets/img/icon11.png'),
      nearTotal: ''
    }
  },
  computed: {
    redbagType1 () {
      return this.redbagState ? '拼手气红包' : '普通红包'
    },
    redbagTypeImg () {
      return this.redbagState ? this.pin : this.pu
    },
    redbagType2 () {
      return this.redbagState ? '普通红包' : '拼手气红包'
    },
    redbagType3 () {
      return this.redbagState ? '总金额' : '单个金额'
    },
    totalMoney () {
      if (this.redbagState && this.money) {
        return ((Number(this.money)) + (this.redbagCount * 0.01)).toFixed(2)
      } else if (!this.redbagState && this.money && this.redbagCount) {
        return ((this.money * this.redbagCount) + (this.redbagCount * 0.01)).toFixed(2)
      } else {
        return '0.00'
      }
    },
    showButton () {
      if (this.money && this.redbagCount && this.totalMoney <= this.nearTotal) {
        return true
      } else {
        return false
      }
    },
    showNoButton () {
      if (this.totalMoney <= this.nearTotal) {
        return false
      } else {
        return true
      }
    }
  },
  methods: {
    showSendRedBag () {
      this.$parent.showSendRedBag()
    },
    changeRedbagType () {
      this.redbagState = !this.redbagState
    },
    getKeyPair () {
      const keyPair = KeyPair.fromRandom('ed25519')
      const publicKey = keyPair.publicKey.toString()
      const secretKey = keyPair.secretKey
      return {
        publicKey,
        secretKey
      }
    },
    toNear (num) {
      const ratio = '000000000000000000000000'
      if (num === '0') return '0'
      if (num.indexOf('.') !== -1) {
        const arr = num.split('.')
        if (arr[0] !== 0) {
          return arr[0] + arr[1] + ratio.slice(0, (ratio.length - arr[1].length))
        } else {
          return arr[1] + ratio.slice(0, (ratio.length - arr[1].length))
        }
      } else {
        return num + ratio
      }
    },
    async getNearTotal () {
      const { total } = await window.walletConnection.account().getAccountBalance()
      this.nearTotal = (total / 1e24)
    },
    async sendRedbag () {
      try {
        const { publicKey, secretKey } = this.getKeyPair()
        const mode = this.redbagState ? 1 : 0
        const slogan = this.redbagTitle ? this.redbagTitle : '万事如意，恭喜发财'
        window.localStorage.setItem(publicKey, secretKey)
        await window.contract.send_redbag({
          public_key: publicKey,
          count: Number.parseInt(this.redbagCount),
          mode: mode,
          slogan: slogan
        }, Math.pow(10, 13).toString(), this.toNear(this.totalMoney))
      } catch (err) {
        console.error(err)
      }
    }
  },
  created () {
    const that = this
    initContract()
      .then(async () => {
        if (window.walletConnection.isSignedIn()) {
          that.getKeyPair()
          that.getNearTotal()
        }
      })
      .catch(console.error)
  },
  watch: {
    money (value) {
      if (value) {
        var newValue = value.toString()
        newValue = newValue.replace(/[^\d.]/g, '') // 清除“数字”和“.”以外的字符
        newValue = newValue.replace(/\.{2,}/g, '.') // 只保留第一个. 清除多余的
        newValue = newValue.replace('.', '$#$').replace(/\./g, '').replace('$#$', '.')
        if (newValue[0] && newValue[0] === '.') {
          newValue = '0' + newValue
        }
        this.money = newValue
      }
    },
    redbagCount (value) {
      var newValue = value.toString()
      newValue = newValue.replace(/\D/g, '') // 清除“数字”和“.”以外的字符
      newValue = newValue.replace(/^0{1,}/g, '')
      this.redbagCount = newValue
    }
  }
}
</script>
