import { BrowserRouter, Route, Routes } from 'react-router-dom'
import Login from './components/Login'
import Secure from './components/Secure'
import { GoogleOAuthProvider } from '@react-oauth/google'

declare global {
  interface Window {
    env: {
      GOOGLE_CLIENT_ID: string
    }
  }
}

function App() {
  return (
    <>
      <GoogleOAuthProvider clientId={window.env.GOOGLE_CLIENT_ID}>
        <BrowserRouter>
          <Routes>
            <Route path="/" element={<Login />} />
            <Route path="/secure" element={<Secure />} />
          </Routes>
        </BrowserRouter>
      </GoogleOAuthProvider>
    </>
  )
}

export default App
