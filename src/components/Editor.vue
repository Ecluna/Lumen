<template>
  <div class="editor-container" :class="{ 'sidebar-hidden': !showSidebar }">
    <!-- 侧边栏 -->
    <div class="sidebar" v-show="showSidebar">
      <div class="outline">
        <!-- 这里后续会放大纲内容 -->
      </div>
    </div>

    <!-- 编辑区域 -->
    <div class="content-area">
      <!-- 工具栏 -->
      <div class="toolbar">
        <button class="toolbar-btn" @click="toggleSidebar">
          <svg viewBox="0 0 16 16" width="16" height="16">
            <path fill="currentColor" d="M2 2.5A1.5 1.5 0 013.5 1h9A1.5 1.5 0 0114 2.5v11a1.5 1.5 0 01-1.5 1.5h-9A1.5 1.5 0 012 13.5v-11zM3.5 2a.5.5 0 00-.5.5v11a.5.5 0 00.5.5h9a.5.5 0 00.5-.5v-11a.5.5 0 00-.5-.5h-9z"/>
            <path fill="currentColor" d="M6 4.5a.5.5 0 01.5-.5h4a.5.5 0 010 1h-4a.5.5 0 01-.5-.5zm0 3a.5.5 0 01.5-.5h4a.5.5 0 010 1h-4a.5.5 0 01-.5-.5zm0 3a.5.5 0 01.5-.5h4a.5.5 0 010 1h-4a.5.5 0 01-.5-.5z"/>
          </svg>
        </button>
      </div>

      <!-- 编辑器主体 -->
      <div class="editor-main">
        <div class="editor-wrapper" :style="{ flex: editorFlex }">
          <textarea
            v-model="markdownContent"
            class="markdown-input"
            @input="handleInput"
            @drop.prevent="handleDrop"
            @dragover.prevent
            placeholder="请输入 Markdown 内容..."
          ></textarea>
        </div>
        <div class="resize-handle" 
          @mousedown="startResize"
          @dblclick="resetSize">
        </div>
        <div class="preview-wrapper" :style="{ flex: previewFlex }">
          <div class="markdown-body" v-html="htmlContent"></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch, defineExpose, onMounted, defineEmits } from 'vue'
import MarkdownIt from 'markdown-it'
import hljs from 'highlight.js'
import { invoke } from '@tauri-apps/api'
import Simplebar from 'simplebar-vue'
import 'simplebar-vue/dist/simplebar.min.css'

const emit = defineEmits(['content-changed'])

// 初始化 markdown-it，配置代码高亮
const md = new MarkdownIt({
  html: true,
  linkify: true,
  typographer: true,
  highlight: function (str, lang) {
    if (lang && hljs.getLanguage(lang)) {
      try {
        return hljs.highlight(str, { language: lang }).value
      } catch (__) {}
    }
    return '' // 使用默认的转义
  }
})

const initialContent = `# 欢迎使用 Markdown 编辑器

## 基本功能

1. 实时预览
2. 文件保存/打开
3. 语法高亮
4. 拖拽文件

## 示例

### 代码块

\`\`\`javascript
function hello() {
  console.log('Hello, World!');
}
\`\`\`

### 表格

| 功能 | 支持 |
|------|------|
| 预览 | ✅ |
| 高亮 | ✅ |
| 导出 | 🚧 |

### 列表

- 项目1
  - 子项目
  - 子项目
- 项目2
- 项目3

> 这是一个引用块

**开始编辑吧！**
`

const markdownContent = ref(initialContent)
const htmlContent = ref('')

// 侧边栏状态
const showSidebar = ref(false)

// 切换侧边栏
const toggleSidebar = () => {
  showSidebar.value = !showSidebar.value
}

// 处理输入
const handleInput = async () => {
  htmlContent.value = md.render(markdownContent.value)
  // 自动保存到临时文件
  await invoke('save_temp_content', { content: markdownContent.value })
  emit('content-changed')
}

// 处理文件拖放
const handleDrop = async (e) => {
  const file = e.dataTransfer.files[0]
  if (file && file.name.match(/\.(md|markdown)$/i)) {
    const text = await file.text()
    markdownContent.value = text
    handleInput()
  }
}

// 暴露方法给父组件
const setContent = (content) => {
  markdownContent.value = content
  handleInput()
}

const getContent = () => markdownContent.value

// 初始化时渲染内容
onMounted(() => {
  handleInput()
})

// 拖动相关状态
const editorFlex = ref(1)
const previewFlex = ref(1)
let isResizing = false
let startX = 0
let startEditorFlex = 0
let startPreviewFlex = 0

// 开始拖动
const startResize = (e) => {
  isResizing = true
  startX = e.clientX
  startEditorFlex = editorFlex.value
  startPreviewFlex = previewFlex.value
  
  document.addEventListener('mousemove', handleResize)
  document.addEventListener('mouseup', stopResize)
  document.body.style.cursor = 'col-resize'
}

// 处理拖动
const handleResize = (e) => {
  if (!isResizing) return
  
  const dx = e.clientX - startX
  const totalWidth = document.querySelector('.editor-main').offsetWidth
  const ratio = dx / totalWidth
  
  const newEditorFlex = startEditorFlex + ratio
  const newPreviewFlex = startPreviewFlex - ratio
  
  if (newEditorFlex >= 0.2 && newPreviewFlex >= 0.2) {
    editorFlex.value = newEditorFlex
    previewFlex.value = newPreviewFlex
  }
}

// 停止拖动
const stopResize = () => {
  isResizing = false
  document.removeEventListener('mousemove', handleResize)
  document.removeEventListener('mouseup', stopResize)
  document.body.style.cursor = ''
}

