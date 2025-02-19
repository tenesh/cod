<script lang="ts">
    import { ArrowLeftRight, LayoutDashboard, Settings } from 'lucide-svelte';
    import { page } from '$app/state';
</script>

<div class="fixed flex min-h-screen min-w-[250px] flex-col border-r border-r-secondary-100">
    <div class="mx-2 flex h-[76px] items-center justify-center gap-2 p-4">
        <img src="/img/app.svg" alt="netbalance logo" class="h-auto w-8" />
        <p class="mb-1.5 text-2xl font-bold text-primary-brand">netbalance</p>
    </div>
    <nav class="mx-2 flex flex-col items-center gap-0.5 p-4 text-sm">
        <a
            href="/app"
            class="my-0.5 flex w-full items-center gap-2 rounded-lg px-2 py-2.5 {page.url.pathname === '/app'
                ? 'bg-primary-100'
                : 'hover:bg-secondary-50'}"
        >
            <LayoutDashboard size={18} />
            Dashboard
        </a>
        <a
            href="/app/wallets"
            class="my-0.5 flex w-full items-center gap-2 rounded-lg px-2 py-2.5 {page.url.pathname.startsWith(
                '/app/wallets',
            )
                ? 'bg-primary-100'
                : 'hover:bg-secondary-50'}"
        >
            <ArrowLeftRight size={18} />
            Wallets
        </a>
        <a
            href="/app/setting"
            class="my-0.5 flex w-full items-center gap-2 rounded-lg px-2 py-2.5 {page.url.pathname.startsWith(
                '/app/setting',
            )
                ? 'bg-primary-100'
                : 'hover:bg-secondary-50'}"
        >
            <Settings size={18} />
            Setting
        </a>
    </nav>
</div>
