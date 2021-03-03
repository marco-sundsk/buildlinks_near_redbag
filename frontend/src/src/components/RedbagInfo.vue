<!--
 * @Author: your name
 * @Date: 2021-03-01 19:27:38
 * @LastEditTime: 2021-03-02 19:41:47
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: /buildlinks-near-redbag/src/components/RedbagInfo.vue
-->
<template>
  <transition name="el-fade-in">
    <div class="mask">
      <div class="redbag-info-wrap">
        <div class="cancel" @click="cancelRedbagInfo"></div>
        <div class="title">
          <img class="near-logo" src="../assets/near-logo.svg" alt="NEAR logo"/>
        </div>
        <div class="master-info">
          <div class="master-name">
            <img src="../assets/img/head.png" alt="">
            <span>{{redbagInfo.owner}}的{{getRedbagType}}</span>
          </div>
          <div class="slogan-info">{{redbagInfo.slogan}}</div>
        </div>
        <div class="line"></div>
        <div class="user-wrap">
          <div class="top">
            {{geCount}}, 共{{(redbagInfo.balance - redbagInfo.remaining_balance) | changeNear}}/{{redbagInfo.balance | changeNear}} Ⓝ
          </div>
          <div class="user-main">
            <div class="user-item" :class="{active: item.user === accountId}" v-for="(item, index) in claimInfo" :key="index">
              <div class="user-info">
                <img src="../assets/img/icon04.png" alt="">
                <span>{{item.user}}</span>
              </div>
              <div class="near">{{item.amount | changeNear}} Ⓝ</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </transition>
</template>

<script>
export default {
  data () {
    return {
    }
  },
  props: {
    redbagInfo: {
      require: true
    },
    redbagBrief: {
      require: true
    },
    accountId: {
      require: true
    }
  },
  computed: {
    getRedbagType () {
      return Number(this.redbagInfo.mode) === 1 ? '拼手气红包' : '普通红包'
    },
    claimInfo () {
      if (this.redbagInfo.claim_info) {
        const list = this.redbagInfo.claim_info
        list.sort((a, b) => b.ts - a.ts)
        return list
      } else {
        return []
      }
    },
    reCount () {
      if (this.redbagInfo.claim_info) {
        return this.redbagInfo.claim_info.length
      } else {
        return ''
      }
    },
    geCount () {
      if ((this.redbagInfo.balance - this.redbagInfo.remaining_balance) === Number(this.redbagInfo.balance)) {
        return '已全部领取'
      } else {
        return `已领取${this.reCount}/${this.redbagInfo.count}个`
      }
    },
    getBalance () {
      if ((this.redbagInfo.balance - this.redbagInfo.remaining_balance) === Number(this.redbagInfo.balance)) {
        return `${this.redbagInfo.balance}`
      } else {
        return `已领取${this.reCount}/${this.redbagInfo.count}个`
      }
    }
  },
  methods: {
    cancelRedbagInfo () {
      this.$parent.cancelRedbagInfo()
    }
  },
  filters: {
    changeNear (value) {
      if (!value) return 0
      return (value / 1e24).toFixed(2)
    }
  }
}
</script>
