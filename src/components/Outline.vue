<template>
  <div class="outline">
    <div class="outline-header">
      <span>大纲</span>
    </div>
    <div class="outline-content">
      <div v-for="(item, index) in outlineItems" 
           :key="index"
           class="outline-item"
           :class="{ 
             'level-1': item.level === 1,
             'level-2': item.level === 2,
             'level-3': item.level === 3,
             'level-4': item.level === 4,
             'level-5': item.level === 5,
             'level-6': item.level === 6,
             'active': currentLine >= item.startLine && currentLine < (outlineItems[index + 1]?.startLine ?? Infinity)
           }"
           @click="scrollToHeading(item)">
        <span class="heading-text">{{ item.text }}</span>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'

const props = defineProps({
  content: {
    type: String,
    required: true
  },
  currentLine: {
    type: Number,
    default: 0
  }
})

const outlineItems = ref([])

// 解析内容生成大纲
const parseOutline = (content) => {
  const lines = content.split('\n')
  const items = []
  
  lines.forEach((line, index) => {
    // 匹配标题行，但不显示 # 符号
    const match = line.match(/^(#{1,6})\s+(.+)$/)
    if (match) {
      items.push({
        level: match[1].length,
        text: match[2],
        startLine: index,
        position: content.split('\n').slice(0, index).join('\n').length + (index > 0 ? 1 : 0)
      })
    }
  })
  
  return items
}

// 监听内容变化
watch(() => props.content, (newContent) => {
  outlineItems.value = parseOutline(newContent)
}, { immediate: true })

// 滚动到对应标题
const scrollToHeading = (item) => {
  const editor = document.querySelector('.markdown-input')
  if (!editor) return
  
  // 设置光标位置并滚动
  editor.setSelectionRange(item.position, item.position)
  editor.focus()
  
  // 计算滚动位置 - 将目标行设置在顶部
  const lineHeight = parseInt(getComputedStyle(editor).lineHeight)
  const lineNumber = editor.value.substr(0, item.position).split('\n').length
  const scrollTop = (lineNumber - 1) * lineHeight // 减1是为了让标题行显示在顶部
  
  // 使用平滑滚动
  editor.scrollTo({
    top: scrollTop,
    behavior: 'smooth'
  })
  
  // 立即更新当前行
  currentLine.value = item.startLine
}

// 计算当前活动标题
const getActiveHeading = (line) => {
  if (!outlineItems.value.length) return null
  
  for (let i = outlineItems.value.length - 1; i >= 0; i--) {
    if (outlineItems.value[i].startLine <= line) {
      return outlineItems.value[i]
    }
  }
  
  return outlineItems.value[0]
}

// 监听当前行变化
watch(() => props.currentLine, (newLine) => {
  const activeHeading = getActiveHeading(newLine)
  if (activeHeading) {
    // 确保活动标题在视图中可见
    const activeElement = document.querySelector('.outline-item.active')
    if (activeElement) {
      activeElement.scrollIntoView({ behavior: 'smooth', block: 'nearest' })
    }
  }
})
</script>

<style>
.outline {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: #fafafa;
  user-select: none;
  position: relative;
  z-index: 2;
}

.outline-header {
  padding: 8px 16px;
  font-size: 14px;
  font-weight: 600;
  color: #24292e;
  border-bottom: 1px solid #e1e4e8;
  background: #fafafa;
  position: sticky;
  top: 0;
  z-index: 3;
}

.outline-content {
  flex: 1;
  overflow-y: auto;
  padding: 8px 0;
}

.outline-item {
  padding: 4px 16px;
  cursor: pointer;
  font-size: 13px;
  color: #57606a;
  transition: all 0.2s ease;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  position: relative;
}

.outline-item:hover {
  background: #f0f1f2;
  color: #24292e;
}

.outline-item.active {
  font-weight: 600;
  color: #0366d6;
}

.outline-item.active::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 2px;
  background-color: #0366d6;
  transition: opacity 0.2s;
}

.heading-text {
  display: block;
  overflow: hidden;
  text-overflow: ellipsis;
}

.level-1 { padding-left: 16px; }
.level-2 { padding-left: 24px; }
.level-3 { padding-left: 32px; }
.level-4 { padding-left: 40px; }
.level-5 { padding-left: 48px; }
.level-6 { padding-left: 56px; }

/* 自定义滚动条 */
.outline-content::-webkit-scrollbar {
  width: 6px;
}

.outline-content::-webkit-scrollbar-track {
  background: transparent;
}

.outline-content::-webkit-scrollbar-thumb {
  background-color: transparent;
  border-radius: 3px;
}

.outline-content:hover::-webkit-scrollbar-thumb {
  background-color: #ccc;
}

.outline-content:hover::-webkit-scrollbar-thumb:hover {
  background-color: #999;
}
</style> 