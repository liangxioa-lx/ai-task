import { dbInvoke } from "./client";
import type { ComplexTaskFlowRecord, UpsertComplexFlowInput } from "./types";

export async function upsertComplexFlow(input: UpsertComplexFlowInput): Promise<ComplexTaskFlowRecord> {
  return dbInvoke<ComplexTaskFlowRecord>("upsert_complex_flow", { input });
}

export async function getComplexFlowByTask(taskId: string): Promise<ComplexTaskFlowRecord | null> {
  const result = await dbInvoke<ComplexTaskFlowRecord | null>("get_complex_flow_by_task", { taskId });
  return result ?? null;
}
