<template>
  <div class="app-container">
    <div class="toolbar">
      <!-- 左侧按钮组 -->
      <div class="toolbar-left">
        <!-- 新建文件按钮 -->
        <button class="toolbar-btn" @click="createNewFile" title="新建文件">
          <svg viewBox="0 0 16 16" width="16" height="16">
            <path fill="currentColor" d="M3.75 1.5a.25.25 0 0 0-.25.25v11.5c0 .138.112.25.25.25h8.5a.25.25 0 0 0 .25-.25V6H9.75A1.75 1.75 0 0 1 8 4.25V1.5H3.75zm5.75.56v2.19c0 .138.112.25.25.25h2.19L9.5 2.06zM2 1.75C2 .784 2.784 0 3.75 0h5.086c.464 0 .909.184 1.237.513l3.414 3.414c.329.328.513.773.513 1.237v8.086A1.75 1.75 0 0 1 12.25 15h-8.5A1.75 1.75 0 0 1 2 13.25V1.75z"/>
            <path fill="currentColor" d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4z"/>
          </svg>
        </button>
        <!-- 打开文件按钮 -->
        <button class="toolbar-btn" @click="openFile" title="打开文件">
          <svg viewBox="0 0 16 16" width="16" height="16">
            <path fill="currentColor" d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4z"/>
          </svg>
        </button>
        <!-- 最近打开按钮 -->
        <button class="toolbar-btn" @click="toggleFileManager" title="最近打开">
          <svg viewBox="0 0 16 16" width="16" height="16">
            <path fill="currentColor" d="M1.75 1A1.75 1.75 0 000 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0016 13.25v-8.5A1.75 1.75 0 0014.25 3H7.5a.25.25 0 01-.2-.1l-.9-1.2C6.07 1.26 5.55 1 5 1H1.75z"/>
          </svg>
        </button>
      </div>

      <!-- 中间文件状态 -->
      <div class="toolbar-center">
        <span class="file-status" :class="{
          'welcome-status': isInitialContent,
          'untitled': !currentFile && !isInitialContent
        }">
          <span v-if="!isInitialContent" class="status-dot" :class="{ 'unsaved': hasUnsavedChanges }"></span>
          <span v-if="isInitialContent" class="welcome-icon">
            <svg viewBox="0 0 16 16" width="16" height="16">
              <path fill="currentColor" d="M8 0a8 8 0 1 0 0 16A8 8 0 0 0 8 0zm.25 12.25a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0zm1.75-4.5a2.25 2.25 0 0 1-3.75 1.68.75.75 0 1 1 1-.96c.27.28.72.28 1 0a.75.75 0 1 0-1-1.12A.75.75 0 0 1 7 6a2.25 2.25 0 0 1 3-2.12.75.75 0 0 1-.5 1.41 .75.75 0 0 0-1 .71v1.5a.75.75 0 0 1-1.5 0v-1.5a2.25 2.25 0 0 1 3-2.12z"/>
            </svg>
          </span>
          {{ getStatusText }}
        </span>
      </div>

      <!-- 右侧按钮组 -->
      <div class="toolbar-right">
        <button class="toolbar-btn" @click="saveFile" title="保存文件">
          <svg viewBox="0 0 16 16" width="16" height="16">
            <path fill="currentColor" d="M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.75.75 0 011.06-1.06L6 10.94l6.72-6.72a.75.75 0 011.06 0z"/>
          </svg>
        </button>
      </div>
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
import { ref, computed, onMounted, onUnmounted } from 'vue'
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
const isInitialContent = ref(true)

// 处理内容变更
const handleContentChanged = () => {
  if (!isInitialContent.value) {
    hasUnsavedChanges.value = true
  }
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
  if (isInitialContent.value || !hasUnsavedChanges.value) {
    await openSelectedFile(path, content)
    return
  }

  const confirmed = await dialog.ask('有未保存的更改，是否继续？', {
    title: '确认',
    type: 'warning'
  })
  if (confirmed) {
    await openSelectedFile(path, content)
  }
}

