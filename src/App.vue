<template>
  <div class="app-container">
    <div class="toolbar">
      <button class="toolbar-button" @click="openFile">打开文件</button>
      <button class="toolbar-button" @click="saveFile">保存文件</button>
    </div>
    <div class="main-content">
      <FileManager 
        ref="fileManagerRef"
        :current-file="currentFile" 
        @file-selected="handleFileSelected" 
      />
      <Editor ref="editorRef" />
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api'
import { dialog } from '@tauri-apps/api'
import Editor from './components/Editor.vue'
import FileManager from './components/FileManager.vue'

const editorRef = ref(null)
const fileManagerRef = ref(null)
const currentFile = ref(null)

// 处理文件选择
const handleFileSelected = async ({ path, content }) => {
  currentFile.value = path
  editorRef.value?.setContent(content)
  await invoke('add_recent_file', { path })
  await fileManagerRef.value?.refreshFiles()  // 刷新文件列表
}

const openFile = async () => {
  try {
    const selected = await dialog.open({
      filters: [{
        name: 'Markdown',
        extensions: ['md', 'markdown']
      }]
    })
    
    if (selected) {
      const content = await invoke('open_file', { path: selected })
      await handleFileSelected({ path: selected, content })
    }
  } catch (err) {
    console.error('打开文件失败:', err)
  }
}

const saveFile = async () => {
  try {
    const filePath = currentFile.value || await dialog.save({
      filters: [{
        name: 'Markdown',
        extensions: ['md']
      }]
    })
    
    if (filePath) {
      const content = editorRef.value?.getContent() || ''
      await invoke('save_file', { 
        path: filePath,
        content
      })
      if (!currentFile.value) {
        currentFile.value = filePath
        await invoke('add_recent_file', { path: filePath })
        await fileManagerRef.value?.refreshFiles()  // 刷新文件列表
      }
    }
  } catch (err) {
    console.error('保存文件失败:', err)
  }
}
</script>

<style>
/* 重置默认样式 */
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, Cantarell, sans-serif;
}

.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background-color: #ffffff;
}

.toolbar {
  padding: 10px 20px;
  border-bottom: 1px solid #e0e0e0;
  background-color: #f5f5f5;
}

.toolbar-button {
  margin-right: 10px;
  padding: 6px 12px;
  border: 1px solid #d1d1d1;
  border-radius: 4px;
  background-color: #ffffff;
  color: #333;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s ease;
}

.toolbar-button:hover {
  background-color: #f0f0f0;
  border-color: #b1b1b1;
}

.toolbar-button:active {
  background-color: #e8e8e8;
}

.main-content {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.editor-container {
  flex: 1;
}
</style>