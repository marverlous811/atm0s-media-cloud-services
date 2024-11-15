import useSWR, { Fetcher } from 'swr'

const useGetUser = () => {
  const fetcher: Fetcher<unknown, string> = (url) =>
    fetch(url, {
      credentials: 'same-origin',
    }).then((res) => res.json())
  const { isLoading, data } = useSWR(`/api/users/me`, fetcher)
  return { isLoading, data }
}

export default function Secure() {
  const { isLoading, data } = useGetUser()

  return isLoading ? <div>loading...</div> : <div>{JSON.stringify(data)}</div>
}
