<template>
  <div class="app-container">
    <div class="toolbar">
      <!-- 左侧按钮组 -->
      <div class="toolbar-left">
        <!-- 新建文件按钮 -->
        <button class="toolbar-btn" @click="createNewFile" title="新建文件">
          <svg viewBox="0 0 16 16" width="16" height="16">
            <path fill="currentColor" d="M3.75 1h8.5A1.75 1.75 0 0 1 14 2.75v10.5A1.75 1.75 0 0 1 12.25 14h-8.5A1.75 1.75 0 0 1 2 12.25v-10.5C2 1.784 2.784 1 3.75 1zm0 1.5a.25.25 0 0 0-.25.25V12.25c0 .138.112.25.25.25h8.5a.25.25 0 0 0 .25-.25V2.75a.25.25 0 0 0-.25-.25h-8.5z"/>
            <path fill="currentColor" d="M8 4.5a.75.75 0 0 1 .75.75v2h2a.75.75 0 0 1 0 1.5h-2v2a.75.75 0 0 1-1.5 0v-2h-2a.75.75 0 0 1 0-1.5h2v-2A.75.75 0 0 1 8 4.5z"/>
          </svg>
        </button>
        <!-- 打开文件按钮 -->
        <button class="toolbar-btn" @click="openFile" title="打开文件">
          <svg viewBox="0 0 16 16" width="16" height="16">
            <path fill="currentColor" d="M1.75 2.5a.25.25 0 0 0-.25.25v10.5c0 .138.112.25.25.25h12.5a.25.25 0 0 0 .25-.25v-8.5a.25.25 0 0 0-.25-.25H7.5a1.75 1.75 0 0 1-1.395-.595L4.964 2.764A.25.25 0 0 0 4.765 2.5H1.75zM0 2.75C0 1.784.784 1 1.75 1h3.015c.464 0 .91.184 1.237.513l1.142 1.142a.25.25 0 0 0 .199.095h6.907c.966 0 1.75.784 1.75 1.75v8.5A1.75 1.75 0 0 1 14.25 14.5H1.75A1.75 1.75 0 0 1 0 12.75V2.75z"/>
            <path fill="currentColor" d="M7.44 6.32a.75.75 0 0 1 1.06 0l2.5 2.5a.75.75 0 0 1 0 1.06l-2.5 2.5a.75.75 0 0 1-1.06-1.06L8.69 10H3.75a.75.75 0 0 1 0-1.5h4.94L7.44 7.38a.75.75 0 0 1 0-1.06z"/>
          </svg>
        </button>
        <!-- 保存文件按钮 -->
        <button class="toolbar-btn" @click="saveFile" title="保存文件">
          <svg viewBox="0 0 16 16" width="16" height="16">
            <path fill="currentColor" d="M13.78 4.22a.75.75 0 0 1 0 1.06l-7.25 7.25a.75.75 0 0 1-1.06 0L2.22 9.28a.75.75 0 0 1 1.06-1.06L6 10.94l6.72-6.72a.75.75 0 0 1 1.06 0z"/>
          </svg>
        </button>
        <!-- 最近打开按钮 -->
        <button class="toolbar-btn" @click="toggleFileManager" title="最近打开">
          <svg viewBox="0 0 16 16" width="16" height="16">
            <!-- 关闭状态的文件夹图标 -->
            <path v-if="!showFileManager" fill="currentColor" d="M1.75 1A1.75 1.75 0 0 0 0 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0 0 16 13.25v-8.5A1.75 1.75 0 0 0 14.25 3H7.5a.25.25 0 0 1-.2-.1l-.9-1.2C6.07 1.26 5.55 1 5 1H1.75zM1.5 2.75a.25.25 0 0 1 .25-.25H5c.182 0 .359.046.515.135l1.14 1.52a1.75 1.75 0 0 0 1.395.595h6.2a.25.25 0 0 1 .25.25v8.5a.25.25 0 0 1-.25.25H1.75a.25.25 0 0 1-.25-.25V2.75z"/>
            <!-- 打开状态的文件夹图标 -->
            <path v-else fill="currentColor" d="M1.75 1A1.75 1.75 0 0 0 0 2.75v10.5C0 14.216.784 15 1.75 15h12.5A1.75 1.75 0 0 0 16 13.25v-8.5A1.75 1.75 0 0 0 14.25 3H7.5a.25.25 0 0 1-.2-.1l-.9-1.2C6.07 1.26 5.55 1 5 1H1.75zM1.5 2.75a.25.25 0 0 1 .25-.25H5c.182 0 .359.046.515.135l1.14 1.52a1.75 1.75 0 0 0 1.395.595h6.2a.25.25 0 0 1 .25.25v8.5a.25.25 0 0 1-.25.25H1.75a.25.25 0 0 1-.25-.25V2.75zM3.5 6v7h9V6h-9z"/>
          </svg>
        </button>
        <!-- 主题切换按钮 -->
        <button class="toolbar-btn" 
          @click="toggleTheme" 
          :title="isDarkMode ? '切换亮色主题' : '切换暗色主题'">
          <svg v-if="!isDarkMode" viewBox="0 0 16 16" width="16" height="16">
            <path fill="currentColor" d="M8 12a4 4 0 1 0 0-8 4 4 0 0 0 0 8zM8 0a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-1 0v-2A.5.5 0 0 1 8 0zm0 13a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-1 0v-2A.5.5 0 0 1 8 13zm8-5a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1 0-1h2a.5.5 0 0 1 .5.5zM3 8a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1 0-1h2A.5.5 0 0 1 3 8zm10.657-5.657a.5.5 0 0 1 0 .707l-1.414 1.414a.5.5 0 1 1-.707-.707l1.414-1.414a.5.5 0 0 1 .707 0zm-9.193 9.193a.5.5 0 0 1 0 .707L3.05 13.657a.5.5 0 0 1-.707-.707l1.414-1.414a.5.5 0 0 1 .707 0zm9.193 2.121a.5.5 0 0 1-.707 0l-1.414-1.414a.5.5 0 0 1 .707-.707l1.414 1.414a.5.5 0 0 1 0 .707zM4.464 4.465a.5.5 0 0 1-.707 0L2.343 3.05a.5.5 0 1 1 .707-.707l1.414 1.414a.5.5 0 0 1 0 .708z"/>
          </svg>
          <svg v-else viewBox="0 0 16 16" width="16" height="16">
            <path fill="currentColor" d="M6 .278a.768.768 0 0 1 .08.858 7.208 7.208 0 0 0-.878 3.46c0 4.021 3.278 7.277 7.318 7.277.527 0 1.04-.055 1.533-.16a.787.787 0 0 1 .81.316.733.733 0 0 1-.031.893A8.349 8.349 0 0 1 8.344 16C3.734 16 0 12.286 0 7.71 0 4.266 2.114 1.312 5.124.06A.752.752 0 0 1 6 .278z"/>
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
        <button class="toolbar-btn" 
          @click="togglePreview" 
          :class="{ 'active': showPreview }"
          title="预览">
          <svg viewBox="0 0 16 16" width="16" height="16">
            <g fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
              <!-- 眼睛形状 -->
              <path d="M1.5 8c0 0 2.5-4.5 6.5-4.5S14.5 8 14.5 8s-2.5 4.5-6.5 4.5S1.5 8 1.5 8z" />
              <!-- 眼睛中心 -->
              <circle cx="8" cy="8" r="2" />
              <!-- 斜线 - 使用 v-show 控制显示/隐藏 -->
              <line v-show="!showPreview" x1="2" y1="14" x2="14" y2="2" stroke-width="1.5" />
            </g>
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
        @file-opened="handleFileOpened"
        @content-changed="handleContentChanged"
        :class="{ 'editor-full': !showFileManager }"
        :showPreview="showPreview"
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
const showFileManager = ref(false)
const isInitialContent = ref(true)
const showPreview = ref(true)

