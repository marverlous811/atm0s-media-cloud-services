import { env } from '@/config'
import axios from 'axios'
import { useEffect, useState } from 'react'

interface ConfigsView {
    clerk_publishable_key: string
}

// TODO handle error
export const useGetConfigsViewQuery = () => {
    console.log('useGetConfigsViewQuery')
    const [configsView, setConfigsView] = useState<ConfigsView | null>(null)
    const [isPending, setIsPending] = useState(true)
    const [error, setError] = useState<Error | null>(null)

    useEffect(() => {
        console.log('useGetConfigsViewQuery')
        const api = axios.create({
            baseURL: env.API_URL,
        })
        api.get(`/api/configs/view`).then((res) => {
            setConfigsView(res.data)
            setIsPending(false)
        }).catch((err) => {
            console.error(err)
            setError(err)
        })
    }, [])

    return {
        data: configsView,
        isPending,
        error,
    }
}
