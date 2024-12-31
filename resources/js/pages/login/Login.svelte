<script lang="ts">
    import { page } from '@inertiajs/svelte';
    import Icon from '@iconify/svelte';
    import AuthLayout from '@/layouts/AuthLayout.svelte';
    import { addToast } from '@/components/toaster/Toaster.svelte';

    if ($page.props.errors.login) {
        addToast({
            data: {
                title: 'Error',
                description: $page.props.errors.login,
                type: 'error',
            },
        });
    }
</script>

<svelte:head>
    <title>Login</title>
    <meta
        name="description"
        content="Access your NetBalance account to track expenses, manage budgets, and gain financial insights anytime, anywhere."
    />
</svelte:head>

<AuthLayout>
    <div class="m-auto p-4 sm:p-7">
        <h1 class="block text-2xl font-bold text-gray-800">Sign in effortlessly and securely</h1>
        <div class="flex flex-col mt-5 space-y-4">
            <a
                class="w-full py-3 px-4 inline-flex justify-center items-center gap-x-2 text-sm font-medium rounded-lg border border-gray-200 bg-white text-gray-800 shadow-sm hover:bg-gray-50 focus:outline-none focus:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none"
                href={route('auth.google')}
            >
                <Icon icon="flat-color-icons:google" width="1.8em" />
                Continue with Google
            </a>
            <a
                class="w-full py-3 px-4 inline-flex justify-center items-center gap-x-2 text-sm font-medium rounded-lg border border-gray-200 bg-white text-gray-800 shadow-sm hover:bg-gray-50 focus:outline-none focus:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none"
                href={route('auth.twitter')}
            >
                <Icon icon="devicon:twitter" width="1.2em" />
                Continue with X
            </a>
        </div>
    </div>
</AuthLayout>
