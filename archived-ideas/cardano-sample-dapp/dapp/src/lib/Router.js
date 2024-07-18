export function getCurrentUrl() {
    return window.location.href
}

export function navigateTo(route) {
    const routeStr = route || ""
    const url = `/#/${routeStr}`
    window.location.href = url;
 }