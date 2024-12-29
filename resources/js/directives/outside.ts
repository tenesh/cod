function outside(node: HTMLElement, listener: string, callback: (event: Event) => void) {
    const handleEvent = (event: Event) => {
        if (!node.contains(event.target as Node) && !event.defaultPrevented) {
            callback(event);
        }
    };

    document.addEventListener(listener, handleEvent, true);

    return {
        destroy() {
            document.removeEventListener(listener, handleEvent, true);
        },
    };
}

export function clickOutside(node: HTMLElement, callback: (event: Event) => void) {
    return outside(node, 'click', callback);
}

export function tapOutside(node: HTMLElement, callback: (event: Event) => void) {
    return outside(node, 'mousedown', callback);
}
