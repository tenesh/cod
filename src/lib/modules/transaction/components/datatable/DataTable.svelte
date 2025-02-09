<script lang="ts">
    import { Filter, Trash2 } from 'lucide-svelte';
    import Search from '$lib/modules/transaction/components/datatable/Search.svelte';
    import Filters from '$lib/modules/transaction/components/datatable/Filters.svelte';
    import Exports from '$lib/modules/transaction/components/datatable/Exports.svelte';
    import Pagination from '$lib/modules/transaction/components/datatable/Pagination.svelte';
    import Table from '$lib/modules/transaction/components/datatable/Table.svelte';
</script>

<div class="flex flex-col gap-4">
    <div class="flex justify-between items-center">
        <Search />
        <div class="flex gap-2">
            <Filters />
            <Exports />
        </div>
    </div>
    <Table />
    <Pagination />
</div>