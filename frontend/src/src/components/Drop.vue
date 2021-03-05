<!--
 * @Author: your name
 * @Date: 2021-02-26 13:53:52
 * @LastEditTime: 2021-03-05 17:45:25
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: /buildlinks-near-redbag/src/components/Drops.vue
-->
<template>
  <div>
    <div class="loading" v-if="loading"></div>
    <div v-else>
      <div class="near-balance">
        <div class="near-balance-title">余额</div>
        <div class="near-balance-funds">{{nearTotal | changeNear}} <small>Ⓝ</small></div>
        <div class="near-balance-actions">
          <button class="btn btn-primary" @click="showSendRedBag">+点击创建红包</button>
        </div>
      </div>
      <div class="near-tabs">
        <ul class="tab">
          <li class='tab-item' :class="{active: isActive === 'active'}" @click="changeActive('active')">发送</li>
          <li class='tab-item' :class="{active: isActive === 'claimed'}" @click="changeActive('claimed')">接收</li>
        </ul>
      </div>
      <div class="loading" v-if="publicLoading" style="position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%);"></div>
    <div>
      <div v-if="isActive === 'active'" class="near-drops">
        <div v-if="listLoading" class="loading"></div>
          <div v-else-if="activeList.length > 0" class="drop">
              <div class="near-drop-item" v-for="(item, index) in activeList" :key="index" @click.stop="showRedbagInfo(item)">
                  <div class="drop-item-funds">
                    {{(item.balance - item.remaining_balance) | changeNear}} / {{item.balance | changeNear}}<small>Ⓝ</small>
                    <span>{{item.received_count}}/{{item.count}} <small>个</small></span>
                  </div>
                  <div class="drop-item-status">{{redbagState(item)}}</div>
                  <div class="drop-item-pubkey text-ellipsis text-gray">发送时间: {{item.ts | changeTime}}</div>
                  <button class="btn btn-sm btn-primary" @click.stop="showUrlInfo(item.id)">发红包</button>
                  <button v-if="Number(item.remaining_balance) !== 0" class="btn btn-sm btn-link" @click.stop="revokeAlertShow(item.id)">撤回</button>
              </div>
          </div>
          <div v-else class="empty">
              <div class="empty-icon">🧧</div>
              <p class="empty-title h5">无红包记录</p>
              <p class="empty-subtitle">点击上方红色按钮创建红包</p>
          </div>
      </div>
    </div>
      <div v-if="isActive === 'claimed'">
        <div v-if="listLoading" class="loading"></div>
        <div class="drop" v-else-if="claimedList.length > 0">
            <div class="near-drop-item" v-for="(item, index) in claimedList" :key="index" @click.stop="showRedbagInfo(item)">
                <div class="drop-item-funds">{{item.balance | changeNear}} <small>Ⓝ</small></div>
                <!-- <div class="drop-item-status"></div> -->
                <div class="drop-item-pubkey text-ellipsis text-gray">领取时间: {{item.ts | changeTime}}</div>
            </div>
        </div>
        <div v-else class="empty">
            <div class="empty-icon">🧧</div>
            <p class="empty-title h5">无红包记录</p>
            <p class="empty-subtitle">点击上方红色按钮创建红包</p>
        </div>
      </div>
    </div>
    <redbag-alert v-if="revokeAlert" title="确认撤回该红包?" alertType="confirm"></redbag-alert>
    <redbag-alert v-if="shareAlert" title="请使用创建该红包的浏览器进行分享"></redbag-alert>
  </div>
</template>

<script>
import RedbagAlert from '@/components/RedbagAlert.vue'
export default {
  components: {
    RedbagAlert
  },
  data () {
    return {
      publicLoading: false,
      loading: true,
      isActive: 'active',
      activeList: [],
      claimedList: [],
      nearTotal: '',
      shareAlert: false,
      revokeAlert: false,
      currentPk: '',
      listLoading: false
    }
  },
  computed: {
    redbagState () {
      return (item) => {
        if (Number(item.remaining_balance) === 0 || item.received_count === item.count) {
          return '已领完'
        } else {
          return '领取中'
        }
      }
    },
    getBalance () {
      return (item) => {
        const balance = (item.balance - item.remaining_balance) ? (item.balance - item.remaining_balance) : 0
        return balance
      }
    }
  },
  methods: {
    alertHide (state = '') {
      if (state === 'revoke') this.revoke(this.currentPk)
      this.shareAlert = false
      this.revokeAlert = false
    },
    revokeAlertShow (id) {
      this.currentPk = id
      this.revokeAlert = true
    },
    showSendRedBag () {
      this.$parent.showSendRedBag()
    },
    async changeActive (type) {
      if (type === 'active') {
        this.listLoading = true
        await this.getSendList()
      } else {
        this.listLoading = true
        await this.getRecvList()
      }
      this.listLoading = false
      this.isActive = type
    },
    async getSendList () {
      try {
        const list = await window.contract.show_send_list({
          account_id: window.accountId
        })
        list.sort((a, b) => {
          return b.ts - a.ts
        })
        this.activeList = list
      } catch (err) {
        console.error(err)
      }
    },
    async getRecvList () {
      try {
        const list = await window.contract.show_recv_list({
          account_id: window.accountId
        })
        list.sort((a, b) => {
          return b.ts - a.ts
        })
        this.claimedList = list
      } catch (err) {
        console.error(err)
      }
    },
    getQueryVariable (name) {
      var reg = new RegExp('(^|&)' + name + '=([^&]*)(&|$)', 'i')
      var r = window.location.search.substr(1).match(reg)
      if (r != null) return unescape(r[2])
      return null
    },
    async getNearTotal () {
      try {
        const { total } = await window.walletConnection.account().getAccountBalance()
        this.nearTotal = total
      } catch (err) {
        console.error(err)
      }
    },
    async showUrlInfo (id) {
      const secretKey = window.localStorage.getItem(id)
      if (secretKey) {
        this.publicLoading = true
        const info = await window.contract.show_redbag_detail({
          public_key: id
        })
        this.publicLoading = false
        this.$parent.showQRCode(`${window.baseUrl}#/sendPacket?secretKey=${secretKey}&publicKey=${id}`, info)
      } else {
        this.shareAlert = true
      }
    },
    async showRedbagInfo (item) {
      try {
        this.publicLoading = true
        const info = await window.contract.show_redbag_detail({
          public_key: item.id
        })
        this.publicLoading = false
        this.getSendList()
        this.$parent.showRedbagInfo(info, item)
      } catch (err) {
        console.error(err)
      }
    },
    // 撤销红包
    async revoke (id) {
      try {
        this.loading = true
        await window.contract.revoke({
          public_key: id
        })
        this.loading = false
      } catch (err) {
        console.error(err)
      }
      this.getSendList()
    }
  },
  filters: {
    changeNear (value) {
      if (!value) return 0
      return (value / 1e24).toFixed(2)
    }
  },
  async created () {
    if (this.getQueryVariable('active')) {
      this.isActive = this.getQueryVariable('active')
    }
    await this.getSendList()
    await this.getRecvList()
    await this.getNearTotal()
    this.loading = false
  }
}
</script>