// 抽取打开文件的共用方法
const openSelectedFile = async (path, content) => {
  currentFile.value = path
  editorRef.value?.setContent(content)
  await invoke('add_recent_file', { path })
  await fileManagerRef.value?.refreshFiles()
  hasUnsavedChanges.value = false
  hasExternalChanges.value = false
  isInitialContent.value = false
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

// 添加快捷键处理
const handleKeydown = async (e) => {
  // Ctrl+S 或 Cmd+S
  if ((e.ctrlKey || e.metaKey) && e.key === 's') {
    e.preventDefault() // 阻止浏览器默认保存
    await saveFile()
  }
}

// 在组件挂载时添加事件监听
onMounted(() => {
  const savedPreference = localStorage.getItem('showFileManager')
  if (savedPreference !== null) {
    showFileManager.value = savedPreference === 'true'
  }
  window.addEventListener('keydown', handleKeydown)
})

// 在组件卸载时移除事件监听
onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
})

// 计算状态文本
const getStatusText = computed(() => {
  if (isInitialContent.value) {
    return '欢迎使用 - 请打开或创建新文件'
  }
  if (!currentFile.value) {
    return '未命名文档'
  }
  return currentFileName.value
})

// 新建文件方法
const createNewFile = async () => {
  try {
    // 先选择保存位置
    const dirPath = await dialog.open({
      directory: true,
      title: '选择保存位置',
      multiple: false
    })
    
    if (!dirPath) return

    // 保存新文件
    const filePath = await dialog.save({
      defaultPath: `${dirPath}/未命名.md`,
      filters: [{
        name: 'Markdown',
        extensions: ['md']
      }],
      title: '新建 Markdown 文件'
    })

    if (filePath) {
      // 创建空文件
      await invoke('save_file', { 
        path: filePath,
        content: ''  // 创建空文件
      })
      
      // 打开新创建的文件
      await handleFileSelected({ 
        path: filePath, 
        content: '' 
      })
      
      // 添加到最近文件
      await invoke('add_recent_file', { path: filePath })
      await fileManagerRef.value?.refreshFiles()
      
      // 更新状态
      isInitialContent.value = false
      hasUnsavedChanges.value = false
    }
  } catch (err) {
    console.error('创建文件失败:', err)
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
  display: flex;
  align-items: center;
  padding: 8px;
  background: #f6f8fa;
  border-bottom: 1px solid #e1e4e8;
  height: 40px;
}

.toolbar-btn {
  width: 32px;
  height: 32px;
  padding: 8px;
  background: none;
  border: none;
  border-radius: 4px;
  color: #57606a;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  margin-right: 4px;
}

.toolbar-btn:hover {
  background: #e1e4e8;
  color: #24292e;
}

/* Tooltip 样式 */
.toolbar-btn[title]::after {
  display: none; /* 禁用重复的 tooltip */
}

/* 添加新的 tooltip 样式 */
.tooltip {
  position: relative;
}

.tooltip::after {
  content: attr(title);
  position: absolute;
  bottom: -24px;
  left: 50%;
  transform: translateX(-50%);
  padding: 4px 8px;
  background: rgba(0, 0, 0, 0.8);
  color: white;
  font-size: 12px;
  border-radius: 4px;
  white-space: nowrap;
  opacity: 0;
  visibility: hidden;
  transition: all 0.2s ease;
  z-index: 1000;
}

.tooltip:hover::after {
  opacity: 1;
  visibility: visible;
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
  display: flex;
  align-items: center;
  font-size: 13px;
  color: #57606a;
  padding: 4px 12px;
  border-radius: 4px;
  background: rgba(175, 184, 193, 0.2);
  transition: all 0.3s ease;
}

.file-status.welcome-status {
  color: #0366d6;
  background: rgba(3, 102, 214, 0.1);
  border: 1px dashed #0366d6;
}

.welcome-icon {
  margin-right: 6px;
  display: flex;
  align-items: center;
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0% { opacity: 0.6; }
  50% { opacity: 1; }
  100% { opacity: 0.6; }
}

.file-status.untitled {
  color: #6a737d;
  font-style: italic;
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #28a745;
  margin-right: 6px;
}

.status-dot.unsaved {
  background: #f9826c;
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

.toolbar-left,
.toolbar-right {
  flex: 1;
  display: flex;
  align-items: center;
}

.toolbar-right {
  justify-content: flex-end;
}

.toolbar-center {
  flex: 2;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>