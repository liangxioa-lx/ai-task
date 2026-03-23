<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useTaskStore } from "./stores/task";

const store = useTaskStore();
const inputValue = ref("");

onMounted(() => {
  void store.bootstrap();
});

function onAddTask() {
  void store.addTask(inputValue.value);
  inputValue.value = "";
}

function onToggle(taskId: string) {
  void store.toggleTask(taskId);
}

function onRemove(taskId: string) {
  void store.removeTask(taskId);
}
</script>

<template>
  <main class="page">
    <section class="panel">
      <header class="panel-header">
        <h1>AI 任务管理工具</h1>
        <p>基于 Tauri + Vue3 + Pinia + SQLite 的任务管理基础架构</p>
      </header>

      <div class="add-row">
        <input
          v-model="inputValue"
          type="text"
          placeholder="例如：给任务增加智能拆解能力"
          :disabled="store.loading"
          @keyup.enter="onAddTask"
        />
        <button :disabled="store.loading" @click="onAddTask">添加</button>
      </div>

      <p class="meta">待完成任务：{{ store.pendingCount }}</p>
      <p v-if="store.databasePath" class="db-path">数据库：{{ store.databasePath }}</p>
      <p v-if="store.error" class="error">{{ store.error }}</p>
      <p v-if="store.loading" class="loading">正在加载数据库任务...</p>

      <ul class="task-list">
        <li v-for="task in store.tasks" :key="task.id" :class="{ done: task.done }">
          <label>
            <input :checked="task.done" type="checkbox" @change="onToggle(task.id)" />
            <span>{{ task.title }}</span>
          </label>
          <div class="task-extra">
            <small>{{ task.taskComplexityType }} / {{ task.executionStatus }}</small>
            <button class="danger" @click="onRemove(task.id)">删除</button>
          </div>
        </li>
      </ul>
    </section>
  </main>
</template>
