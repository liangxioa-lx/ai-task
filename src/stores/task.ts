import { computed, ref } from "vue";
import { defineStore } from "pinia";
import {
  createTask,
  deleteTask,
  initDatabase,
  listTasks,
  updateTaskExecution,
  type ExecutionResult,
  type ExecutionStatus,
  type TaskComplexityType,
  type TaskRecord
} from "../api/database";

export interface TaskItem {
  id: string;
  title: string;
  done: boolean;
  executionCount: number;
  lastExecutedAt: string | null;
  executionStatus: ExecutionStatus;
  executionResult: ExecutionResult;
  taskComplexityType: TaskComplexityType;
}

function toTaskItem(task: TaskRecord): TaskItem {
  return {
    id: task.taskId,
    title: task.taskName,
    done: task.executionResult === "success",
    executionCount: task.executionCount,
    lastExecutedAt: task.lastExecutedAt,
    executionStatus: task.executionStatus,
    executionResult: task.executionResult,
    taskComplexityType: task.taskComplexityType
  };
}

export const useTaskStore = defineStore("task", () => {
  const tasks = ref<TaskItem[]>([]);
  const loading = ref(false);
  const loaded = ref(false);
  const error = ref<string>("");
  const databasePath = ref<string>("");

  const pendingCount = computed(() => tasks.value.filter((item) => !item.done).length);

  async function bootstrap() {
    if (loaded.value) return;

    loading.value = true;
    error.value = "";

    try {
      const db = await initDatabase();
      databasePath.value = db.dbPath;

      const records = await listTasks();
      tasks.value = records.map(toTaskItem);
      loaded.value = true;
    } catch (err) {
      error.value = err instanceof Error ? err.message : "初始化数据库失败";
    } finally {
      loading.value = false;
    }
  }

  async function addTask(title: string) {
    const value = title.trim();
    if (!value) return;

    error.value = "";

    try {
      const created = await createTask({
        taskId: crypto.randomUUID(),
        taskName: value,
        taskDescription: "",
        taskType: "one_time",
        taskComplexityType: "simple",
        status: "enabled"
      });

      tasks.value.unshift(toTaskItem(created));
    } catch (err) {
      error.value = err instanceof Error ? err.message : "创建任务失败";
    }
  }

  async function toggleTask(id: string) {
    const task = tasks.value.find((item) => item.id === id);
    if (!task) return;

    error.value = "";

    const shouldMarkDone = !task.done;
    const nextStatus: ExecutionStatus = shouldMarkDone ? "success" : "idle";
    const nextResult: ExecutionResult = shouldMarkDone ? "success" : "unknown";

    try {
      const updated = await updateTaskExecution({
        taskId: id,
        executionStatus: nextStatus,
        executionResult: nextResult,
        increaseCount: false
      });

      const nextItem = toTaskItem(updated);
      const index = tasks.value.findIndex((item) => item.id === id);
      if (index >= 0) tasks.value[index] = nextItem;
    } catch (err) {
      error.value = err instanceof Error ? err.message : "更新任务状态失败";
    }
  }

  async function removeTask(id: string) {
    error.value = "";

    try {
      await deleteTask(id);
      tasks.value = tasks.value.filter((item) => item.id !== id);
    } catch (err) {
      error.value = err instanceof Error ? err.message : "删除任务失败";
    }
  }

  return {
    tasks,
    loading,
    loaded,
    error,
    databasePath,
    pendingCount,
    bootstrap,
    addTask,
    toggleTask,
    removeTask
  };
});
