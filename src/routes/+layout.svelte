<script>
    import '../app.css';
    import { authState } from '$modules/auth/state.svelte.js';
    import { onMount } from 'svelte';
    import { goto } from '$app/navigation';

    let { children } = $props();

    onMount(() => {
        if (!authState.getUser()) {
            goto('/auth');
        }
    });
</script>

{@render children()}
