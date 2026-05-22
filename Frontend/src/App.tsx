import { useSelector } from 'react-redux'
import type { RootState } from './store'
import Sidebar from './components/Sidebar/Sidebar'
import MainPanel from './components/MainPanel/MainPanel'
import styles from './App.module.scss'

export default function App() {
  const selectedProjectId = useSelector((s: RootState) => s.ui.selectedProjectId)

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