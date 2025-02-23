<script lang="ts">
    import { Filter } from 'lucide-svelte';
</script>

<div class="w-1/2">
    <input type="text" class="w-full rounded-md text-sm" placeholder="Search here" />
</div>
<div class="flex w-full justify-end">
    <button class="button-secondary">
        <Filter size={14} class="mt-0.5" />
        Filters
    </button>
</div>