// 添加主题状态
const isDarkMode = ref(false)

// 切换主题
const toggleTheme = () => {
  isDarkMode.value = !isDarkMode.value
  // 保存用户偏好
  localStorage.setItem('theme', isDarkMode.value ? 'dark' : 'light')
  // 应用主题
  document.documentElement.classList.toggle('dark-theme', isDarkMode.value)
}

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

// 切换预览显示
const togglePreview = () => {
  showPreview.value = !showPreview.value
  // 保存用户偏好
  localStorage.setItem('showPreview', showPreview.value.toString())
}

// 在组件挂载时添加事件监听
onMounted(() => {
  // 只在用户手动设置过的情况下才使用保存的偏好
  const savedPreference = localStorage.getItem('showFileManager')
  if (savedPreference !== null) {
    showFileManager.value = savedPreference === 'true'
  }
  window.addEventListener('keydown', handleKeydown)
  const savedPreviewPreference = localStorage.getItem('showPreview')
  if (savedPreviewPreference !== null) {
    showPreview.value = savedPreviewPreference === 'true'
  }
  const savedTheme = localStorage.getItem('theme')
  if (savedTheme) {
    isDarkMode.value = savedTheme === 'dark'
    document.documentElement.classList.toggle('dark-theme', isDarkMode.value)
  } else {
    // 如果没有保存的偏好，跟随系统
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
    isDarkMode.value = prefersDark
    document.documentElement.classList.toggle('dark-theme', prefersDark)
  }
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

// 处理文件打开事件
const handleFileOpened = async ({ path, content }) => {
  currentFile.value = path
  isInitialContent.value = false
  hasUnsavedChanges.value = false
  await updateCurrentFileName()
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
  background-color: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  height: 40px;
}

.toolbar-btn {
  width: 32px;
  height: 32px;
  padding: 8px;
  background: none;
  border: none;
  border-radius: 4px;
  color: var(--text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  margin-right: 4px;
  transition: all 0.2s ease;
}

.toolbar-btn:hover {
  background-color: var(--hover-bg);
  color: var(--text-primary);
}

.toolbar-btn.active {
  background-color: var(--hover-bg);
  color: var(--text-primary);
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

/* 添加预览区域的过渡效果 */
.preview-wrapper {
  transition: width 0.3s ease, opacity 0.3s ease;
}

.preview-hidden {
  width: 0;
  padding: 0;
  opacity: 0;
  overflow: hidden;
}

/* 暗色主题变量 */
:root {
  --bg-primary: #ffffff;
  --bg-secondary: #f6f8fa;
  --border-color: #e1e4e8;
  --text-primary: #24292e;
  --text-secondary: #57606a;
  --accent-color: #0366d6;
  --hover-bg: #e1e4e8;
  --code-bg: #f6f8fa;
  --blockquote-bg: #f6f8fa;
  --blockquote-border: #dfe2e5;
  --scrollbar-thumb: #ccc;
  --scrollbar-thumb-hover: #999;
  --file-manager-bg: #fafafa;
  --status-bar-bg: #f8f9fa;
  --welcome-bg: rgba(3, 102, 214, 0.1);
  --welcome-border: #0366d6;
  --unsaved-dot: #f9826c;
  --saved-dot: #28a745;
}

:root.dark-theme {
  --bg-primary: #0d1117;
  --bg-secondary: #161b22;
  --border-color: #30363d;
  --text-primary: #c9d1d9;
  --text-secondary: #8b949e;
  --accent-color: #58a6ff;
  --hover-bg: #21262d;
  --code-bg: #1f2428;
  --blockquote-bg: #1f2428;
  --blockquote-border: #30363d;
  --scrollbar-thumb: #30363d;
  --scrollbar-thumb-hover: #424a53;
  --file-manager-bg: #161b22;
  --status-bar-bg: #161b22;
  --welcome-bg: rgba(88, 166, 255, 0.1);
  --welcome-border: #58a6ff;
  --unsaved-dot: #f85149;
  --saved-dot: #238636;
}

/* 应用主题变量到所有相关元素 */
.app-container {
  background-color: var(--bg-primary);
  color: var(--text-primary);
}

.markdown-input {
  background-color: var(--bg-primary);
  color: var(--text-primary);
}

.markdown-body {
  background-color: var(--bg-primary);
  color: var(--text-primary);
}

.markdown-body pre {
  background-color: var(--code-bg);
}

.markdown-body code {
  background-color: var(--code-bg);
}

.markdown-body blockquote {
  background-color: var(--blockquote-bg);
  border-color: var(--blockquote-border);
  color: var(--text-secondary);
}

.file-manager {
  background-color: var(--bg-secondary);
  border-right: 1px solid var(--border-color);
}

.status-bar {
  background-color: var(--bg-secondary);
  border-top: 1px solid var(--border-color);
  color: var(--text-secondary);
}

.status-dot.unsaved {
  background-color: var(--unsaved-dot);
}

.status-dot {
  background-color: var(--saved-dot);
}

/* 滚动条样式 */
::-webkit-scrollbar-thumb {
  background-color: var(--scrollbar-thumb);
}

::-webkit-scrollbar-thumb:hover {
  background-color: var(--scrollbar-thumb-hover);
}

/* 欢迎状态样式 */
.file-status.welcome-status {
  color: var(--accent-color);
  background: var(--welcome-bg);
  border-color: var(--welcome-border);
}

/* 修改大纲侧边栏样式 */
.outline-sidebar {
  width: 240px;
  background-color: var(--bg-secondary);
  border-right: 1px solid var(--border-color);
  transition: all 0.3s ease;
}

.status-btn {
  color: var(--text-secondary);
}

.status-btn:hover {
  background-color: var(--hover-bg);
  color: var(--text-primary);
}

.status-btn.active {
  background-color: var(--hover-bg);
  color: var(--text-primary);
}
</style>