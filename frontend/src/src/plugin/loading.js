/*
 * @Author: your name
 * @Date: 2021-03-01 21:21:29
 * @LastEditTime: 2021-03-01 21:21:40
 * @LastEditors: Please set LastEditors
 * @Description: In U/*
 * @Author: your name
 * @Date: 2020-12-24 15:49:18
 * @LastEditTime: 2020-12-24 15:49:35
 * @LastEditors: Please set LastEditors
 * @Description: In User Settings Edit
 * @FilePath: /d2-admin-start-kit/src/plugin/loading/index.js
 */
import { Loading } from 'element-ui'

let loadingCount = 0
let loading

const startLoading = () => {
  loading = Loading.service({
    lock: true,
    text: '加载中……',
    background: 'rgba(0, 0, 0, 0.2)'
  })
}

const endLoading = () => {
  loading.close()
}

export const showLoading = () => {
  if (loadingCount === 0) {
    startLoading()
  }
  loadingCount += 1
}

export const hideLoading = () => {
  if (loadingCount <= 0) {
    return
  }
  loadingCount -= 1
  if (loadingCount === 0) {
    endLoading()
  }
}
