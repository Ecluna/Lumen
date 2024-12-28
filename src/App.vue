<template>
  <div class="app-container">
    <div class="toolbar">
      <button @click="openFile">打开文件</button>
      <button @click="saveFile">保存文件</button>
    </div>
    <Editor />
  </div>
</template>

<script setup>
import { open, save } from '@tauri-apps/api/dialog'
import { invoke } from '@tauri-apps/api/tauri'
import Editor from './components/Editor.vue'

const openFile = async () => {
  try {
    const selected = await open({
      filters: [{
        name: 'Markdown',
        extensions: ['md', 'markdown']
      }]
    })
    
    if (selected) {
      const content = await invoke('open_file', { path: selected })
      // TODO: 将内容传递给编辑器组件
    }
  } catch (err) {
    console.error('打开文件失败:', err)
  }
}

const saveFile = async () => {
  try {
    const filePath = await save({
      filters: [{
        name: 'Markdown',
        extensions: ['md']
      }]
    })
    
    if (filePath) {
      // TODO: 从编辑器组件获取内容并保存
      await invoke('save_file', { 
        path: filePath,
        content: '' // 需要获取编辑器内容
      })
    }
  } catch (err) {
    console.error('保存文件失败:', err)
  }
}
</script>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
}

.toolbar {
  padding: 10px;
  border-bottom: 1px solid #ddd;
}

.toolbar button {
  margin-right: 10px;
  padding: 5px 10px;
}
</style>