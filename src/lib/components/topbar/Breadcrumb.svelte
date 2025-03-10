<script lang="ts">
    import { ChevronRight } from 'lucide-svelte';
    import { page } from '$app/state';

    const capitalize = (str: string) => str.charAt(0).toUpperCase() + str.slice(1);

    let breadcrumbs = $derived(
        page.url.pathname
            .split('/')
            .filter(Boolean)
            .map((segment, index, array) => ({
                label: capitalize(segment),
                href: '/' + array.slice(0, index + 1).join('/'),
            })),
    );
</script>

<div class="flex items-center gap-2.5 text-sm">
    {#if page.url.pathname === '/'}
        <p>Dashboard</p>
    {/if}

    {#if page.url.pathname !== '/' && breadcrumbs.length > 0}
        <a href="/" class="text-primary-brand">Home</a>
        <ChevronRight size={16} class="mt-1" />
        {#each breadcrumbs as breadcrumb, i}
            {#if i < breadcrumbs.length - 1}
                <a href={breadcrumb.href} class="text-primary-brand">{breadcrumb.label}</a>
                <ChevronRight size={16} class="mt-1" />
            {:else}
                <p class="text-black">{breadcrumb.label}</p>
            {/if}
        {/each}
    {/if}
</div>
