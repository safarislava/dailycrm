import { useEffect } from 'react'
import { useDispatch, useSelector } from 'react-redux'
import type { AppDispatch, RootState } from './store'
import Sidebar from './components/Sidebar/Sidebar'
import MainPanel from './components/MainPanel/MainPanel'
import LoginPage from './components/LoginPage/LoginPage'
import { useRefreshMutation } from './store/crmApi'
import styles from './App.module.scss'

export default function App() {
  const dispatch = useDispatch<AppDispatch>()
  const { accessToken, initialized } = useSelector((s: RootState) => s.auth)
  const selectedProjectId = useSelector((s: RootState) => s.ui.selectedProjectId)
  const [refresh] = useRefreshMutation()

  useEffect(() => {
    refresh()
  }, [])

  if (!initialized) return null

  if (!accessToken) return <LoginPage />

  return (
    <div className={`${styles.app} ${selectedProjectId ? styles.projectOpen : ''}`}>
      <div className={styles.sidebarPane}>
        <Sidebar />
      </div>
      <div className={styles.mainPane}>
        <MainPanel />
      </div>
    </div>
  )
}