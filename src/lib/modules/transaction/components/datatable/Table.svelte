<script lang="ts">
    import { ArrowUp, ArrowDown, Eye } from 'lucide-svelte';
    import Badge from '$lib/common/components/badge/Badge.svelte';

    let sortCreatedDate = $state('ascending');
    let sortAmount = $state('ascending');
</script>

<div class="table-wrapper">
    <table>
        <thead class="">
            <tr class="row head-row">
                <th class="column action-column">
                    <input type="checkbox" id="" class="checkbox" />
                </th>
                <th class="column text-column date-column">
                    <p>Created at</p>
                    <button class="button-icon" onclick={() => (sortCreatedDate = 'descending')}>
                        <ArrowUp size={14} class="mt-0.5" />
                    </button>
                </th>
                <th class="column text-column type-column">
                    <p>Type</p>
                </th>
                <th class="column text-column party-column">
                    <p>Party</p>
                </th>
                <th class="column text-column amount-column">
                    <p>Amount</p>
                    <button class="button-icon" onclick={() => (sortAmount = 'descending')}>
                        <ArrowUp size={14} class="mt-0.5" />
                    </button>
                </th>
                <th class="column text-column medium-column">
                    <p>Medium</p>
                </th>
                <th class="column badge-column category-column">
                    <p>Categories</p>
                </th>
                <th class="column text-column note-column">
                    <p>Note</p>
                </th>
                <th class="column text-column status-column">
                    <p>Status</p>
                </th>
                <th class="column action-column"> </th>
            </tr>
        </thead>
        <tbody>
            <tr class="row body-row">
                <td class="column checkbox-column action-column">
                    <input type="checkbox" id="" class="checkbox" />
                </td>
                <td class="column text-column date-column">
                    <p>25 January 2025</p>
                </td>
                <td class="column text-column type-column">
                    <Badge text="Expense" size="sm" />
                </td>
                <td class="column text-column party-column">
                    <p>Ramlee bukit bintang bills</p>
                </td>
                <td class="column text-column amount-column">
                    <p>RM 3.00</p>
                </td>
                <td class="column text-column medium-column">
                    <p>Credit card</p>
                </td>
                <td class="column badge-column category-column">
                    <Badge text="ipsum dolor" size="sm" />
                    <Badge text="Household" size="sm" />
                    <Badge text="Shopping" size="sm" />
                </td>
                <td class="column text-column note-column">
                    <p>
                        Lorem, ipsum dolor sit amet consectetur adipisicing elit. Ducimus facere beatae delectus quo
                        aspernatur impedit, exercitationem quaerat.
                    </p>
                </td>
                <td class="column text-column status-column">
                    <Badge text="Completed" size="sm" />
                </td>
                <td class="column action-column">
                    <button class="button-icon">
                        <Eye size={16} />
                    </button>
                </td>
            </tr>
            <tr class="row body-row">
                <td class="column checkbox-column action-column">
                    <input type="checkbox" id="" class="checkbox" />
                </td>
                <td class="column text-column date-column">
                    <p>25 January 2025</p>
                </td>
                <td class="column text-column type-column">
                    <Badge text="Expense" size="sm" />
                </td>
                <td class="column text-column party-column">
                    <p>Ramlee bukit bintang bills</p>
                </td>
                <td class="column text-column amount-column">
                    <p>RM 3.00</p>
                </td>
                <td class="column text-column medium-column">
                    <p>Credit card</p>
                </td>
                <td class="column badge-column category-column">
                    <Badge text="ipsum dolor" size="sm" />
                    <Badge text="Household" size="sm" />
                    <Badge text="Shopping" size="sm" />
                </td>
                <td class="column text-column note-column">
                    <p>
                        Lorem, ipsum dolor sit amet consectetur adipisicing elit. Ducimus facere beatae delectus quo
                        aspernatur impedit, exercitationem quaerat.
                    </p>
                </td>
                <td class="column text-column status-column">
                    <Badge text="Completed" size="sm" />
                </td>
                <td class="column action-column">
                    <button class="button-icon">
                        <Eye size={16} />
                    </button>
                </td>
            </tr>
            <tr class="row body-row">
                <td class="column checkbox-column action-column">
                    <input type="checkbox" id="" class="checkbox" />
                </td>
                <td class="column text-column date-column">
                    <p>25 January 2025</p>
                </td>
                <td class="column text-column type-column">
                    <Badge text="Expense" size="sm" />
                </td>
                <td class="column text-column party-column">
                    <p>Ramlee bukit bintang bills</p>
                </td>
                <td class="column text-column amount-column">
                    <p>RM 3.00</p>
                </td>
                <td class="column text-column medium-column">
                    <p>Credit card</p>
                </td>
                <td class="column badge-column category-column">
                    <Badge text="ipsum dolor" size="sm" />
                    <Badge text="Household" size="sm" />
                    <Badge text="Shopping" size="sm" />
                </td>
                <td class="column text-column note-column">
                    <p>
                        Lorem, ipsum dolor sit amet consectetur adipisicing elit. Ducimus facere beatae delectus quo
                        aspernatur impedit, exercitationem quaerat.
                    </p>
                </td>
                <td class="column text-column status-column">
                    <Badge text="Completed" size="sm" />
                </td>
                <td class="column action-column">
                    <button class="button-icon">
                        <Eye size={16} />
                    </button>
                </td>
            </tr>
            <tr class="row body-row">
                <td class="column checkbox-column action-column">
                    <input type="checkbox" id="" class="checkbox" />
                </td>
                <td class="column text-column date-column">
                    <p>25 January 2025</p>
                </td>
                <td class="column text-column type-column">
                    <Badge text="Expense" size="sm" />
                </td>
                <td class="column text-column party-column">
                    <p>Ramlee bukit bintang bills</p>
                </td>
                <td class="column text-column amount-column">
                    <p>RM 3.00</p>
                </td>
                <td class="column text-column medium-column">
                    <p>Credit card</p>
                </td>
                <td class="column badge-column category-column">
                    <Badge text="ipsum dolor" size="sm" />
                    <Badge text="Household" size="sm" />
                    <Badge text="Shopping" size="sm" />
                </td>
                <td class="column text-column note-column">
                    <p>
                        Lorem, ipsum dolor sit amet consectetur adipisicing elit. Ducimus facere beatae delectus quo
                        aspernatur impedit, exercitationem quaerat.
                    </p>
                </td>
                <td class="column text-column status-column">
                    <Badge text="Completed" size="sm" />
                </td>
                <td class="column action-column">
                    <button class="button-icon">
                        <Eye size={16} />
                    </button>
                </td>
            </tr>
            <tr class="row body-row">
                <td class="column checkbox-column action-column">
                    <input type="checkbox" id="" class="checkbox" />
                </td>
                <td class="column text-column date-column">
                    <p>25 January 2025</p>
                </td>
                <td class="column text-column type-column">
                    <Badge text="Expense" size="sm" />
                </td>
                <td class="column text-column party-column">
                    <p>Ramlee bukit bintang bills</p>
                </td>
                <td class="column text-column amount-column">
                    <p>RM 3.00</p>
                </td>
                <td class="column text-column medium-column">
                    <p>Credit card</p>
                </td>
                <td class="column badge-column category-column">
                    <Badge text="ipsum dolor" size="sm" />
                    <Badge text="Household" size="sm" />
                    <Badge text="Shopping" size="sm" />
                </td>
                <td class="column text-column note-column">
                    <p>
                        Lorem, ipsum dolor sit amet consectetur adipisicing elit. Ducimus facere beatae delectus quo
                        aspernatur impedit, exercitationem quaerat.
                    </p>
                </td>
                <td class="column text-column status-column">
                    <Badge text="Completed" size="sm" />
                </td>
                <td class="column action-column">
                    <button class="button-icon">
                        <Eye size={16} />
                    </button>
                </td>
            </tr>
            <tr class="row body-row">
                <td class="column checkbox-column action-column">
                    <input type="checkbox" id="" class="checkbox" />
                </td>
                <td class="column text-column date-column">
                    <p>25 January 2025</p>
                </td>
                <td class="column text-column type-column">
                    <Badge text="Expense" size="sm" />
                </td>
                <td class="column text-column party-column">
                    <p>Ramlee bukit bintang bills</p>
                </td>
                <td class="column text-column amount-column">
                    <p>RM 3.00</p>
                </td>
                <td class="column text-column medium-column">
                    <p>Credit card</p>
                </td>
                <td class="column badge-column category-column">
                    <Badge text="ipsum dolor" size="sm" />
                    <Badge text="Household" size="sm" />
                    <Badge text="Shopping" size="sm" />
                </td>
                <td class="column text-column note-column">
                    <p>
                        Lorem, ipsum dolor sit amet consectetur adipisicing elit. Ducimus facere beatae delectus quo
                        aspernatur impedit, exercitationem quaerat.
                    </p>
                </td>
                <td class="column text-column status-column">
                    <Badge text="Completed" size="sm" />
                </td>
                <td class="column action-column">
                    <button class="button-icon">
                        <Eye size={16} />
                    </button>
                </td>
            </tr>
        </tbody>
    </table>
</div>

<style lang="postcss">
    .table-wrapper {
        @apply flex w-full flex-col overflow-x-auto rounded-lg border border-secondary-100;
    }

    .checkbox {
        cursor: pointer;
        @apply h-4 w-4 rounded-sm border-gray-300 bg-gray-100 text-blue-600 focus:ring-transparent;
    }

    table {
        min-width: 1200px;
    }

    tbody tr:not(:last-child) {
        @apply border-b border-secondary-100;
    }

    .row {
        @apply flex items-center justify-start gap-4 hover:bg-primary-50;
    }

    .head-row {
        @apply bg-primary-100 p-3 text-sm text-neutral-600;
    }

    .body-row {
        @apply p-3 text-sm text-neutral-600;
    }

    .column {
        @apply flex w-[100px] flex-wrap items-center gap-2;
    }

    .action-column {
        @apply mt-1 w-[20px] p-0;
    }

    .type-column,
    .status-column,
    .date-column,
    .medium-column {
        @apply w-[150px];
    }

    .amount-column,
    .party-column,
    .category-column {
        @apply w-[250px];
    }

    .note-column {
        @apply min-w-[300px] flex-grow;
    }
</style>
