<script lang="ts">
    import { clickOutside } from '@/directives/outside';
    import Icon from '@iconify/svelte';
    import { melt } from '@melt-ui/svelte';
    import { slide } from 'svelte/transition';
    import { Link, page } from '@inertiajs/svelte';

    let routes = [
        { label: 'About', link: '/about' },
        { label: 'Pricing', link: '/pricing' },
        { label: 'Contact', link: '/contact' },
    ];

    let isOpen = $state(false);
</script>

<nav class="flex flex-col max-w-[66rem] w-full py-2.5 px-2.5 mx-auto mt-2">
    <div class="flex justify-between items-center w-full">
        <div>
            <Link href="/" class="w-full flex items-center space-x-2">
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
                    href="/register"
                    class={`px-3 hover:text-gray-600 ${$page.url.startsWith('/register') ? 'active' : ''}`}
                >
                    Register
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
            class="flex flex-col w-full px-4 py-2 bg-gray-100 mt-2 rounded-lg"
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
                    href="/register"
                    class={`py-2 hover:text-gray-600 ${$page.url.startsWith('/register') ? 'active' : ''}`}
                >
                    Register
                </Link>
            {/if}
        </div>
    {/if}
</nav>
