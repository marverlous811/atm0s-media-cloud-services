export const env = {
  API_URL: (import.meta.env.VITE_APP_API_URL as string) || '/',
  CLERK_PUBLISHABLE_KEY: (import.meta.env.VITE_CLERK_PUBLISHABLE_KEY as string) || '',
}
