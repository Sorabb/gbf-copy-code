<script lang="ts">
  import type { AddCodesResult } from '../types';

  export let disabled = false;
  export let submit: (candidates: string[]) => Promise<AddCodesResult>;

  let value = '';
  let busy = false;
  let feedback = '';
  let feedbackKind: 'success' | 'warning' | 'error' = 'success';
  let feedbackTimer: number | undefined;

  function candidatesFrom(text: string) {
    return text.trim().split(/[\s,;，；]+/).filter(Boolean);
  }

  function handleInput(event: Event) {
    const input = event.currentTarget as HTMLInputElement;
    const sanitized = input.value.replace(/\s+/g, '');
    value = sanitized;
    input.value = sanitized;
  }

  function showFeedback(result: AddCodesResult) {
    if (result.added > 0) {
      feedbackKind = result.duplicates || result.invalid ? 'warning' : 'success';
      feedback = `已添加 ${result.added} 个${result.duplicates ? `，${result.duplicates} 个已存在` : ''}${result.invalid ? `，${result.invalid} 个无效` : ''}`;
    } else if (result.duplicates > 0 && result.invalid === 0) {
      feedbackKind = 'warning';
      feedback = result.duplicates === 1 ? '已存在' : `${result.duplicates} 个已存在`;
    } else {
      feedbackKind = 'error';
      feedback = '未找到有效的 8 位码';
    }
    window.clearTimeout(feedbackTimer);
    feedbackTimer = window.setTimeout(() => (feedback = ''), 2200);
  }

  async function process(text: string, clearOnFinish = true) {
    if (busy || disabled) return;
    const candidates = candidatesFrom(text);
    if (!candidates.length) return;
    busy = true;
    try {
      const result = await submit(candidates);
      showFeedback(result);
      if (clearOnFinish || result.added > 0 || result.duplicates > 0) value = '';
    } catch (error) {
      feedbackKind = 'error';
      feedback = error instanceof Error ? error.message : String(error);
    } finally {
      busy = false;
    }
  }

  function handlePaste(event: ClipboardEvent) {
    event.preventDefault();
    const text = event.clipboardData?.getData('text') ?? '';
    value = text.replace(/\s+/g, '');
    void process(text);
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key !== 'Enter' || event.shiftKey) return;
    event.preventDefault();
    void process(value, false);
  }
</script>

<section class="input-section">
  <div class="heading">
    <div>
      <span class="label">快速录入</span>
      <h2>粘贴 Code</h2>
    </div>
    {#if feedback}<div class="feedback {feedbackKind}" role="status">{feedback}</div>{/if}
  </div>
  <input
    type="text"
    value={value}
    on:input={handleInput}
    on:paste={handlePaste}
    on:keydown={handleKeydown}
    {disabled}
    spellcheck="false"
    placeholder={disabled ? '请先选择列表' : '粘贴一个或多个 8 位字母数字码…'}
    aria-label="Code 输入区"
  />
  <div class="hint"><span>自动移除换行和空白；批量粘贴仍支持常见分隔符</span><span><kbd>Enter</kbd> 手动提交</span></div>
</section>

<style>
  .input-section { min-height: 158px; flex: 0 0 158px; padding: 20px; border-top: 1px solid var(--border); background: var(--surface); display: flex; flex-direction: column; }
  .heading { display: flex; align-items: flex-end; justify-content: space-between; min-height: 39px; margin-bottom: 13px; }
  .label { color: var(--muted); font-size: 10px; font-weight: 600; letter-spacing: .08em; text-transform: uppercase; }
  h2 { margin: 3px 0 0; font-size: 15px; }
  .feedback { font-size: 12px; padding: 5px 9px; border-radius: 5px; background: var(--surface-2); }
  .feedback.success { color: var(--success); }
  .feedback.warning { color: var(--warning); }
  .feedback.error { color: var(--danger); }
  input { width: 100%; height: 44px; flex: 0 0 44px; border: 1px solid var(--border-strong); border-radius: 7px; padding: 0 13px; outline: none; color: var(--text); background: var(--input); font: 14px/1 var(--mono); }
  input:focus { border-color: var(--accent); box-shadow: 0 0 0 2px color-mix(in srgb, var(--accent) 18%, transparent); }
  input::placeholder { color: var(--muted); font-family: var(--sans); }
  input:disabled { opacity: .55; }
  .hint { display: flex; justify-content: space-between; margin-top: 9px; color: var(--muted); font-size: 10px; }
  kbd { border: 1px solid var(--border-strong); border-radius: 3px; padding: 1px 4px; font: inherit; }
</style>
