<script lang="ts">
    export const mainCircleSize: number = 510;
    export const mainCircleOpacity: number = 0.24;
    export const numCircles: number = 10;
</script>

<main class="relative flex min-h-screen w-full flex-col items-center justify-center gap-2 overflow-hidden">
    <img src="/img/app.svg" alt="netbalance logo" class="h-auto w-[200px]" />
    <p class="text-5xl font-bold text-primary-brand">netbalance</p>
    <div
        class="absolute inset-0 flex items-center justify-center bg-white/5 [mask-image:linear-gradient(to_bottom,white,transparent)]"
    >
        {#each { length: numCircles } as _, i}
            <div
                class="bg-foreground/30 absolute left-1/2 top-1/2 translate-x-1/2 translate-y-1/2 animate-ripple rounded-full border shadow-xl [--i:{i}]"
                style="width: {mainCircleSize + i * 70}px;
         height: {mainCircleSize + i * 70}px;
         opacity: {mainCircleOpacity - i * 0.03};
         animation-delay: {i * 0.08}s;
         border-style:{i === numCircles - 1 ? 'dashed' : 'solid'};
         border-width: 1px;
         border-color: rgba(var(--foreground-rgb), {(5 + i * 5) / 100});
        "
            ></div>
        {/each}
    </div>
</main>
