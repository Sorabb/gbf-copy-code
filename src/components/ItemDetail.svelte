<script lang="ts">
  import type { Item } from '../types';
  import Countdown from './Countdown.svelte';

  export let item: Item | null;
  export let onCopy: (item: Item) => Promise<void>;

  let copying = false;
  let error = '';

  async function copyAndRemove(item: Item) {
    if (copying) return;
    copying = true;
    error = '';
    try {
      await onCopy(item);
    } catch (reason) {
      error = reason instanceof Error ? reason.message : String(reason);
    } finally {
      copying = false;
    }
  }
</script>

<section class="detail">
  <div class="section-label">数据详情</div>
  {#if item}
    <div class="content">
      <span class="meta">8 位字母数字码</span>
      <div class="code">{item.code}</div>
      <div class="time-block">
        <span>剩余时间</span>
        <Countdown expiresAt={item.expiresAt} large />
      </div>
      <div class="footer">
        <span>{error || '复制成功后将从当前列表移除'}</span>
        <button disabled={copying} on:click={() => copyAndRemove(item)}>{copying ? '复制中…' : '复制'}</button>
      </div>
    </div>
  {:else}
    <div class="empty">
      <div class="empty-icon">⌁</div>
      <h2>等待数据</h2>
      <p>从下方输入区域粘贴一个或多个 8 位码</p>
    </div>
  {/if}
</section>

<style>
  .detail { min-height: 0; flex: 1.2; display: flex; flex-direction: column; }
  .section-label { height: 42px; padding: 0 20px; display: flex; align-items: center; color: var(--muted); font-size: 11px; font-weight: 600; letter-spacing: .08em; text-transform: uppercase; border-bottom: 1px solid var(--border); }
  .content { padding: clamp(24px, 5vw, 60px); flex: 1; display: flex; flex-direction: column; min-height: 0; }
  .meta { color: var(--muted); font-size: 12px; }
  .code { margin-top: 8px; font: 650 clamp(30px, 5vw, 52px)/1.1 var(--mono); letter-spacing: .04em; }
  .time-block { margin-top: auto; display: flex; flex-direction: column; gap: 11px; }
  .time-block > span { color: var(--muted); font-size: 12px; }
  .footer { margin-top: auto; display: flex; justify-content: space-between; align-items: center; color: var(--muted); font-size: 11px; }
  button { min-width: 150px; height: 52px; border: 1px solid color-mix(in srgb, var(--accent) 70%, var(--border)); border-radius: 8px; background: var(--accent); color: #fff; font-size: 15px; font-weight: 650; cursor: pointer; }
  button:hover { filter: brightness(1.08); }
  button:disabled { opacity: .6; cursor: wait; }
  .empty { flex: 1; display: grid; place-content: center; text-align: center; color: var(--muted); }
  .empty-icon { font-size: 44px; opacity: .45; }
  h2 { margin: 10px 0 5px; color: var(--text-secondary); font-size: 16px; }
  p { margin: 0; font-size: 12px; }
</style>
