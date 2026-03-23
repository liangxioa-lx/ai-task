import { dbInvoke } from "./client";
import type {
  CreateTaskInput,
  DatabaseHealth,
  TaskRecord,
  UpdateTaskExecutionInput
} from "./types";

export async function initDatabase(): Promise<DatabaseHealth> {
  return dbInvoke<DatabaseHealth>("init_database");
}

export async function listTasks(): Promise<TaskRecord[]> {
  return dbInvoke<TaskRecord[]>("list_tasks");
}

export async function createTask(input: CreateTaskInput): Promise<TaskRecord> {
  return dbInvoke<TaskRecord>("create_task", { input });
}

export async function deleteTask(taskId: string): Promise<void> {
  return dbInvoke<void>("delete_task", { taskId });
}

export async function updateTaskExecution(input: UpdateTaskExecutionInput): Promise<TaskRecord> {
  return dbInvoke<TaskRecord>("update_task_execution", { input });
}
