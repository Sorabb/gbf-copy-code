import { writable } from 'svelte/store';
import type { ListInfo } from '../types';

export const lists = writable<ListInfo[]>([]);
export const currentListId = writable<number | null>(null);
