<!--
 * @Author: yiranzai wuqingdzx@gmail.com
 * @Date: 2024-07-12 23:01:27
 * @LastEditors: yiranzai wuqingdzx@gmail.com
 * @LastEditTime: 2024-07-12 23:47:02
 * @FilePath: /tauri-test/src/components/OpenDir.vue
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
-->
<template>  
    <div>  
      <label for="targetDir">目标目录:</label>  
      <input id="targetDir" v-model="targetDir" type="text" @blur="saveDir('targetDir')">  
      <button @click="openDir('targetDir')">浏览目录</button>  
    
      <label for="sourceDir">源目录:</label>  
      <input id="sourceDir" v-model="sourceDir" type="text" @blur="saveDir('sourceDir')">  
      <button @click="openDir('sourceDir')">浏览目录</button>  
    </div>  
  </template>  
    
  <script lang="ts">  
  import { defineComponent, ref, onMounted } from 'vue';  
  import { open } from '@tauri-apps/plugin-dialog';
    
  export default defineComponent({  
    name: 'DirectoryInputs',  
    setup() {  
      const targetDir = ref<string>('');  
      const sourceDir = ref<string>('');  
    
      const loadDirs = () => {  
        const target = localStorage.getItem('targetDir');  
        if (target) targetDir.value = target;  
        const source = localStorage.getItem('sourceDir');  
        if (source) sourceDir.value = source;  
      };  
    
      const saveDir = (key: 'targetDir' | 'sourceDir') => {  
        localStorage.setItem(key, (key === 'targetDir' ? targetDir.value : sourceDir.value));  
      };  
    
      const openDir = (key: 'targetDir' | 'sourceDir') => {  
        open({ directory: true }).then((res) => {  
          if (res) {  
            if (key === 'targetDir') targetDir.value = res;  
            else sourceDir.value = res;  
            saveDir(key);  
          }  
        }); 
      };  
    
      onMounted(loadDirs);  
    
      return { targetDir, sourceDir, saveDir, openDir };  
    },  
  });  
  </script>  
    
  <style scoped>  
  /* 样式可以根据需要添加 */  
  </style>