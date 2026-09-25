<script lang="ts">
  import type { AddCodesResult, Item, ListInfo } from '../types';
  import ListSidebar from '../components/ListSidebar.svelte';
  import ItemDetail from '../components/ItemDetail.svelte';
  import CodeInput from '../components/CodeInput.svelte';

  export let currentList: ListInfo | null;
  export let items: Item[];
  export let selectedCode: string | null;
  export let onSelectItem: (code: string) => void;
  export let onRenameList: () => void;
  export let onDeleteList: () => void;
  export let onCopyItem: (item: Item) => Promise<void>;
  export let onSubmit: (candidates: string[]) => Promise<AddCodesResult>;

  $: selectedItem = items.find((item) => item.code === selectedCode) ?? items[0] ?? null;
</script>

<div class="main-layout">
  <ListSidebar
    list={currentList}
    {items}
    {selectedCode}
    onSelectItem={onSelectItem}
    onRename={onRenameList}
    onDelete={onDeleteList}
  />
  <main class="workspace">
    <ItemDetail item={selectedItem} onCopy={onCopyItem} />
    <CodeInput disabled={!currentList} submit={onSubmit} />
  </main>
</div>

<style>
  .main-layout { display: flex; flex: 1; min-width: 0; }
  .workspace { display: flex; flex-direction: column; flex: 1; min-width: 0; background: var(--background); }
</style>
