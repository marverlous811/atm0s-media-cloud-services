import { useGetConfigsViewQuery } from '@/hooks'
import { ClerkProvider } from '@clerk/clerk-react'

type Props = {
  children: React.ReactNode
}

export const AuthProvider: React.FC<Props> = ({ children }) => {
  const { data: configsView } = useGetConfigsViewQuery()
  if (!configsView?.clerk_publishable_key) {
    return <>Loading...</>
  } else {
    return <ClerkProvider publishableKey={configsView?.clerk_publishable_key}>{children}</ClerkProvider>
  }
}
