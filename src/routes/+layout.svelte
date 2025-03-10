<script>
    import '../app.css';
    import TopBar from '$lib/components/topbar/TopBar.svelte';
    import SideBar from '$lib/components/sidebar/SideBar.svelte';

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
