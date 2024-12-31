<script lang="ts">
    import type { Snippet } from 'svelte';
    import Toaster from '@/components/toaster/Toaster.svelte';

    let { children }: { children: Snippet } = $props();
</script>

<header></header>
<main class="flex flex-grow w-full p-2.5">
    {@render children()}
</main>
<footer>
    <Toaster />
</footer>
