<template>
  <div class="file-manager">
    <div class="recent-files">
      <h3>最近文件</h3>
      <ul>
        <li 
          v-for="file in recentFiles" 
          :key="file.path"
          @click="openRecentFile(file.path)"
          :class="{ 
            active: currentFile === file.path,
            'has-changes': hasExternalChanges && currentFile === file.path
          }"
        >
          <span class="file-name">{{ fileNames[file.path] || file.path }}</span>
          <span class="file-path">{{ file.path }}</span>
          <span 
            v-if="hasExternalChanges && currentFile === file.path" 
            class="change-indicator"
            title="文件已被外部修改"
          >
            ⚠️
          </span>
          <span class="file-time">{{ formatTime(file.last_modified) }}</span>
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api'
import { basename } from '@tauri-apps/api/path'

const props = defineProps({
  currentFile: String,
  hasExternalChanges: Boolean
})

const emit = defineEmits(['fileSelected', 'external-changes'])
const recentFiles = ref([])
const fileNames = ref({})

// 获取文件名
const updateFileNames = async () => {
  for (const file of recentFiles.value) {
    try {
      fileNames.value[file.path] = await basename(file.path)
    } catch {
      fileNames.value[file.path] = file.path
    }
  }
}

// 加载最近文件列表
const loadRecentFiles = async () => {
  try {
    recentFiles.value = await invoke('get_recent_files')
    await updateFileNames()
  } catch (err) {
    console.error('加载最近文件失败:', err)
  }
}

// 暴露刷新方法给父组件
const refreshFiles = async () => {
  await loadRecentFiles()
}

defineExpose({
  refreshFiles
})

// 打开最近的文件
const openRecentFile = async (path) => {
  try {
    const content = await invoke('open_file', { path })
    emit('fileSelected', { path, content })
  } catch (err) {
    console.error('打开文件失败:', err)
  }
}

// 格式化时间
const formatTime = (timestamp) => {
  const date = new Date(timestamp * 1000)
  return date.toLocaleString()
}

// 检查文件变更
const checkFileChanges = async () => {
  if (props.currentFile) {
    try {
      const hasChanges = await invoke('check_file_changes', { 
        path: props.currentFile 
      })
      if (hasChanges) {
        emit('external-changes')
      }
    } catch (err) {
      console.error('检查文件变更失败:', err)
    }
  }
}

// 定期检查文件变更
let checkInterval
onMounted(() => {
  loadRecentFiles()
  checkInterval = setInterval(checkFileChanges, 5000) // 每5秒检查一次
})

onUnmounted(() => {
  if (checkInterval) {
    clearInterval(checkInterval)
  }
})
</script>

<style>
.file-manager {
  width: 250px;
  min-width: 250px;
  border-right: 1px solid var(--border-color);
  background-color: var(--file-manager-bg);
  height: 100%;
  overflow-y: auto;
  transition: all 0.3s ease;
}

.recent-files {
  padding: 16px;
}

.recent-files h3 {
  font-size: 14px;
  color: var(--text-primary);
  margin-bottom: 8px;
}

.recent-files ul {
  list-style: none;
  padding: 0;
  margin: 0;
}

.recent-files li {
  padding: 8px;
  cursor: pointer;
  border-radius: 4px;
  margin-bottom: 4px;
  color: var(--text-primary);
  transition: background-color 0.2s ease;
}

.recent-files li:hover {
  background-color: var(--hover-bg);
}

.recent-files li.active {
  background-color: var(--accent-color);
  color: #ffffff;
}

.file-name {
  display: block;
  font-weight: 500;
  margin-bottom: 2px;
}

.file-path {
  display: block;
  font-size: 12px;
  color: var(--text-secondary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

li.active .file-path {
  color: rgba(255, 255, 255, 0.7);
}

.change-indicator {
  margin-left: 8px;
  color: var(--unsaved-dot);
}

.has-changes {
  background-color: var(--welcome-bg);
}

.has-changes.active {
  background-color: var(--accent-color);
}

.file-time {
  display: block;
  font-size: 12px;
  color: var(--text-secondary);
  margin-top: 4px;
}

li.active .file-time {
  color: rgba(255, 255, 255, 0.7);
}
</style> 