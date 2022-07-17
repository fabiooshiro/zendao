import React from "react"
import { ZendaoService } from "../services/ZendaoService"

type AmountProps = {
    value: string
    decimals: number | null
}

export function Amount({ value, decimals }: AmountProps) {
    if (decimals === null || value === '') {
        return <span></span>
    }
    return <span>{ZendaoService.format(value, { decimals: decimals })}</span>
}
