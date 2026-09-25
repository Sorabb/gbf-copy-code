<script lang="ts">
  import { onMount } from 'svelte';

  export let expiresAt: number;
  export let large = false;

  let now = Date.now();

  onMount(() => {
    const timer = window.setInterval(() => (now = Date.now()), 1000);
    return () => window.clearInterval(timer);
  });

  $: remaining = Math.max(0, expiresAt - now);
  $: totalSeconds = Math.ceil(remaining / 1000);
  $: hours = Math.floor(totalSeconds / 3600);
  $: minutes = Math.floor((totalSeconds % 3600) / 60);
  $: seconds = totalSeconds % 60;
  $: formatted = [hours, minutes, seconds].map((value) => String(value).padStart(2, '0')).join(':');
</script>

<span class:large class:urgent={remaining <= 5 * 60 * 1000}>{formatted}</span>

<style>
  span {
    color: var(--muted);
    font: 500 12px/1 var(--mono);
    font-variant-numeric: tabular-nums;
  }

  span.large {
    color: var(--text);
    font-size: clamp(32px, 5vw, 56px);
    letter-spacing: -0.04em;
  }

  span.urgent {
    color: var(--danger);
  }
</style>
