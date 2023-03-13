// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
import { invoke } from "@tauri-apps/api/tauri";

export const getDirs = (path: string): Promise<string[]> => {
  return invoke<string[]>('get_dirs', { path })
  .catch(err => {
    console.log(err)
    return []
  })
}