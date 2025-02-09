<script>
    export let text = "Badge";
    export let size = "sm";
    export let bgColor = "#e0f2ff";
    export let textColor = "#007aff";
</script>

<span
    class={` w-fit inline-flex items-center justify-center rounded-full font-medium whitespace-nowrap
    ${size === "sm" ? "text-xs px-3 py-1" : ""}
    ${size === "md" ? "text-sm px-4 py-1.5" : ""}
    ${size === "lg" ? "text-base px-5 py-2" : ""}
  `}
    style={`background-color: ${bgColor}; color: ${textColor};`}
>
  {text}
</span>
