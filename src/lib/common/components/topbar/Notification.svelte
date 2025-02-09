<script lang="ts">
    import { Bell } from 'lucide-svelte';
    import { createDropdownMenu, melt } from '@melt-ui/svelte';

    const {
        elements: { menu, item, trigger },
        states: { open },
    } = createDropdownMenu();
</script>

<div class="relative">
    <button class="flex cursor-pointer rounded-lg border p-2 hover:bg-secondary-50" use:melt={$trigger}>
        <Bell size={16} />
    </button>
    {#if $open}
        <div
            use:melt={$menu}
            class="max-h-[400px] w-[300px] overflow-y-auto rounded-lg border border-secondary-100 bg-white"
        >
            <div class="flex items-center justify-between border-b border-b-slate-200 p-2">
                <button class="rounded-lg p-2 text-xs font-semibold text-primary-brand hover:bg-primary-50"
                    >Mark all as read</button
                >
                <button class="rounded-lg p-2 text-xs font-semibold text-red-600 hover:bg-red-50">Clear all</button>
            </div>
            <div
                use:melt={$item}
                class="flex cursor-pointer justify-between gap-2 rounded-sm p-3 text-xs text-gray-500 hover:bg-gray-50"
            >
                <div class="flex flex-col gap-1">
                    <p class="font-semibold">Budget</p>
                    <p>Lorem ipsum is a random...</p>
                </div>
                <p>Jul 10</p>
            </div>
            <div use:melt={$item}></div>
        </div>
    {/if}
</div>
