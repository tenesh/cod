<script lang="ts" context="module">
    export type ToastType = 'error' | 'success' | 'warning' | 'other';

    export type ToastData = {
        title: string;
        description: string;
        type: ToastType;
    };

    const {
        elements: { content, title, description, close },
        helpers,
        states: { toasts },
        actions: { portal },
    } = createToaster<ToastData>();

    export const addToast = helpers.addToast;
</script>

<script lang="ts">
    import Icon from '@iconify/svelte';
    import { createToaster, melt } from '@melt-ui/svelte';
    import { flip } from 'svelte/animate';
    import { fly } from 'svelte/transition';
</script>

<div class="fixed right-0 top-0 z-50 m-4 flex flex-col items-end gap-2 md:bottom-0 md:top-auto" use:portal>
    {#each $toasts as { id, data } (id)}
        <div
            class="rounded-lg bg-white border border-gray-200 shadow-lg"
            use:melt={$content(id)}
            animate:flip={{ duration: 500 }}
            in:fly={{ duration: 150, x: '100%' }}
            out:fly={{ duration: 150, x: '100%' }}
        >
            <div class="relative flex w-[24rem] max-w-[calc(100vw-2rem)] items-center justify-between gap-4 p-5">
                <div class="space-y-3">
                    <h4 class="flex items-center gap-2 font-semibold" use:melt={$title(id)}>
                        {data.title}

                        {#if data.type === 'success'}
                            <span class="size-1.5 rounded-full bg-green-500"></span>
                        {:else if data.type === 'warning'}
                            <span class="size-1.5 rounded-full bg-yellow-500"></span>
                        {:else if data.type === 'error'}
                            <span class="size-1.5 rounded-full bg-red-500"></span>
                        {:else}
                            <span class="size-1.5 rounded-full bg-gray-500"></span>
                        {/if}
                    </h4>
                    <div use:melt={$description(id)} class="text-sm">
                        {data.description}
                    </div>
                </div>
                <button
                    class="absolute right-4 top-4 grid size-6 place-items-center rounded-full text-magnum-500 hover:bg-magnum-900/50"
                    use:melt={$close(id)}
                    aria-label="close notification"
                >
                    <Icon icon="material-symbols:close-rounded" style="font-size: 16px;" />
                </button>
            </div>
        </div>
    {/each}
</div>
