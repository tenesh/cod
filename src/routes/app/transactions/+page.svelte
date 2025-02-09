<script lang="ts">
    import DataTable from '$lib/modules/transaction/components/datatable/DataTable.svelte';
</script>

<div class="flex items-end gap-2">
    <a
        href="/app/transactions/new"
        class="ml-auto rounded-lg bg-primary-brand px-4 py-2 text-sm text-white hover:bg-primary-600"
    >
        Add Transaction
    </a>
</div>
<DataTable />
