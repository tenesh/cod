<script>
    export let text = 'Badge';
    export let size = 'sm';
    export let bgColor = '#e0f2ff';
    export let textColor = '#007aff';
</script>

<span
    class={`inline-flex w-fit items-center justify-center whitespace-nowrap rounded-full font-medium ${size === 'sm' ? 'px-3 py-1 text-xs' : ''} ${size === 'md' ? 'px-4 py-1.5 text-sm' : ''} ${size === 'lg' ? 'px-5 py-2 text-base' : ''} `}
    style={`background-color: ${bgColor}; color: ${textColor};`}
>
    {text}
</span>
