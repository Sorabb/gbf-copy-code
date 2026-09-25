<script lang="ts">
  import { onMount } from 'svelte';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { writeText } from '@tauri-apps/plugin-clipboard-manager';
  import brandIcon from './assets/guild-war-icon.png';
  import GlobalSidebar from './components/GlobalSidebar.svelte';
  import ListDialog from './components/ListDialog.svelte';
  import FirstRun from './routes/FirstRun.svelte';
  import Main from './routes/Main.svelte';
  import Settings from './routes/Settings.svelte';
  import { api } from './services/tauri';
  import type { AddCodesResult, AppSettings, ExpiredItem, Item, ListInfo } from './types';

  let appSettings: AppSettings | null = null;
  let listData: ListInfo[] = [];
  let itemData: Item[] = [];
  let currentListId: number | null = null;
  let selectedCode: string | null = null;
  let settingsOpen = false;
  let loading = true;
  let fatalError = '';
  let listDialog: { mode: 'create' | 'rename' | 'delete'; list: ListInfo | null } | null = null;

  $: currentList = listData.find((list) => list.id === currentListId) ?? null;

  function normalizeSelection() {
    if (!itemData.some((item) => item.code === selectedCode)) selectedCode = itemData[0]?.code ?? null;
  }

  async function loadItems() {
    if (currentListId === null) { itemData = []; selectedCode = null; return; }
    itemData = await api.getItems(currentListId);
    normalizeSelection();
  }

  async function loadLists(preferredId?: number | null) {
    listData = await api.getLists();
    const wanted = preferredId ?? currentListId ?? appSettings?.lastSelectedListId;
    currentListId = listData.some((list) => list.id === wanted) ? wanted! : (listData[0]?.id ?? null);
    if (currentListId !== null) await api.setLastSelectedList(currentListId);
    await loadItems();
  }

  async function initialize() {
    try {
      appSettings = await api.getSettings();
      if (appSettings.firstRunCompleted) await loadLists(appSettings.lastSelectedListId);
    } catch (error) {
      fatalError = error instanceof Error ? error.message : String(error);
    } finally { loading = false; }
  }

  async function completeFirstRun() {
    appSettings = await api.completeFirstRun();
    await loadLists();
  }

  async function selectList(id: number) {
    settingsOpen = false;
    if (id === currentListId) return;
    currentListId = id;
    selectedCode = null;
    await api.setLastSelectedList(id);
    await loadItems();
  }

  async function confirmListAction(name: string) {
    if (!listDialog) return;
    const { mode, list } = listDialog;
    if (mode === 'create') {
      const created = await api.createList(name);
      await loadLists(created.id);
      settingsOpen = false;
    } else if (mode === 'rename' && list) {
      if (name !== list.name) {
        await api.renameList(list.id, name);
        await loadLists(list.id);
      }
    } else if (mode === 'delete' && list) {
      await api.deleteList(list.id);
      currentListId = null;
      selectedCode = null;
      await loadLists();
    }
    listDialog = null;
  }

  async function submitCodes(candidates: string[]): Promise<AddCodesResult> {
    if (currentListId === null) return { added: 0, duplicates: 0, invalid: candidates.length };
    const result = await api.addCodes(currentListId, candidates);
    await loadItems();
    return result;
  }

  async function copyItem(item: Item) {
    await writeText(item.code);
    await api.deleteCode(item.listId, item.code);
    itemData = itemData.filter((existing) => existing.code !== item.code);
    normalizeSelection();
  }

  onMount(() => {
    let unlisten: UnlistenFn | undefined;
    void listen<ExpiredItem[]>('items-expired', (event) => {
      if (event.payload.some((item) => item.listId === currentListId)) {
        const expired = new Set(event.payload.filter((item) => item.listId === currentListId).map((item) => item.code));
        itemData = itemData.filter((item) => !expired.has(item.code));
        normalizeSelection();
      }
    }).then((fn) => (unlisten = fn));
    void initialize();
    return () => unlisten?.();
  });
</script>

{#if loading}
  <div class="loading"><img src={brandIcon} alt="星之古战场主题图标" /><p>正在初始化本地数据库…</p></div>
{:else if fatalError}
  <div class="fatal"><h1>应用初始化失败</h1><p>{fatalError}</p><button on:click={() => location.reload()}>重试</button></div>
{:else if appSettings && !appSettings.firstRunCompleted}
  <FirstRun onStart={completeFirstRun} />
{:else if appSettings}
  <div class="app-shell">
    <GlobalSidebar
      lists={listData}
      {currentListId}
      {settingsOpen}
      onSelect={selectList}
      onCreate={() => (listDialog = { mode: 'create', list: null })}
      onSettings={() => (settingsOpen = true)}
    />
    {#if settingsOpen}
      <Settings settings={appSettings} onBack={() => (settingsOpen = false)} />
    {:else}
      <Main
        {currentList}
        items={itemData}
        {selectedCode}
        onSelectItem={(code) => (selectedCode = code)}
        onRenameList={() => currentList && (listDialog = { mode: 'rename', list: currentList })}
        onDeleteList={() => currentList && (listDialog = { mode: 'delete', list: currentList })}
        onCopyItem={copyItem}
        onSubmit={submitCodes}
      />
    {/if}
    {#if listDialog}
      <ListDialog
        mode={listDialog.mode}
        listName={listDialog.list?.name ?? `列表 ${listData.length + 1}`}
        itemCount={listDialog.list?.id === currentListId ? itemData.length : 0}
        onConfirm={confirmListAction}
        onClose={() => (listDialog = null)}
      />
    {/if}
  </div>
{/if}

<style>
  .app-shell { height: 100vh; display: flex; overflow: hidden; }
  .loading, .fatal { min-height: 100vh; display: grid; place-content: center; justify-items: center; color: var(--muted); }
  .loading img { width: 52px; height: 52px; display: block; object-fit: cover; border-radius: 12px; box-shadow: 0 0 0 1px color-mix(in srgb, #d5a956 55%, transparent), 0 8px 24px rgba(0, 0, 0, .32); }
  .loading p { font-size: 12px; margin-top: 15px; }
  .fatal { text-align: center; padding: 30px; }
  .fatal h1 { color: var(--text); font-size: 20px; }
  .fatal p { max-width: 620px; font: 12px/1.6 var(--mono); }
  .fatal button { border: 1px solid var(--border-strong); background: var(--surface); color: var(--text); border-radius: 6px; padding: 7px 14px; cursor: pointer; }
</style>
