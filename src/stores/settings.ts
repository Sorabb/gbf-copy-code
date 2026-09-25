import { writable } from 'svelte/store';
import type { AppSettings } from '../types';

export const settings = writable<AppSettings | null>(null);
