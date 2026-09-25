<script lang="ts">
  import { onMount } from 'svelte';

  export let mode: 'create' | 'rename' | 'delete';
  export let listName = '';
  export let itemCount = 0;
  export let onConfirm: (name: string) => Promise<void>;
  export let onClose: () => void;

  let name = mode === 'create' ? listName : listName;
  let busy = false;
  let error = '';
  let input: HTMLInputElement | undefined;
  let primaryButton: HTMLButtonElement | undefined;

  $: title = mode === 'create' ? '新建列表' : mode === 'rename' ? '编辑列表标题' : '删除列表';
  $: description = mode === 'create'
    ? '创建一个独立列表，用于保存另一组 Code。'
    : mode === 'rename'
      ? '修改仅影响列表标题，不会改变其中的数据。'
      : `“${listName}”中的 ${itemCount} 条活跃数据也会一并删除。`;

  onMount(() => {
    if (input) {
      input.focus();
      input.select();
    } else {
      primaryButton?.focus();
    }
  });

  function close() {
    if (!busy) onClose();
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape') close();
  }

  async function submit() {
    if (busy) return;
    const normalized = name.trim();
    if (mode !== 'delete') {
      if (!normalized) {
        error = '列表名称不能为空';
        input?.focus();
        return;
      }
      if (Array.from(normalized).length > 80) {
        error = '列表名称不能超过 80 个字符';
        input?.focus();
        return;
      }
    }

    busy = true;
    error = '';
    try {
      await onConfirm(normalized);
    } catch (reason) {
      error = reason instanceof Error ? reason.message : String(reason);
      busy = false;
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="overlay">
  <div class="dialog" role="dialog" aria-modal="true" aria-labelledby="list-dialog-title">
    <header>
      <div class:danger-mark={mode === 'delete'} class="mark">{mode === 'create' ? '＋' : mode === 'rename' ? '✎' : '×'}</div>
      <div>
        <h2 id="list-dialog-title">{title}</h2>
        <p>{description}</p>
      </div>
    </header>

    <form on:submit|preventDefault={submit}>
      {#if mode !== 'delete'}
        <label for="list-name">列表名称</label>
        <input
          id="list-name"
          bind:this={input}
          bind:value={name}
          maxlength="80"
          autocomplete="off"
          spellcheck="false"
          disabled={busy}
          on:input={() => (error = '')}
        />
      {:else}
        <div class="warning"><strong>此操作无法撤销</strong><span>删除后，该列表及其全部 Code 将从本地数据库中移除。</span></div>
      {/if}

      {#if error}<div class="error" role="alert">{error}</div>{/if}

      <footer>
        <button type="button" class="secondary" disabled={busy} on:click={close}>取消</button>
        <button
          bind:this={primaryButton}
          type="submit"
          class:destructive={mode === 'delete'}
          disabled={busy}
        >{busy ? '处理中…' : mode === 'create' ? '创建列表' : mode === 'rename' ? '保存标题' : '确认删除'}</button>
      </footer>
    </form>
  </div>
</div>

<style>
  .overlay { position: fixed; inset: 0; z-index: 100; display: grid; place-items: center; padding: 24px; background: rgba(4, 6, 10, .68); backdrop-filter: blur(3px); }
  .dialog { width: min(100%, 430px); border: 1px solid var(--border-strong); border-radius: 10px; background: var(--surface); box-shadow: 0 24px 80px rgba(0, 0, 0, .42); overflow: hidden; }
  header { display: flex; gap: 14px; padding: 22px 22px 18px; border-bottom: 1px solid var(--border); }
  .mark { width: 34px; height: 34px; flex: 0 0 34px; display: grid; place-items: center; border-radius: 8px; color: var(--accent-contrast); background: var(--accent-soft); font-size: 17px; font-weight: 650; }
  .mark.danger-mark { color: var(--danger); background: color-mix(in srgb, var(--danger) 12%, transparent); }
  h2 { margin: 0; color: var(--text); font-size: 17px; }
  p { margin: 6px 0 0; color: var(--muted); font-size: 11px; line-height: 1.55; }
  form { padding: 19px 22px 21px; }
  label { display: block; margin-bottom: 7px; color: var(--text-secondary); font-size: 11px; font-weight: 600; }
  input { width: 100%; height: 42px; border: 1px solid var(--border-strong); border-radius: 7px; padding: 0 11px; outline: none; color: var(--text); background: var(--input); }
  input:focus { border-color: var(--accent); box-shadow: 0 0 0 2px color-mix(in srgb, var(--accent) 18%, transparent); }
  .warning { display: flex; flex-direction: column; gap: 5px; padding: 13px; border: 1px solid color-mix(in srgb, var(--danger) 25%, var(--border)); border-radius: 7px; background: color-mix(in srgb, var(--danger) 7%, transparent); }
  .warning strong { color: var(--danger); font-size: 12px; }
  .warning span { color: var(--muted); font-size: 11px; line-height: 1.5; }
  .error { margin-top: 10px; color: var(--danger); font-size: 11px; }
  footer { display: flex; justify-content: flex-end; gap: 8px; margin-top: 20px; }
  button { min-width: 88px; height: 36px; border: 1px solid var(--accent); border-radius: 6px; color: #fff; background: var(--accent); font-size: 12px; font-weight: 600; cursor: pointer; }
  button:hover { filter: brightness(1.08); }
  button:disabled { opacity: .55; cursor: wait; }
  button.secondary { border-color: var(--border-strong); color: var(--text-secondary); background: transparent; }
  button.secondary:hover { background: var(--hover); filter: none; }
  button.destructive { border-color: var(--danger); background: var(--danger); }
</style>
