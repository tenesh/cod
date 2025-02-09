<script>
    import SideBar from '$lib/common/components/sidebar/SideBar.svelte';
    import TopBar from '$lib/common/components/topbar/TopBar.svelte';

    let { children } = $props();
</script>

<header>
    <TopBar />
    <SideBar />
</header>
<main
    id="content"
    class="fixed left-[250px] top-[76px] mx-auto flex h-full min-w-[800px] flex-col gap-4 overflow-x-auto overflow-y-auto p-10"
>
    {@render children()}
</main>
<footer></footer>
