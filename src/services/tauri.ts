import { invoke } from '@tauri-apps/api/core';
import type { AddCodesResult, AppSettings, Item, ListInfo } from '../types';

export const api = {
  getSettings: () => invoke<AppSettings>('get_settings'),
  completeFirstRun: () => invoke<AppSettings>('complete_first_run'),
  setLastSelectedList: (listId: number) => invoke<void>('set_last_selected_list', { listId }),
  getLists: () => invoke<ListInfo[]>('get_lists'),
  createList: (name: string) => invoke<ListInfo>('create_list', { name }),
  renameList: (id: number, name: string) => invoke<void>('rename_list', { id, name }),
  deleteList: (id: number) => invoke<void>('delete_list', { id }),
  getItems: (listId: number) => invoke<Item[]>('get_items', { listId }),
  addCodes: (listId: number, codes: string[]) =>
    invoke<AddCodesResult>('add_codes', { listId, codes }),
  deleteCode: (listId: number, code: string) => invoke<void>('delete_code', { listId, code })
};
