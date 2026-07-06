import React from 'react'
import { selectProject } from '../../../store/uiSlice'
import {
  ArrowLeftIcon,
  TrashIcon,
  InlineEdit,
} from './Helpers'
import styles from '../MainPanel.module.scss'

interface ProjectHeaderProps {
  projectId: string
  projectTitle: string
  topLevelStagesCount: number
  activeTab: 'stages' | 'dashboard'
  setActiveTab: (tab: 'stages' | 'dashboard') => void
  renameProject: (args: { id: string; title: string }) => Promise<any>
  setPendingDelete: (deleteObj: any) => void
  dispatch: any
}

export default function ProjectHeader({
  projectId,
  projectTitle,
  topLevelStagesCount,
  activeTab,
  setActiveTab,
  renameProject,
  setPendingDelete,
  dispatch,
}: ProjectHeaderProps) {
  return (
    <header className={styles.header}>
      <button className={`${styles.backBtn} ${styles.mobileOnly}`} onClick={() => dispatch(selectProject(null))}>
        <ArrowLeftIcon />
      </button>
      <div className={styles.headerInfo}>
        <InlineEdit
          value={projectTitle}
          onSave={async (v) => { if (v.trim()) await renameProject({ id: projectId, title: v.trim() }) }}
          className={styles.headerTitle}
        />
        <span className={styles.headerSub}>
          {topLevelStagesCount} {topLevelStagesCount === 1 ? 'этап' : topLevelStagesCount < 5 ? 'этапа' : 'этапов'}
        </span>
      </div>
      <div className={styles.tabs}>
        <button
          className={`${styles.tab} ${activeTab === 'stages' ? styles.tabActive : ''}`}
          onClick={() => setActiveTab('stages')}
        >
          Этапы
        </button>
        <button
          className={`${styles.tab} ${activeTab === 'dashboard' ? styles.tabActive : ''}`}
          onClick={() => setActiveTab('dashboard')}
        >
          Дашборд
        </button>
      </div>
      <button className={styles.dangerBtn} onClick={() => setPendingDelete({ kind: 'project' })} title="Удалить проект">
        <TrashIcon />
      </button>
    </header>
  )
}
