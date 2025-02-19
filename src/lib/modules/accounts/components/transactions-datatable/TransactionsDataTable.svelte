<script lang="ts">
    import Search from '$lib/modules/accounts/components/transactions-datatable/Search.svelte';
    import Filters from '$lib/modules/accounts/components/transactions-datatable/Filters.svelte';
    import Pagination from '$lib/modules/accounts/components/transactions-datatable/Pagination.svelte';
    import Table from '$lib/modules/accounts/components/transactions-datatable/Table.svelte';
</script>

<div class="flex flex-col gap-4">
    <div class="space-between flex items-center gap-4">
        <Search />
        <Filters />
    </div>
    <Table />
    <Pagination />
</div>
