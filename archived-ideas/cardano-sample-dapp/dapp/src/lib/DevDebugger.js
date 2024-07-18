export function console_log(v1, v2, v3, v4, v5) {
    if (process.env.NODE_ENV !== "production") {
        console.log(v1, v2, v3, v4, v5)
    } else {
        null
    }
}

export function console_warn(v1, v2, v3, v4, v5) {
    if (process.env.NODE_ENV !== "production") {
        console.warn(v1, v2, v3, v4, v5)
    } else {
        null
    }
}

