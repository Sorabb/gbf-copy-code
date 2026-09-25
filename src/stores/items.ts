import { writable } from 'svelte/store';
import type { Item } from '../types';

export const items = writable<Item[]>([]);
export const selectedCode = writable<string | null>(null);
