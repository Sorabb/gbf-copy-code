<script lang="ts">
  import type { AppSettings } from '../types';
  export let settings: AppSettings;
  export let onBack: () => void;
</script>

<main class="settings-page">
  <header>
    <div><span>应用设置</span><h1>设置</h1></div>
    <button on:click={onBack}>返回列表</button>
  </header>
  <section>
    <h2>常规</h2>
    <div class="row">
      <div><strong>运行模式</strong><small>数据当前的存储和连接方式</small></div>
      <span class="value"><i></i> 本地运行</span>
    </div>
    <div class="row">
      <div><strong>默认有效期</strong><small>新录入数据的保存时长</small></div>
      <span class="value">90 分钟</span>
    </div>
  </section>
  <section>
    <h2>存储</h2>
    <div class="row path-row">
      <div><strong>数据库路径</strong><small>SQLite 数据文件位于当前设备的应用数据目录</small></div>
      <code title={settings.databasePath}>{settings.databasePath}</code>
    </div>
  </section>
  <section>
    <h2>联网模式</h2>
    <div class="notice"><span>◇</span><div><strong>暂不可用</strong><p>主机和客户端模式将在后续版本中提供。</p></div></div>
  </section>
</main>

<style>
  .settings-page { flex: 1; overflow-y: auto; background: var(--background); }
  header { height: 76px; padding: 0 clamp(24px, 5vw, 64px); display: flex; justify-content: space-between; align-items: center; border-bottom: 1px solid var(--border); background: var(--surface); }
  header span { color: var(--muted); font-size: 10px; text-transform: uppercase; letter-spacing: .08em; }
  h1 { margin: 3px 0 0; font-size: 19px; }
  header button { border: 1px solid var(--border-strong); border-radius: 6px; color: var(--text-secondary); background: transparent; padding: 7px 12px; cursor: pointer; }
  header button:hover { color: var(--text); background: var(--hover); }
  section { margin: 38px clamp(24px, 8vw, 96px); max-width: 850px; }
  h2 { color: var(--muted); font-size: 11px; text-transform: uppercase; letter-spacing: .1em; margin: 0 0 10px; }
  .row { min-height: 72px; border-top: 1px solid var(--border); display: flex; align-items: center; justify-content: space-between; gap: 24px; }
  .row:last-child { border-bottom: 1px solid var(--border); }
  strong, small { display: block; }
  strong { font-size: 13px; font-weight: 600; }
  small { color: var(--muted); font-size: 11px; margin-top: 5px; }
  .value { color: var(--text-secondary); font-size: 12px; white-space: nowrap; }
  .value i { display: inline-block; width: 7px; height: 7px; border-radius: 50%; background: var(--success); margin-right: 6px; }
  code { max-width: 55%; overflow: hidden; text-overflow: ellipsis; color: var(--muted); font: 11px/1.4 var(--mono); white-space: nowrap; }
  .notice { display: flex; gap: 13px; padding: 17px; border: 1px solid var(--border); background: var(--surface); border-radius: 7px; color: var(--muted); }
  .notice strong { color: var(--text-secondary); }
  .notice p { margin: 5px 0 0; font-size: 11px; }
  @media (max-width: 680px) { .path-row { align-items: flex-start; flex-direction: column; padding: 16px 0; } code { max-width: 100%; } }
</style>
