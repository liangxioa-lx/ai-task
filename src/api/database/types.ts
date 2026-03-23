export type TaskType = "one_time" | "recurring";
export type TaskComplexityType = "simple" | "complex";
export type TaskStatus = "enabled" | "disabled" | "archived";
export type ExecutionStatus = "idle" | "running" | "success" | "failed" | "canceled";
export type ExecutionResult = "unknown" | "success" | "failed" | "partial";

export interface DatabaseHealth {
  ready: boolean;
  dbPath: string;
}

export interface TaskRecord {
  taskId: string;
  taskName: string;
  taskDescription: string;
  taskType: TaskType;
  taskComplexityType: TaskComplexityType;
  scheduleRule: string | null;
  permissionPolicyJson: string;
  executionCount: number;
  lastExecutedAt: string | null;
  executionStatus: ExecutionStatus;
  executionResult: ExecutionResult;
  flowId: string | null;
  status: TaskStatus;
  createdAt: string;
  updatedAt: string;
}

export interface CreateTaskInput {
  taskId: string;
  taskName: string;
  taskDescription?: string;
  taskType: TaskType;
  taskComplexityType: TaskComplexityType;
  scheduleRule?: string;
  permissionPolicyJson?: string;
  status?: TaskStatus;
}

export interface UpdateTaskExecutionInput {
  taskId: string;
  executionStatus: ExecutionStatus;
  executionResult: ExecutionResult;
  errorMessage?: string;
  executedAt?: string;
  increaseCount?: boolean;
}

export interface ComplexTaskFlowNode {
  nodeId: string;
  flowId: string;
  refTaskId: string;
  nodeType: "simple_task" | "complex_task";
  position: number;
  conditionExpr: string | null;
  onSuccessNext: string | null;
  onFailureNext: string | null;
  createdAt: string;
  updatedAt: string;
}

export interface ComplexTaskFlowRecord {
  flowId: string;
  parentTaskId: string;
  flowName: string | null;
  flowDescription: string | null;
  version: number;
  status: string;
  createdAt: string;
  updatedAt: string;
  nodes: ComplexTaskFlowNode[];
}

export interface UpsertComplexFlowNodeInput {
  nodeId: string;
  refTaskId: string;
  nodeType: "simple_task" | "complex_task";
  position: number;
  conditionExpr?: string;
  onSuccessNext?: string;
  onFailureNext?: string;
}

export interface UpsertComplexFlowInput {
  flowId: string;
  parentTaskId: string;
  flowName?: string;
  flowDescription?: string;
  version?: number;
  status?: string;
  nodes: UpsertComplexFlowNodeInput[];
}

export interface SettingRecord {
  settingKey: string;
  settingValue: string;
  valueType: string;
  createdAt: string;
  updatedAt: string;
}

export interface UpsertSettingInput {
  settingKey: string;
  settingValue: string;
  valueType?: string;
}
