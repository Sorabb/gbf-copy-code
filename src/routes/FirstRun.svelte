<script lang="ts">
  import brandIcon from '../assets/guild-war-icon.png';

  export let onStart: () => Promise<void>;
  let starting = false;

  async function start() {
    starting = true;
    try { await onStart(); } finally { starting = false; }
  }
</script>

<main class="first-run">
  <section>
    <img class="mark" src={brandIcon} alt="星之古战场主题图标" />
    <p class="eyebrow">首次运行设置</p>
    <h1>选择运行模式</h1>
    <p class="intro">选择数据的保存和连接方式。你可以稍后在设置中查看当前模式。</p>

    <button class="mode selected" aria-pressed="true">
      <span class="radio"><i></i></span>
      <span><strong>本地运行</strong><small>数据仅保存在当前设备，不需要网络连接。</small></span>
      <em>推荐</em>
    </button>
    <button class="mode disabled" disabled>
      <span class="radio"></span>
      <span><strong>联网运行</strong><small>通过主机在多台设备之间同步数据。</small></span>
      <em>即将支持</em>
    </button>

    <button class="start" on:click={start} disabled={starting}>{starting ? '正在初始化…' : '开始使用'}</button>
    <p class="footnote">默认有效期 90 分钟 · SQLite 本地持久化</p>
  </section>
</main>

<style>
  .first-run { min-height: 100vh; display: grid; place-items: center; padding: 32px; background: radial-gradient(circle at 50% -20%, var(--accent-soft), transparent 45%), var(--background); }
  section { width: min(100%, 520px); }
  .mark { width: 52px; height: 52px; display: block; object-fit: cover; border-radius: 12px; margin-bottom: 34px; box-shadow: 0 0 0 1px color-mix(in srgb, #d5a956 55%, transparent), 0 8px 24px rgba(0, 0, 0, .32); }
  .eyebrow { color: var(--accent); text-transform: uppercase; font-size: 11px; font-weight: 700; letter-spacing: .12em; margin: 0 0 8px; }
  h1 { font-size: 30px; letter-spacing: -.03em; margin: 0; }
  .intro { color: var(--muted); line-height: 1.6; font-size: 13px; margin: 12px 0 28px; }
  .mode { width: 100%; min-height: 86px; display: grid; grid-template-columns: 22px 1fr auto; gap: 13px; align-items: center; text-align: left; border: 1px solid var(--border-strong); border-radius: 8px; background: var(--surface); color: var(--text); padding: 15px; margin-bottom: 10px; }
  .mode.selected { border-color: var(--accent); box-shadow: 0 0 0 2px color-mix(in srgb, var(--accent) 12%, transparent); }
  .mode.disabled { opacity: .48; }
  .radio { width: 17px; height: 17px; border: 1px solid var(--border-strong); border-radius: 50%; display: grid; place-items: center; }
  .selected .radio { border-color: var(--accent); }
  .radio i { width: 9px; height: 9px; border-radius: 50%; background: var(--accent); }
  strong, small { display: block; }
  strong { font-size: 14px; margin-bottom: 6px; }
  small { color: var(--muted); font-size: 11px; }
  em { align-self: start; color: var(--accent); background: var(--accent-soft); font-size: 10px; font-style: normal; padding: 4px 7px; border-radius: 4px; }
  .start { width: 100%; height: 42px; margin-top: 18px; border: 0; border-radius: 7px; background: var(--accent); color: #fff; font-weight: 650; cursor: pointer; }
  .start:hover { filter: brightness(1.08); }
  .start:disabled { opacity: .6; cursor: wait; }
  .footnote { text-align: center; color: var(--muted); font-size: 10px; margin-top: 15px; }
</style>
