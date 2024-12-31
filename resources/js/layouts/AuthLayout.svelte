<script lang="ts">
    import type { Snippet } from 'svelte';
    import { Link } from '@inertiajs/svelte';
    import Toaster from '@/components/toaster/Toaster.svelte';

    let { children }: { children: Snippet } = $props();
</script>

<header>
    <nav class="flex w-full py-2.5 px-2.5 mx-auto mt-2">
        <div>
            <Link href="/" class="w-full flex items-center space-x-2">
                <img class="block h-6 w-auto m-auto" src="/images/netbalance-logo.png" alt="Logo" />
                <p class="text-2xl">netbalance</p>
            </Link>
        </div>
    </nav>
</header>
<main class="flex flex-grow w-full p-2.5">
    {@render children()}
</main>
<footer>
    <Toaster />
</footer>
