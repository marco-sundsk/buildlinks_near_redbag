<template>
  <transition name="el-fade-in">
    <div class="mask">
      <div class="qr-code">
        <div class="cancel" @click="cancelQRCode"></div>
        <div class="main">
          <div class="qr-wrap" style="display: block; background-color: #fafafa;" ref="qrWrap">
            <div class="title">
              <div class="logo">
                <img class="near-logo" ref="logoImg" :src="nearLogo" alt="NEAR logo"/>
              </div>
              <span>扫描二维码，接收红包</span>
            </div>
            <div class="qrccode-canvas-wrap">
              <div class="qrccode" ref="qrCodeUrl"></div>
            </div>
          </div>
          <div class="qr-img" ref="qrImg"></div>
        </div>
        <div class="url-wrap">
          <!-- <div class="url-title">链接</div> -->
          <div class="url">
            <div class="btn btn-primary" @click="doCopy">复制链接</div>
          </div>
        </div>
      </div>
    </div>
  </transition>
</template>

<script>
import QRCode from 'qrcodejs2'
import html2canvas from 'html2canvas'
export default {
  data () {
    return {
      qrcode: '',
      nearLogo: require('../assets/near-logo.svg') + `?t=${new Date().getTime()}`,
      logoFlag: true
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
      this.$refs.qrWrap.style.display = 'block'
      this.$refs.qrImg.innerText = ''
      if (this.qrcode) {
        this.$refs.qrCodeUrl.innerText = ''
      }
      if (!this.url) return
      this.qrcode = new QRCode(this.$refs.qrCodeUrl, {
        text: this.url,
        width: 200,
        height: 200,
        colorDark: '#000000',
        colorLight: '#fafafa',
        correctLevel: QRCode.CorrectLevel.M
      })
      this.$nextTick(() => {
        setTimeout(() => {
          const opts = {
            useCORS: true
          }
          html2canvas(this.$refs.qrWrap, opts)
            .then(canvas => {
              const oImg = new Image()
              oImg.src = canvas.toDataURL()
              this.$refs.qrWrap.style.display = 'none'
              this.$refs.qrImg.appendChild(oImg)
            }).catch((err) => {
              console.error(err)
            })
        }, 100)
      })
    },
    cancelQRCode () {
      this.$parent.cancelQRCode()
    },
    doCopy () {
      if (!this.url) return
      this.$copyText(this.url)
        .then(() => {
          alert('复制成功')
        })
        .catch(() => {
          alert('复制失败')
        })
    },
    createImg () {
      if (this.logoFlag) {
        var img = this.$refs.logoImg.src
        var image = new Image()
        image.setAttribute('crossOrigin', 'anonymous')
        image.src = img
        image.onload = () => {
          var canvas = document.createElement('canvas')
          canvas.width = image.width
          canvas.height = image.height
          var ctx = canvas.getContext('2d')
          ctx.drawImage(image, 0, 0, image.width, image.height)
          var ext = image.src.substring(image.src.lastIndexOf('.') + 1).toLowerCase()
          this.nearLogo = canvas.toDataURL('image/' + ext)
          this.$nextTick(() => {
            this.createQrc()
            this.logoFlag = false
          })
        }
      } else {
        this.createQrc()
      }
    }
  },
  mounted () {
    this.createImg()
  },
  watch: {
    url (newValue) {
      this.$refs.qrCodeUrl.innerText = ''
      this.$refs.qrImg.innerText = ''
    }
  }
}
</script>
