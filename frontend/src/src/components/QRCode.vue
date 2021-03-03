<!--
 * @Author: your name
 * @Date: 2021-03-01 16:22:12
 * @LastEditTime: 2021-03-01 19:42:25
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: /buildlinks-near-redbag/src/components/QRCode.vue
-->
<template>
  <transition name="el-fade-in">
    <div class="mask">
      <div class="qr-code">
        <div class="cancel" @click="cancelQRCode"></div>
        <div class="main">
          <div class="title">
            <h4>红包码</h4>
            <span>扫描二维码，接收红包</span>
          </div>
          <div class="qrccode-canvas-wrap">
            <div class="qrccode" ref="qrCodeUrl"></div>
          </div>
          <div class="url-wrap">
            <div class="url-title">链接</div>
            <div class="url">{{url}}</div>
          </div>
        </div>
      </div>
    </div>
  </transition>
</template>

<script>
import QRCode from 'qrcodejs2'
export default {
  data () {
    return {
      qrcode: ''
    }
  },
  props: {
    url: {
      type: String,
      require: true
    }
  },
  methods: {
    createQrc () {
      if (this.qrcode) {
        this.$refs.qrCodeUrl.innerText = ''
      }
      if (!this.url) return
      this.qrcode = new QRCode(this.$refs.qrCodeUrl, {
        text: this.url,
        colorDark: '#000000',
        colorLight: '#fafafa',
        correctLevel: QRCode.CorrectLevel.H
      })
    },
    cancelQRCode () {
      this.$parent.cancelQRCode()
    }
  },
  mounted () {
    this.$nextTick(function () {
      // DOM操作
      this.createQrc()
    })
  },
  watch: {
    url (newValue) {
      this.$refs.qrCodeUrl.innerText = ''
    }
  }
}
</script>
