import React, { useState, useCallback, useMemo } from 'react'
import { useDispatch, useSelector } from 'react-redux'
import type { AppDispatch, RootState } from '../../store'
import type { Stage } from '../../types'
import { selectProject, selectStage } from '../../store/uiSlice'
import {
  useGetProjectsQuery,
    useGetArchivedProjectsQuery,
  useGetStagesQuery,
  useDeleteStageMutation,
  useReorderStageMutation,
  useReorderSubStageMutation,
    useArchiveProjectMutation,
    useRestoreProjectMutation,
  useAppendSubStageMutation,
  useDeleteSubStageMutation,
  useRenameProjectMutation,
} from '../../store/crmApi'
import ConfirmDeleteModal from '../ConfirmDeleteModal/ConfirmDeleteModal'
import ConfirmArchiveModal from '../ConfirmArchiveModal/ConfirmArchiveModal'
import styles from './MainPanel.module.scss'

// Import extracted subcomponents
import { FolderIcon } from './components/Helpers'
import ProjectHeader from './components/ProjectHeader'
import StageTree from './components/StageTree'
import AddStageForm from './components/AddStageForm'
import Dashboard from './components/Dashboard'
import StageDetailsSidebar from './components/StageDetailsSidebar'

export default function MainPanel() {
  const dispatch = useDispatch<AppDispatch>()
  const projectId     = useSelector((s: RootState) => s.ui.selectedProjectId)
  const selectedStage = useSelector((s: RootState) => s.ui.selectedStage)
  const isSub         = selectedStage ? selectedStage.parentPosition !== 0 : false

  const { data: projects = [] } = useGetProjectsQuery()
    const {data: archivedProjects = []} = useGetArchivedProjectsQuery()
    const isArchived = archivedProjects.some((p) => p.id === projectId)
    const project = isArchived
        ? archivedProjects.find((p) => p.id === projectId)
        : projects.find((p) => p.id === projectId)

  const { data: stages = [], isLoading: stagesLoading } = useGetStagesQuery(
      {projectId: projectId!, isArchived},
      {skip: !projectId},
  )

  const [activeTab, setActiveTab] = useState<'stages' | 'dashboard'>('stages')
    const [showArchiveModal, setShowArchiveModal] = useState(false)

  const getStageLabel = useCallback((stage: Stage) => {
    if (stage.parent_position === 0) {
      return `${stage.position}`
    }
    return `${stage.parent_position}.${stage.position}`
  }, [])

  const sortedStagesForDashboard = useMemo(() => {
    const list: Stage[] = []
    const topLevels = stages.filter(s => s.parent_position === 0)
    for (const top of topLevels) {
      list.push(top)
      const children = stages.filter(s => s.parent_position === top.position)
      list.push(...children)
    }
    return list
  }, [stages])

  // ── Stage list mutations ───────────────────────────────────
  const [deleteStage]     = useDeleteStageMutation()
  const [reorderStage]    = useReorderStageMutation()
  const [reorderSubStage] = useReorderSubStageMutation()
    const [archiveProject] = useArchiveProjectMutation()
    const [restoreProject] = useRestoreProjectMutation()
  const [appendSubStage]  = useAppendSubStageMutation()
  const [deleteSubStage]  = useDeleteSubStageMutation()
  const [renameProject]   = useRenameProjectMutation()

  type PendingDelete =
    | { kind: 'stage'; pos: number; stageTitle: string }
    | { kind: 'sub'; parentPos: number; pos: number; stageTitle: string }
  const [pendingDelete, setPendingDelete] = useState<PendingDelete | null>(null)

  const topLevelStagesCount = useMemo(() => {
    return stages.filter(s => s.parent_position === 0).length
  }, [stages])

  const confirmDelete = async () => {
    if (!pendingDelete || !projectId) return
      if (pendingDelete.kind === 'stage') {
      await deleteStage({ projectId, position: pendingDelete.pos })
      if (selectedStage?.parentPosition === 0 && selectedStage.position === pendingDelete.pos)
        dispatch(selectStage(null))
    } else {
      await deleteSubStage({ projectId, parentPosition: pendingDelete.parentPos, position: pendingDelete.pos })
      if (selectedStage?.parentPosition === pendingDelete.parentPos && selectedStage.position === pendingDelete.pos)
        dispatch(selectStage(null))
    }
    setPendingDelete(null)
  }

    const confirmArchive = async () => {
        if (!projectId) return
        await archiveProject(projectId)
        dispatch(selectProject(null))
        setShowArchiveModal(false)
    }

    const handleRestore = async () => {
        if (!projectId) return
        await restoreProject(projectId)
    }

    const pendingDeleteName = pendingDelete?.stageTitle ?? ''

  // ── Empty state ────────────────────────────────────────────
  if (!projectId) {
    return (
      <div className={styles.empty}>
        <div className={styles.emptyIcon}><FolderIcon /></div>
        <p className={styles.emptyTitle}>Выберите проект</p>
        <p className={styles.emptyHint}>Выберите проект из списка, чтобы просмотреть его этапы</p>
      </div>
    )
  }

    const currentStage = stages.find(
        s => s.parent_position === selectedStage?.parentPosition && s.position === selectedStage?.position
    )

  // ── Stage detail ───────────────────────────────────────────
  if (selectedStage !== null) {
    return (
      <div className={styles.panel}>
        {pendingDelete && (
          <ConfirmDeleteModal
              heading="Удалить этап"
            name={pendingDeleteName}
            onConfirm={confirmDelete}
            onCancel={() => setPendingDelete(null)}
          />
        )}
        <StageDetailsSidebar
          projectId={projectId}
          selectedStage={selectedStage}
          isSub={isSub}
          isArchived={isArchived}
          archivedStage={currentStage}
          setPendingDelete={setPendingDelete}
        />
      </div>
    )
  }

  // ── Stages list with accordion ─────────────────────────────
  return (
    <div className={styles.panel}>
      {pendingDelete && (
        <ConfirmDeleteModal
            heading="Удалить этап"
          name={pendingDeleteName}
          onConfirm={confirmDelete}
          onCancel={() => setPendingDelete(null)}
        />
      )}

        {showArchiveModal && (
            <ConfirmArchiveModal
                heading="Архивировать проект"
                name={project?.title ?? ''}
                onConfirm={confirmArchive}
                onCancel={() => setShowArchiveModal(false)}
            />
        )}

      <ProjectHeader
        projectId={projectId}
        projectTitle={project?.title ?? ''}
        topLevelStagesCount={topLevelStagesCount}
        activeTab={activeTab}
        setActiveTab={setActiveTab}
        renameProject={renameProject}
        isArchived={isArchived}
        onArchive={() => setShowArchiveModal(true)}
        onRestore={handleRestore}
        dispatch={dispatch}
      />

      {activeTab === 'dashboard' ? (
        <Dashboard
          projectId={projectId}
          stagesLoading={stagesLoading}
          stages={stages}
          sortedStagesForDashboard={sortedStagesForDashboard}
          getStageLabel={getStageLabel}
          dispatch={dispatch}
        />
      ) : (
        <>
          <StageTree
            projectId={projectId}
            stagesLoading={stagesLoading}
            stages={stages}
            isArchived={isArchived}
            setPendingDelete={setPendingDelete}
            reorderStage={reorderStage}
            reorderSubStage={reorderSubStage}
            appendSubStage={appendSubStage}
            dispatch={dispatch}
          />
            {!isArchived && <AddStageForm projectId={projectId}/>}
        </>
      )}
    </div>
  )
}