<script lang="ts">
    import Icon from '@iconify/svelte';
    import { createCollapsible, melt } from '@melt-ui/svelte';
    import { slide } from 'svelte/transition';
    import { Link, page } from '@inertiajs/svelte';

    let routes = [
        { label: 'About', link: '/about' },
        { label: 'Pricing', link: '/pricing' },
        { label: 'Contact', link: '/contact' },
    ];

    let open = false;
    let disabled = false;

    const {
        elements: { root, content, trigger },
        states,
        options,
    } = createCollapsible();
</script>

<nav use:melt={$root} class="flex flex-col max-w-[66rem] w-full py-2.5 px-2.5 mx-auto mt-2">
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
        <button class="lg:hidden" on:click={() => (open = !open)} use:melt={$trigger}>
            {#if open}
                <Icon icon="material-symbols:close-rounded" style="font-size: 24px;" />
            {:else}
                <Icon icon="material-symbols:menu-rounded" style="font-size: 24px;" />
            {/if}
        </button>
    </div>
    {#if open}
        <div use:melt={$content} transition:slide class="flex flex-col w-full my-4">
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
