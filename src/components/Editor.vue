<template>
  <div class="editor-container">
    <!-- 编辑区域 -->
    <div class="content-area">
      <!-- 编辑器主体 -->
      <div class="editor-main" :class="{ 'outline-open': showOutline }">
        <!-- 大纲侧边栏 -->
        <div class="outline-sidebar" :class="{ 'outline-hidden': !showOutline }">
          <Outline :content="markdownContent" :current-line="currentLine" />
        </div>

        <!-- 编辑器和预览区域 -->
        <div class="editor-content">
          <div class="editor-wrapper" :style="{ flex: editorFlex }">
            <textarea
              v-model="markdownContent"
              class="markdown-input"
              @input="handleInput"
              @drop.prevent="handleDrop"
              @dragover.prevent
              @click="handleCursorMove"
              @keyup="handleCursorMove"
              @scroll="handleScroll"
              placeholder="请输入 Markdown 内容..."
              ref="editorRef"
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

      <!-- 底部状态栏 -->
      <div class="status-bar" :class="{ 'outline-open': showOutline }">
        <div class="status-left">
          <button class="status-btn" 
            :class="{ 'active': showOutline }"
            @click="toggleOutline" 
            title="切换侧边栏">
            <div class="icon-wrapper">
              <svg class="icon-menu" viewBox="0 0 16 16" width="16" height="16">
                <path fill="currentColor" d="M2 2.5A.5.5 0 0 1 2.5 2h9a.5.5 0 0 1 0 1h-9a.5.5 0 0 1-.5-.5zm0 4a.5.5 0 0 1 .5-.5h9a.5.5 0 0 1 0 1h-9a.5.5 0 0 1-.5-.5zm0 4a.5.5 0 0 1 .5-.5h9a.5.5 0 0 1 0 1h-9a.5.5 0 0 1-.5-.5z"/>
              </svg>
              <svg class="icon-close" viewBox="0 0 16 16" width="16" height="16">
                <path fill="currentColor" d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z"/>
              </svg>
            </div>
          </button>
        </div>
        <div class="status-right">
          <span class="word-count">{{ wordCount }} 词</span>
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
import Outline from './Outline.vue'

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

// 大纲显示状态
const showOutline = ref(false)

// 切换大纲显示
const toggleOutline = () => {
  showOutline.value = !showOutline.value
}

// 添加字数统计
const wordCount = ref(0)

// 更新字数统计
const updateWordCount = () => {
  // 移除 Markdown 语法标记
  const cleanText = markdownContent.value
    .replace(/```[\s\S]*?```/g, '') // 移除代码块
    .replace(/`.*?`/g, '')          // 移除行内代码
    .replace(/\[.*?\]/g, '')        // 移除链接文本
    .replace(/\(.*?\)/g, '')        // 移除链接地址
    .replace(/[#*_~>]/g, '')        // 移除其他 Markdown 标记
    .trim()
  
  // 使用中文分词统计
  const words = cleanText.match(/[\u4e00-\u9fa5]|[a-zA-Z0-9]+/g) || []
  wordCount.value = words.length
}

// 处理输入
const handleInput = async () => {
  htmlContent.value = md.render(markdownContent.value)
  updateWordCount() // 添加字数统计更新
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

const editorRef = ref(null)
const currentLine = ref(0)

// 处理光标位置变化
const handleCursorMove = (e) => {
  const textarea = e.target
  const content = textarea.value.substring(0, textarea.selectionStart)
  const newLine = content.split('\n').length - 1
  if (currentLine.value !== newLine) {
    currentLine.value = newLine
  }
}

// 处理滚动事件
const handleScroll = () => {
  if (!editorRef.value) return
  
  const textarea = editorRef.value
  const lineHeight = parseInt(getComputedStyle(textarea).lineHeight)
  const scrollTop = textarea.scrollTop
  const visibleLines = Math.floor(scrollTop / lineHeight)
  
  // 获取可见区域中间位置的行号
  const middleOffset = Math.floor(textarea.clientHeight / lineHeight / 2)
  const targetLine = visibleLines + middleOffset
  
  // 更新当前行
  const lines = textarea.value.split('\n')
  if (targetLine < lines.length) {
    currentLine.value = targetLine
  }
}

defineExpose({
  setContent,
  getContent,
  toggleOutline
})
</script>

<style>
@import 'highlight.js/styles/github.css';

/* 引入 JetBrains Mono 字体 */
@import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;500&display=swap');

.editor-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #fff;
}

.content-area {
  display: flex;
  flex-direction: column;
  flex: 1;
  min-height: 0;
}

.editor-main {
  display: flex;
  flex: 1;
  min-height: 0;
  position: relative;
  transition: margin-left 0.3s ease;
}

.editor-main.outline-open {
  margin-left: 240px;
}

.outline-btn {
  transition: transform 0.3s ease;
}

.outline-btn.outline-open {
  transform: translateX(240px);
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
}

.toolbar-btn:hover {
  background: #e1e4e8;
  color: #24292e;
}

/* Tooltip 样式 */
.toolbar-btn[title]::after {
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
}

.toolbar-btn[title]:hover::after {
  opacity: 1;
  visibility: visible;
}

.outline-sidebar {
  position: absolute;
  left: -240px;
  top: 0;
  bottom: 0;
  width: 240px;
  background: #fafafa;
  border-right: 1px solid #e1e4e8;
  transition: transform 0.3s ease;
  z-index: 2;
}

.outline-hidden {
  transform: translateX(-100%);
}

.editor-content {
  display: flex;
  flex: 1;
  min-width: 0;
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
  overflow-y: auto;
  color: #24292e;
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

.outline-wrapper {
  width: 240px;
  height: 100%;
  flex-shrink: 0;
}

/* 状态栏样式 */
.status-bar {
  height: 25px;
  border-top: 1px solid #e1e4e8;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 12px;
  background: #f8f9fa;
  font-size: 12px;
  color: #57606a;
  transition: margin-left 0.3s ease;
  flex-shrink: 0;
}

.status-bar.outline-open {
  margin-left: 240px;
}

.status-btn {
  width: 24px;
  height: 24px;
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

.status-btn:hover {
  background: #e1e4e8;
  color: #24292e;
}

.status-btn.active {
  background: #e1e4e8;
  color: #24292e;
}

.icon-wrapper {
  position: relative;
  width: 16px;
  height: 16px;
}

.icon-menu,
.icon-close {
  position: absolute;
  top: 0;
  left: 0;
  transition: opacity 0.3s ease, transform 0.3s ease;
}

.icon-close {
  opacity: 0;
  transform: rotate(-180deg);
}

.status-btn.active .icon-menu {
  opacity: 0;
  transform: rotate(180deg);
}

.status-btn.active .icon-close {
  opacity: 1;
  transform: rotate(0);
}

.word-count {
  color: #57606a;
  font-size: 12px;
  user-select: none;
}

/* 修改 status-btn 的 tooltip 样式 */
.status-btn[title]::after {
  display: none; /* 禁用重复的 tooltip */
}

/* 修改预览区域代码块字体 */
.markdown-body pre,
.markdown-body code {
  font-family: 'JetBrains Mono', 'Fira Code', Consolas, monospace;
}

.markdown-body :not(pre) > code {
  font-family: 'JetBrains Mono', 'Fira Code', Consolas, monospace;
}
</style> 