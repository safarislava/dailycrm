import React, { useState, useCallback, useMemo } from 'react'
import { useDispatch, useSelector } from 'react-redux'
import type { AppDispatch, RootState } from '../../store'
import type { Stage } from '../../types'
import { selectProject, selectStage } from '../../store/uiSlice'
import {
  useGetProjectsQuery,
  useGetStagesQuery,
  useDeleteStageMutation,
  useReorderStageMutation,
  useReorderSubStageMutation,
  useDeleteProjectMutation,
  useAppendSubStageMutation,
  useDeleteSubStageMutation,
  useRenameProjectMutation,
} from '../../store/crmApi'
import ConfirmDeleteModal from '../ConfirmDeleteModal/ConfirmDeleteModal'
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
  const project = projects.find((p) => p.id === projectId)

  const { data: stages = [], isLoading: stagesLoading } = useGetStagesQuery(
    projectId!, { skip: !projectId },
  )

  const [activeTab, setActiveTab] = useState<'stages' | 'dashboard'>('stages')

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
  const [deleteProject]   = useDeleteProjectMutation()
  const [appendSubStage]  = useAppendSubStageMutation()
  const [deleteSubStage]  = useDeleteSubStageMutation()
  const [renameProject]   = useRenameProjectMutation()

  type PendingDelete =
    | { kind: 'project' }
    | { kind: 'stage'; pos: number; stageTitle: string }
    | { kind: 'sub'; parentPos: number; pos: number; stageTitle: string }
  const [pendingDelete, setPendingDelete] = useState<PendingDelete | null>(null)

  const topLevelStagesCount = useMemo(() => {
    return stages.filter(s => s.parent_position === 0).length
  }, [stages])

  const confirmDelete = async () => {
    if (!pendingDelete || !projectId) return
    if (pendingDelete.kind === 'project') {
      await deleteProject(projectId)
      dispatch(selectProject(null))
    } else if (pendingDelete.kind === 'stage') {
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

  const pendingDeleteName =
    pendingDelete?.kind === 'project'
      ? (project?.title ?? '')
      : pendingDelete?.kind === 'stage' || pendingDelete?.kind === 'sub'
      ? pendingDelete.stageTitle
      : ''

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

  // ── Stage detail ───────────────────────────────────────────
  if (selectedStage !== null) {
    return (
      <div className={styles.panel}>
        {pendingDelete && (
          <ConfirmDeleteModal
            heading={pendingDelete.kind === 'project' ? 'Удалить проект' : 'Удалить этап'}
            name={pendingDeleteName}
            onConfirm={confirmDelete}
            onCancel={() => setPendingDelete(null)}
          />
        )}
        <StageDetailsSidebar
          projectId={projectId}
          selectedStage={selectedStage}
          isSub={isSub}
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
          heading={pendingDelete.kind === 'project' ? 'Удалить проект' : 'Удалить этап'}
          name={pendingDeleteName}
          onConfirm={confirmDelete}
          onCancel={() => setPendingDelete(null)}
        />
      )}

      <ProjectHeader
        projectId={projectId}
        projectTitle={project?.title ?? ''}
        topLevelStagesCount={topLevelStagesCount}
        activeTab={activeTab}
        setActiveTab={setActiveTab}
        renameProject={renameProject}
        setPendingDelete={setPendingDelete}
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
            setPendingDelete={setPendingDelete}
            reorderStage={reorderStage}
            reorderSubStage={reorderSubStage}
            appendSubStage={appendSubStage}
            dispatch={dispatch}
          />
          <AddStageForm projectId={projectId} />
        </>
      )}
    </div>
  )
}