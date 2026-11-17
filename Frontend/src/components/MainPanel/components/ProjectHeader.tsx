import React from 'react'
import {selectProject} from '../../../store/uiSlice'
import {ArchiveIcon, ArrowLeftIcon, InlineEdit, RestoreIcon,} from './Helpers'
import styles from '../MainPanel.module.scss'

interface ProjectHeaderProps {
  projectId: string
  projectTitle: string
  topLevelStagesCount: number
  activeTab: 'stages' | 'dashboard'
  setActiveTab: (tab: 'stages' | 'dashboard') => void
  renameProject: (args: { id: string; title: string }) => Promise<any>
    isArchived?: boolean
    onArchive: () => void
    onRestore: () => void
  dispatch: any
}

export default function ProjectHeader({
  projectId,
  projectTitle,
  topLevelStagesCount,
  activeTab,
  setActiveTab,
  renameProject,
                                          isArchived = false,
                                          onArchive,
                                          onRestore,
  dispatch,
}: ProjectHeaderProps) {
  return (
    <header className={styles.header}>
      <button className={`${styles.backBtn} ${styles.mobileOnly}`} onClick={() => dispatch(selectProject(null))}>
        <ArrowLeftIcon />
      </button>
      <div className={styles.headerInfo}>
          {isArchived ? (
              <span className={styles.headerTitle}>{projectTitle}</span>
          ) : (
              <InlineEdit
                  value={projectTitle}
                  onSave={async (v) => {
                      if (v.trim()) await renameProject({id: projectId, title: v.trim()})
                  }}
                  className={styles.headerTitle}
              />
          )}
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
        {isArchived ? (
            <button className={styles.actionBtn} onClick={onRestore} title="Восстановить проект">
                <RestoreIcon/>
            </button>
        ) : (
            <button className={styles.actionBtn} onClick={onArchive} title="В архив">
                <ArchiveIcon/>
            </button>
        )}
    </header>
  )
}

