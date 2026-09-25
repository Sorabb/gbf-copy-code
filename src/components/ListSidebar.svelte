<script lang="ts">
  import type { Item, ListInfo } from '../types';
  import Countdown from './Countdown.svelte';

  export let list: ListInfo | null;
  export let items: Item[];
  export let selectedCode: string | null;
  export let onSelectItem: (code: string) => void;
  export let onRename: () => void;
  export let onDelete: () => void;
</script>

<aside class="panel">
  <header>
    <div>
      <span class="eyebrow">当前列表</span>
      <h1>{list?.name ?? '未选择列表'}</h1>
    </div>
    {#if list}
      <div class="actions">
        <button title="重命名列表" aria-label="重命名列表" on:click={onRename}>✎</button>
        <button class="danger" title="删除列表" aria-label="删除列表" on:click={onDelete}>×</button>
      </div>
    {/if}
  </header>

  <div class="summary"><span>活跃数据</span><strong>{items.length}</strong></div>
  <div class="items">
    {#each items as item (item.code)}
      <button class:selected={selectedCode === item.code} on:click={() => onSelectItem(item.code)}>
        <span class="code">{item.code}</span>
        <Countdown expiresAt={item.expiresAt} />
      </button>
    {:else}
      <div class="empty">
        <span>⌁</span>
        <p>暂无数据</p>
        <small>粘贴 8 位码后会显示在这里</small>
      </div>
    {/each}
  </div>
</aside>

<style>
  .panel { width: clamp(220px, 25vw, 300px); flex: 0 0 clamp(220px, 25vw, 300px); border-right: 1px solid var(--border); background: var(--surface); display: flex; flex-direction: column; min-width: 0; }
  header { min-height: 76px; padding: 16px 16px 12px; display: flex; align-items: center; justify-content: space-between; border-bottom: 1px solid var(--border); }
  .eyebrow { display: block; color: var(--muted); font-size: 11px; margin-bottom: 5px; }
  h1 { font-size: 16px; margin: 0; font-weight: 620; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; max-width: 160px; }
  .actions { display: flex; gap: 4px; }
  .actions button { width: 32px; height: 32px; border: 1px solid transparent; border-radius: 6px; color: var(--muted); background: transparent; cursor: pointer; font-size: 16px; }
  .actions button:hover { color: var(--text); background: var(--hover); }
  .actions button.danger:hover { color: var(--danger); border-color: color-mix(in srgb, var(--danger) 25%, transparent); }
  .summary { height: 39px; padding: 0 14px; display: flex; align-items: center; justify-content: space-between; color: var(--muted); font-size: 12px; border-bottom: 1px solid var(--border); }
  .summary strong { color: var(--text); font-weight: 600; }
  .items { overflow-y: auto; flex: 1; padding: 7px; }
  .items > button { width: 100%; height: 46px; padding: 0 10px; border: 0; border-radius: 6px; display: flex; align-items: center; justify-content: space-between; color: var(--text); background: transparent; cursor: pointer; }
  .items > button:hover { background: var(--hover); }
  .items > button.selected { background: var(--accent-soft); box-shadow: inset 2px 0 var(--accent); }
  .code { font: 550 14px/1 var(--mono); letter-spacing: .02em; }
  .empty { min-height: 180px; display: flex; flex-direction: column; align-items: center; justify-content: center; text-align: center; color: var(--muted); }
  .empty > span { font-size: 30px; opacity: .55; }
  .empty p { margin: 9px 0 4px; color: var(--text-secondary); font-size: 13px; }
  .empty small { font-size: 11px; max-width: 160px; line-height: 1.5; }
</style>
