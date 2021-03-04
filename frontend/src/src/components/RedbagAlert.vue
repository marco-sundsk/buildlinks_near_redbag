<!--
 * @Author: your name
 * @Date: 2021-03-04 12:35:51
 * @LastEditTime: 2021-03-04 13:38:49
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: /buildlinks-near-redbag/src/components/RedbagAlert.vue
-->
<template>
  <transition name="el-fade-in">
    <div class="mask">
      <div class="redbag-alert">
        <div class="empty">{{title}}</div>
        <div class="btn-wrap">
          <button v-if="alertType === 'confirm'" class="btn" @click="cancel">取消</button>
          <button class="btn btn-primary" @click="confirm">确定</button>
        </div>
      </div>
    </div>
  </transition>
</template>

<script>
export default {
  props: {
    title: {
      type: String,
      require: true
    },
    alertType: {
      type: String
    }
  },
  methods: {
    cancel () {
      this.$parent.alertHide()
    },
    confirm () {
      if (this.alertType === 'confirm') {
        this.$parent.alertHide('revoke')
      } else {
        this.$parent.alertHide()
      }
    }
  }
}
</script>
