<script lang="ts">
    import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
    import { type SetupFailed, SetupFailedEvent } from '$lib/events/types';
    import { onDestroy, onMount } from 'svelte';

    let listener: (() => void) | null = null;
    let failed = false;

    const appWebview = getCurrentWebviewWindow();

    onMount(async () => {
        listener = await appWebview.listen<SetupFailed>(SetupFailedEvent, (event) => {
            failed = !!event.payload.message;
        });
    });

    onDestroy(() => {
        if (listener) listener();
    });
</script>

<main class="relative flex min-h-screen w-full flex-col items-center justify-center gap-2 overflow-hidden">
    {#if failed}
        <div class="flex flex-col gap-2">
            <h1 class="text-2xl text-red-600">Something went wrong</h1>
            <p class="text-md">We couldn't start the program, try repairing the app or contact support.</p>
        </div>
    {:else}
        <p class="text-5xl font-bold text-primary-brand">netbalance</p>
        <div
            class="absolute inset-0 flex items-center justify-center bg-white/5 [mask-image:linear-gradient(to_bottom,white,transparent)]"
        >
            {#each { length: 10 } as _, i}
                <div
                    class="bg-foreground/30 absolute left-1/2 top-1/2 translate-x-1/2 translate-y-1/2 animate-ripple rounded-full border shadow-xl [--i:{i}]"
                    style="width: {510 + i * 70}px;
         height: {510 + i * 70}px;
         opacity: {0.24 - i * 0.03};
         animation-delay: {i * 0.08}s;
         border-style:{i === 10 - 1 ? 'dashed' : 'solid'};
         border-width: 1px;
         border-color: rgba(var(--foreground-rgb), {(5 + i * 5) / 100});
        "
                ></div>
            {/each}
        </div>
    {/if}
</main>
