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
        <div class="editor-wrapper">
          <textarea
            v-model="markdownContent"
            class="markdown-input"
            @input="handleInput"
            @drop.prevent="handleDrop"
            @dragover.prevent
            placeholder="请输入 Markdown 内容..."
          ></textarea>
        </div>
        <div class="preview-wrapper markdown-body" v-html="htmlContent"></div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch, defineExpose, onMounted, defineEmits } from 'vue'
import MarkdownIt from 'markdown-it'
import hljs from 'highlight.js'
import { invoke } from '@tauri-apps/api'

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
  flex: 1;
  display: flex;
  min-height: 0;
  overflow: hidden; /* 隐藏中间的滚动条 */
}

/* 调整编辑器和预览区域样式 */
.editor-wrapper {
  flex: 1;
  padding: 20px;
  padding-bottom: 80px; /* 增加更多底部内边距 */
}

.preview-wrapper {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
  padding-bottom: 80px; /* 增加更多底部内边距 */
  border-left: 1px solid #e1e4e8;
}

/* 编辑器输入区域样式 */
.markdown-input {
  width: 100%;
  min-height: 100%;
  border: none;
  resize: none;
  outline: none;
  font-family: 'Fira Code', monospace;
  font-size: 14px;
  line-height: 1.6;
  background: transparent;
  overflow-y: auto;
  padding-bottom: 40px; /* 为输入框内容添加底部间距 */
}

/* 预览区域样式 */
.preview-wrapper {
  border-left: 1px solid #e1e4e8;
}

/* 预览区域的基础样式 */
.markdown-body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 1.6;
  word-wrap: break-word;
  color: #24292e;
  padding-bottom: 40px; /* 增加预览内容的底部间距 */
}

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

.markdown-body h1,
.markdown-body h2 {
  border-bottom: 1px solid #eaecef;
  padding-bottom: 0.3em;
}

.markdown-body blockquote {
  padding: 0 1em;
  color: #6a737d;
  border-left: 0.25em solid #dfe2e5;
  margin: 1em 0;
}

.markdown-body table {
  border-collapse: collapse;
  width: 100%;
  margin: 1em 0;
}

.markdown-body table th,
.markdown-body table td {
  padding: 6px 13px;
  border: 1px solid #dfe2e5;
}

.markdown-body table tr:nth-child(2n) {
  background-color: #f6f8fa;
}

/* 隐藏侧边栏时的样式 */
.sidebar-hidden .sidebar {
  width: 0;
  padding: 0;
  border: none;
  overflow: hidden;
}

/* 最后一个元素的额外间距 */
.markdown-body > *:last-child {
  margin-bottom: 40px; /* 确保最后一个元素有足够的底部间距 */
}

/* 美化滚动条样式 */
.editor-wrapper::-webkit-scrollbar,
.preview-wrapper::-webkit-scrollbar,
.markdown-input::-webkit-scrollbar {
  width: 6px; /* 更窄的滚动条 */
  height: 6px;
}

.editor-wrapper::-webkit-scrollbar-track,
.preview-wrapper::-webkit-scrollbar-track,
.markdown-input::-webkit-scrollbar-track {
  background: transparent; /* 透明轨道 */
}

.editor-wrapper::-webkit-scrollbar-thumb,
.preview-wrapper::-webkit-scrollbar-thumb,
.markdown-input::-webkit-scrollbar-thumb {
  background: #ccc; /* 浅灰色滑块 */
  border-radius: 3px;
  transition: background 0.2s ease;
}

/* 悬浮时的滚动条样式 */
.editor-wrapper::-webkit-scrollbar-thumb:hover,
.preview-wrapper::-webkit-scrollbar-thumb:hover,
.markdown-input::-webkit-scrollbar-thumb:hover {
  background: #999; /* 深一点的灰色 */
}

/* 确保滚动条只在需要时显示 */
.editor-wrapper,
.preview-wrapper,
.markdown-input {
  scrollbar-width: thin; /* Firefox */
  scrollbar-color: #ccc transparent; /* Firefox */
}

/* 当内容滚动时才显示滚动条 */
.editor-wrapper:not(:hover)::-webkit-scrollbar-thumb,
.preview-wrapper:not(:hover)::-webkit-scrollbar-thumb,
.markdown-input:not(:hover)::-webkit-scrollbar-thumb {
  background: transparent;
}
</style> 