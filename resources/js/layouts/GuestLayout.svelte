<script lang="ts">
    import type { Snippet } from 'svelte';
    import Icon from '@iconify/svelte';
    import { Link, page } from '@inertiajs/svelte';
    import { slide } from 'svelte/transition';
    import { clickOutside } from '@/directives/outside';
    import Toaster from '@/components/toaster/Toaster.svelte';

    let { children }: { children: Snippet } = $props();

    let routes = [
        { label: 'About', link: '/about' },
        { label: 'Pricing', link: '/pricing' },
        { label: 'Contact', link: '/contact' },
    ];

    let isOpen = $state(false);
</script>

<header>
    <nav class="flex flex-col max-w-[66rem] w-full p-2.5 mx-auto mt-2">
        <div class="flex justify-between items-center w-full">
            <div>
                <Link href="/public" class="w-full flex items-center space-x-2">
                    <img class="block h-6 w-auto m-auto lg:h-8" src="/images/netbalance-logo.png" alt="Logo" />
                    <p class="text-2xl lg:text-3xl">netbalance</p>
                </Link>
            </div>
            <div class="hidden lg:block">
                {#each routes as route}
                    <Link
                        href={route.link}
                        class={`px-3 hover:text-gray-600 ${$page.url.startsWith(route.link) ? 'active' : ''}`}
                    >
                        {route.label}
                    </Link>
                {/each}
                {#if $page.props.auth.user}
                    <Link
                        href="/dashboard"
                        class={`px-3 hover:text-gray-600 ${$page.url.startsWith('/dashboard') ? 'active' : ''}`}
                    >
                        Dashboard
                    </Link>
                {/if}
                {#if !$page.props.auth.user}
                    <Link
                        href="/login"
                        class={`px-3 hover:text-gray-600 ${$page.url.startsWith('/login') ? 'active' : ''}`}
                    >
                        Login
                    </Link>
                {/if}
            </div>
            <button
                class="lg:hidden"
                onclick={() => {
                    isOpen = !isOpen;
                }}
            >
                {#if isOpen}
                    <Icon icon="material-symbols:close-rounded" style="font-size: 24px;" />
                {:else}
                    <Icon icon="material-symbols:menu-rounded" style="font-size: 24px;" />
                {/if}
            </button>
        </div>
        {#if isOpen}
            <div
                class="flex flex-col w-full px-4 py-2 bg-gray-100 mt-2 rounded-lg lg:hidden"
                transition:slide
                use:clickOutside={() => {
                    if (!isOpen) {
                        return;
                    }
                    isOpen = !isOpen;
                }}
            >
                {#each routes as route}
                    <Link
                        href={route.link}
                        class={`py-2 hover:text-gray-600 ${$page.url.startsWith(route.link) ? 'active' : ''}`}
                    >
                        {route.label}
                    </Link>
                {/each}
                {#if $page.props.auth.user}
                    <Link
                        href="/dashboard"
                        class={`py-2 hover:text-gray-600 ${$page.url.startsWith('/dashboard') ? 'active' : ''}`}
                    >
                        Dashboard
                    </Link>
                {/if}
                {#if !$page.props.auth.user}
                    <Link
                        href="/login"
                        class={`py-2 hover:text-gray-600 ${$page.url.startsWith('/login') ? 'active' : ''}`}
                    >
                        Login
                    </Link>
                {/if}
            </div>
        {/if}
    </nav>
</header>
<main class="flex flex-grow w-full max-w-[66rem] p-2.5">
    {@render children()}
</main>
<footer>
    <Toaster />
</footer>
