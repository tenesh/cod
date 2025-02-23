<script lang="ts">
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
            class="my-1 flex w-full items-center rounded-lg px-4 py-2.5 {page.url.pathname === '/app'
                ? 'bg-primary-100'
                : 'hover:bg-secondary-50'}"
        >
            Dashboard
        </a>
        <a
            href="/app/accounts"
            class="my-1 flex w-full items-center rounded-lg px-4 py-2.5 {page.url.pathname.startsWith('/app/accounts')
                ? 'bg-primary-100'
                : 'hover:bg-secondary-50'}"
        >
            Accounts
        </a>
        <a
            href="/app/settings"
            class="my-1 flex w-full items-center rounded-lg px-4 py-2.5 {page.url.pathname.startsWith('/app/settings')
                ? 'bg-primary-100'
                : 'hover:bg-secondary-50'}"
        >
            Settings
        </a>
    </nav>
</div>
