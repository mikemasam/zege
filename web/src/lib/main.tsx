import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './style.css'
import 'material-icons/iconfont/material-icons.css';
import App from '../App.tsx'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
