import { dbInvoke } from "./client";
import type { SettingRecord, UpsertSettingInput } from "./types";

export async function listSettings(): Promise<SettingRecord[]> {
  return dbInvoke<SettingRecord[]>("list_settings");
}

export async function getSetting(settingKey: string): Promise<SettingRecord | null> {
  const result = await dbInvoke<SettingRecord | null>("get_setting", { settingKey });
  return result ?? null;
}

export async function upsertSetting(input: UpsertSettingInput): Promise<SettingRecord> {
  return dbInvoke<SettingRecord>("upsert_setting", { input });
}

export async function deleteSetting(settingKey: string): Promise<void> {
  return dbInvoke<void>("delete_setting", { settingKey });
}
