<template>
  <div class="app-container">
    <div class="toolbar">
      <button class="toolbar-button" @click="toggleFileManager">
        <span class="icon">📁</span>
      </button>
      <button class="toolbar-button" @click="openFile">打开文件</button>
      <button class="toolbar-button" @click="saveFile">保存文件</button>
      <span class="file-status" v-if="currentFile">
        <span class="status-dot" :class="{ 'unsaved': hasUnsavedChanges }"></span>
        {{ currentFileName }}
      </span>
    </div>
    <div class="main-content">
      <FileManager 
        ref="fileManagerRef"
        :current-file="currentFile"
        :has-external-changes="hasExternalChanges"
        @file-selected="handleFileSelected"
        v-show="showFileManager"
        :class="{ 'file-manager-hidden': !showFileManager }"
      />
      <Editor 
        ref="editorRef"
        @content-changed="handleContentChanged"
        :class="{ 'editor-full': !showFileManager }"
      />
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api'
import { dialog, notification } from '@tauri-apps/api'
import { basename } from '@tauri-apps/api/path'
import Editor from './components/Editor.vue'
import FileManager from './components/FileManager.vue'

const editorRef = ref(null)
const fileManagerRef = ref(null)
const currentFile = ref(null)
const hasUnsavedChanges = ref(false)
const hasExternalChanges = ref(false)
const currentFileName = ref('')
const showFileManager = ref(true)

// 处理内容变更
const handleContentChanged = () => {
  hasUnsavedChanges.value = true
}

// 更新当前文件名
const updateCurrentFileName = async () => {
  if (currentFile.value) {
    currentFileName.value = await basename(currentFile.value)
  } else {
    currentFileName.value = '未命名'
  }
}

// 处理文件选择
const handleFileSelected = async ({ path, content }) => {
  if (hasUnsavedChanges.value) {
    const confirmed = await dialog.ask('有未保存的更改，是否继续？', {
      title: '确认',
      type: 'warning'
    })
    if (!confirmed) return
  }

  currentFile.value = path
  editorRef.value?.setContent(content)
  await invoke('add_recent_file', { path })
  await fileManagerRef.value?.refreshFiles()
  hasUnsavedChanges.value = false
  hasExternalChanges.value = false
  await updateCurrentFileName()
}

// 处理外部文件变更
const handleExternalChanges = async () => {
  hasExternalChanges.value = true
  await notification.sendNotification({
    title: '文件已被修改',
    body: `文件 ${currentFileName.value} 已被外部程序修改`,
    icon: 'warning'
  })
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
        await fileManagerRef.value?.refreshFiles()
        await updateCurrentFileName()
      }
      hasUnsavedChanges.value = false
      hasExternalChanges.value = false
      await notification.sendNotification({
        title: '保存成功',
        body: `文件 ${currentFileName.value} 已保存`,
        icon: 'success'
      })
    }
  } catch (err) {
    console.error('保存文件失败:', err)
  }
}

// 切换文件管理器显示
const toggleFileManager = () => {
  showFileManager.value = !showFileManager.value
  // 保存用户偏好
  localStorage.setItem('showFileManager', showFileManager.value.toString())
}

// 初始化时读取用户偏好
onMounted(() => {
  const savedPreference = localStorage.getItem('showFileManager')
  if (savedPreference !== null) {
    showFileManager.value = savedPreference === 'true'
  }
})
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

.file-status {
  display: inline-flex;
  align-items: center;
  margin-left: 16px;
  font-size: 14px;
  color: #666;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  margin-right: 8px;
  background-color: #4caf50;
}

.status-dot.unsaved {
  background-color: #ff9800;
}

.file-status.has-external-changes {
  color: #f44336;
}

.icon {
  font-size: 16px;
  line-height: 1;
}

/* 添加过渡动画 */
.file-manager {
  transition: transform 0.3s ease, width 0.3s ease;
}

.file-manager-hidden {
  transform: translateX(-100%);
  width: 0;
  padding: 0;
  margin: 0;
  overflow: hidden;
}

.editor-full {
  width: 100%;
}

/* 修改工具栏按钮样式 */
.toolbar-button:first-child {
  padding: 6px 8px;
  margin-right: 16px;
}

.toolbar-button:first-child:hover {
  background-color: #e8e8e8;
}

.toolbar-button:first-child.active {
  background-color: #0366d6;
  color: white;
  border-color: #0366d6;
}
</style>