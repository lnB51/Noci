<!-- <script lang="ts">
    import { onMount } from 'svelte'; // if using Svelte, otherwise adapt accordingly
    import { getCurrentWindow, LogicalSize } from '@tauri-apps/api/window';

  onMount(() => {
    getCurrentWindow().show();

    const body = document.body;
    if (!body) return;

    body.addEventListener('mouseenter', () => {
      // getCurrentWindow().setSize(new LogicalSize(800, 600));
      console.log('Mouse entered the body');
      
    });

    body.addEventListener('mouseleave', () => {
      // getCurrentWindow().setSize(new LogicalSize(400, 32));
    });
  });
</script> -->