// 双击重置大小
const resetSize = () => {
  editorFlex.value = 1
  previewFlex.value = 1
}

defineExpose({
  setContent,
  getContent
})
</script>

<style>
@import 'highlight.js/styles/github.css';

.editor-container {
  display: flex;
  height: 100vh;
  background: #fff;
}

/* 侧边栏样式 */
.sidebar {
  width: 250px;
  border-right: 1px solid #e1e4e8;
  background: #f8f9fa;
  transition: all 0.3s ease;
  flex-shrink: 0; /* 防止侧边栏被压缩 */
}

/* 内容区域样式 */
.content-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-width: 0;
  height: 100vh;
}

/* 工具栏样式 */
.toolbar {
  height: 40px;
  border-bottom: 1px solid #e1e4e8;
  display: flex;
  align-items: center;
  padding: 0 8px;
  background: #f8f9fa;
  flex-shrink: 0; /* 防止工具栏被压缩 */
}

.toolbar-btn {
  padding: 4px;
  background: none;
  border: none;
  border-radius: 4px;
  color: #57606a;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
}

.toolbar-btn:hover {
  background: #e1e4e8;
  color: #24292e;
}

/* 编辑器主体样式 */
.editor-main {
  display: flex;
  height: 100%;
  min-height: 0;
  user-select: none;
}

/* 编辑器和预览区域基础样式 */
.editor-wrapper,
.preview-wrapper {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  user-select: text;
}

.preview-wrapper {
  border-left: 1px solid #e1e4e8;
}

/* 编辑器输入区域样式 */
.markdown-input {
  width: 100%;
  height: 100%;
  border: none;
  resize: none;
  outline: none;
  font-family: 'Fira Code', monospace;
  font-size: 14px;
  line-height: 1.6;
  background: transparent;
  padding: 20px;
  padding-bottom: 80px;
  overflow-y: auto; /* 只在 textarea 上保留滚动 */
}

/* 自定义滚动条样式 - 应用到 textarea */
.markdown-input::-webkit-scrollbar {
  width: 6px;
}

.markdown-input::-webkit-scrollbar-track {
  background: transparent;
}

.markdown-input::-webkit-scrollbar-thumb {
  background-color: transparent;
  border-radius: 3px;
  transition: background-color 0.2s;
}

.markdown-input:hover::-webkit-scrollbar-thumb {
  background-color: #ccc;
}

.markdown-input:hover::-webkit-scrollbar-thumb:hover {
  background-color: #999;
}

/* 移除滚动条按钮 */
.markdown-input::-webkit-scrollbar-button {
  display: none;
}

/* 移除外层容器的滚动 */
.editor-wrapper {
  overflow: hidden;
}

/* 预览内容容器样式 */
.markdown-body {
  height: 100%;
  padding: 20px;
  padding-bottom: 80px;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 1.6;
  word-wrap: break-word;
  color: #24292e;
  overflow-y: auto;
}

/* Markdown 样式 */
.markdown-body pre {
  padding: 16px;
  overflow: auto;
  font-size: 85%;
  line-height: 1.45;
  background-color: #f6f8fa;
  border-radius: 6px;
}

.markdown-body code {
  padding: 0.2em 0.4em;
  margin: 0;
  font-size: 85%;
  background-color: rgba(175, 184, 193, 0.2);
  border-radius: 6px;
}

.markdown-body pre code {
  padding: 0;
  background-color: transparent;
}

.markdown-body blockquote {
  padding: 0 1em;
  color: #6a737d;
  border-left: 0.25em solid #dfe2e5;
  margin: 1em 0;
}

/* SimpleBar 自定义样式 */
.simplebar-scrollbar::before {
  background-color: #ccc;
  width: 4px;
  opacity: 0;
  transition: opacity 0.2s ease;
}

.simplebar-scrollbar.simplebar-visible::before {
  opacity: 0.4;
}

.simplebar-track.simplebar-vertical {
  width: 6px;
  right: 2px;
}

.simplebar-track.simplebar-vertical .simplebar-scrollbar:hover::before {
  background-color: #999;
  opacity: 0.6;
}

/* 移除重复的样式定义 */
/* 删除所有重复的 SimpleBar 相关样式 */
/* 删除所有重复的滚动条相关样式 */

/* 自定义滚动条样式 - 应用到 textarea 和预览区 */
.markdown-input::-webkit-scrollbar,
.markdown-body::-webkit-scrollbar {
  width: 6px;
}

.markdown-input::-webkit-scrollbar-track,
.markdown-body::-webkit-scrollbar-track {
  background: transparent;
}

.markdown-input::-webkit-scrollbar-thumb,
.markdown-body::-webkit-scrollbar-thumb {
  background-color: transparent;
  border-radius: 3px;
  transition: background-color 0.2s;
}

.markdown-input:hover::-webkit-scrollbar-thumb,
.markdown-body:hover::-webkit-scrollbar-thumb {
  background-color: #ccc;
}

.markdown-input:hover::-webkit-scrollbar-thumb:hover,
.markdown-body:hover::-webkit-scrollbar-thumb:hover {
  background-color: #999;
}

/* 移除滚动条按钮 */
.markdown-input::-webkit-scrollbar-button,
.markdown-body::-webkit-scrollbar-button {
  display: none;
}

/* 拖动条样式 */
.resize-handle {
  width: 5px;
  background-color: transparent;
  cursor: col-resize;
  transition: background-color 0.2s;
}

.resize-handle:hover {
  background-color: #e1e4e8;
}

/* 拖动时的样式 */
.resize-handle:active {
  background-color: #0366d6;
}
</style> 