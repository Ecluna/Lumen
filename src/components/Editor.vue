<template>
  <div class="editor-container">
    <div class="editor-wrapper">
      <textarea
        v-model="markdownContent"
        class="markdown-input"
        @input="handleInput"
        placeholder="请输入 Markdown 内容..."
      ></textarea>
    </div>
    <div class="preview-wrapper" v-html="htmlContent"></div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'
import MarkdownIt from 'markdown-it'
import { invoke } from '@tauri-apps/api/tauri'

const md = new MarkdownIt()
const markdownContent = ref('')
const htmlContent = ref('')

const handleInput = async () => {
  htmlContent.value = md.render(markdownContent.value)
  // 自动保存到临时文件
  await invoke('save_temp_content', { content: markdownContent.value })
}
</script>

<style scoped>
.editor-container {
  display: flex;
  height: 100vh;
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
  font-family: monospace;
  padding: 10px;
}

.preview-wrapper {
  border-left: 1px solid #ddd;
  background-color: #fafafa;
}
</style> 