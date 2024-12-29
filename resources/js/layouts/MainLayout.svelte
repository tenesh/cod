<script lang="ts">
    import type { Snippet } from 'svelte';
    import NavBar from '@/components/NavBar.svelte';

    let { children }: { children: Snippet } = $props();
</script>

<header>
    <NavBar />
</header>
<main class="flex w-full max-w-[66rem] py-2.5 px-2.5 mx-auto">
    {@render children()}
</main>
<footer></footer>
