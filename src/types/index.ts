export interface ListInfo {
  id: number;
  name: string;
  createdAt: number;
}

export interface Item {
  listId: number;
  code: string;
  createdAt: number;
  expiresAt: number;
}

export interface AppSettings {
  firstRunCompleted: boolean;
  runMode: 'local';
  lastSelectedListId: number | null;
  databasePath: string;
}

export interface AddCodesResult {
  added: number;
  duplicates: number;
  invalid: number;
}

export interface ExpiredItem {
  listId: number;
  code: string;
}
