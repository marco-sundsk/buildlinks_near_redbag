/*
 * @Author: your name
 * @Date: 2021-02-26 11:37:19
 * @LastEditTime: 2021-03-04 11:08:02
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: /buildlinks-near-redbag/src/main.js
 */
import Vue from 'vue'
import App from './App.vue'
import router from './router'
import '@/assets/css/near.css'
import { initContract } from './utils/utils'
import moment from 'moment'
import VueClipboard from 'vue-clipboard2'
import 'vue-flipper/dist/vue-flipper.css'

Vue.use(VueClipboard)
Vue.config.productionTip = false

// ---------------filter
Vue.filter('changeNear', function (num) {
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
})

Vue.filter('changeTime', function (value) {
  const ts = Number((value / 1e6).toFixed(0))
  return moment(ts).format('YYYY-MM-DD HH:mm:ss')
})
// -------------------
new Vue({
  router,
  render: h => h(App),
  async created () {
    await initContract()
  }
}).$mount('#app')
