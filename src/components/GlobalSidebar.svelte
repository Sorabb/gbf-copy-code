<script lang="ts">
  import type { ListInfo } from '../types';
  import brandIcon from '../assets/guild-war-icon.png';

  export let lists: ListInfo[];
  export let currentListId: number | null;
  export let settingsOpen: boolean;
  export let onSelect: (id: number) => void;
  export let onCreate: () => void;
  export let onSettings: () => void;

  const shortName = (name: string) => Array.from(name.trim()).slice(0, 2).join('').toUpperCase() || 'L';
</script>

<nav class="rail" aria-label="全局功能">
  <img class="brand" src={brandIcon} alt="星之古战场主题图标" title="GBF Copy Code" />
  <div class="list-buttons">
    {#each lists as list (list.id)}
      <button
        class:active={!settingsOpen && list.id === currentListId}
        class="list-button"
        title={list.name}
        aria-label={`切换到${list.name}`}
        on:click={() => onSelect(list.id)}
      >{shortName(list.name)}</button>
    {/each}
    <button class="icon-button add" title="创建列表" aria-label="创建列表" on:click={onCreate}>＋</button>
  </div>
  <button
    class:active={settingsOpen}
    class="icon-button settings"
    title="设置"
    aria-label="设置"
    on:click={onSettings}
  >⚙</button>
</nav>

<style>
  .rail {
    width: 64px;
    flex: 0 0 64px;
    display: flex;
    flex-direction: column;
    align-items: center;
    border-right: 1px solid var(--border);
    background: var(--rail);
    padding: 12px 8px;
    gap: 16px;
  }

  .brand {
    width: 36px;
    height: 36px;
    border-radius: 10px;
    display: block;
    object-fit: cover;
    box-shadow: 0 0 0 1px color-mix(in srgb, #d5a956 52%, transparent), 0 5px 14px rgba(0, 0, 0, .34);
  }

  .list-buttons {
    display: flex;
    flex-direction: column;
    gap: 8px;
    width: 100%;
    align-items: center;
    overflow-y: auto;
  }

  button {
    border: 0;
    color: var(--muted);
    background: transparent;
    cursor: pointer;
  }

  .list-button, .icon-button {
    width: 40px;
    height: 40px;
    border-radius: 9px;
    display: grid;
    place-items: center;
    transition: 120ms ease;
  }

  .list-button {
    font: 600 12px/1 var(--sans);
    background: var(--surface-2);
  }

  button:hover { color: var(--text); background: var(--hover); }
  button.active { color: var(--accent-contrast); background: var(--accent-soft); box-shadow: inset 3px 0 var(--accent); }
  .add { font-size: 22px; border: 1px dashed var(--border-strong); }
  .settings { margin-top: auto; font-size: 18px; }
</style>
