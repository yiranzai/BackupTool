<!--
 * @Author: yiranzai wuqingdzx@gmail.com
 * @Date: 2024-07-12 20:25:47
 * @LastEditors: yiranzai wuqingdzx@gmail.com
 * @LastEditTime: 2024-07-12 23:03:39
 * @FilePath: /tauri-test/src/App.vue
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
-->
<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import HelloWorld from './components/HelloWorld.vue'
import OpenDir from './components/OpenDir.vue'
</script>

<template>
  <div class="flex flex-col p-2">
    <div class="flex flex-row items-center justify-center">
      <img src="/tauri.svg" class="w-20 h-20 logo tauri" alt="Tauri logo" />
      <img src="/vue.svg" class="w-20 h-20 logo vue" alt="Vue logo" />
    </div>

    <HelloWorld msg="Hello Tauri + Vue!" />

    <OpenDir msg=" Hello OpenDir " />
  </div>
</template>
