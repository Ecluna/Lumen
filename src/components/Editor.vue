<template>
  <div class="editor-container">
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
    <div 
      class="preview-wrapper markdown-body" 
      v-html="htmlContent"
    ></div>
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
  height: calc(100vh - 50px); /* 减去工具栏高度 */
  width: 100%;
}

.editor-wrapper,
.preview-wrapper {
  width: 50%;
  padding: 20px;
  overflow-y: auto;
}

.markdown-input {
  width: 100%;
  height: 100%;
  border: none;
  resize: none;
  outline: none;
  font-family: 'Fira Code', monospace;
  font-size: 14px;
  line-height: 1.6;
  padding: 20px;
  background-color: #fafbfc;
  color: #24292e;
}

.preview-wrapper {
  border-left: 1px solid #e1e4e8;
  background-color: #ffffff;
  padding: 20px 30px;
}

/* 预览区域的基础样式 */
.markdown-body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 1.6;
  word-wrap: break-word;
  color: #24292e;
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
</style> 