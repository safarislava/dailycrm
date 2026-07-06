import React, { useState, useRef, useCallback, useMemo } from 'react'
import { useDispatch, useSelector } from 'react-redux'
import type { AppDispatch, RootState } from '../../store'
import type { Stage } from '../../types'
import { selectProject, selectStage } from '../../store/uiSlice'
import {
  useGetProjectsQuery,
  useGetStagesQuery,
  useAppendStageMutation,
  useInsertStageMutation,
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

// Import extracted components
import {
  ArrowLeftIcon,
  TrashIcon,
  SendIcon,
  CloseIcon,
  FolderIcon,
  ListIcon,
  CircleIcon,
  CheckCircleIcon,
  ChevronRightIcon,
  PlusIcon,
  InlineEdit,
} from './components/Helpers'
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
  const [appendStage, { isLoading: appending }]   = useAppendStageMutation()
  const [insertStage, { isLoading: inserting }]   = useInsertStageMutation()
  const [deleteStage]                             = useDeleteStageMutation()
  const [reorderStage]                            = useReorderStageMutation()
  const [reorderSubStage]                         = useReorderSubStageMutation()
  const [deleteProject]                           = useDeleteProjectMutation()
  const [appendSubStage]                          = useAppendSubStageMutation()
  const [deleteSubStage]                          = useDeleteSubStageMutation()
  const [renameProject]                           = useRenameProjectMutation()

  // ── Stage list state ───────────────────────────────────────
  const [title, setTitle]       = useState('')
  const [position, setPosition] = useState('')
  const [expandedStages, setExpandedStages] = useState<Set<number>>(new Set())
  const [addingSubTo, setAddingSubTo]       = useState<number | null>(null)
  const [subTitle, setSubTitle]             = useState('')
  const bottomRef = useRef<HTMLDivElement>(null)
  const creating  = appending || inserting

  // ── Drag-and-drop reordering ───────────────────────────────
  const [dragPos, setDragPos]           = useState<number | null>(null)
  const [dragOverPos, setDragOverPos]   = useState<number | null>(null)
  const [dragSub, setDragSub]           = useState<{ parent: number; pos: number } | null>(null)
  const [dragOverSub, setDragOverSub]   = useState<{ parent: number; pos: number } | null>(null)

  const handleStageDrop = async (target: number) => {
    const from = dragPos
    setDragPos(null)
    setDragOverPos(null)
    if (from === null || from === target || !projectId) return
    await reorderStage({ projectId, position: from, to: target })
  }

  const handleSubDrop = async (parent: number, target: number) => {
    const dragged = dragSub
    setDragSub(null)
    setDragOverSub(null)
    if (!dragged || dragged.parent !== parent || dragged.pos === target || !projectId) return
    await reorderSubStage({ projectId, parentPosition: parent, position: dragged.pos, to: target })
  }

  type PendingDelete =
    | { kind: 'project' }
    | { kind: 'stage'; pos: number; stageTitle: string }
    | { kind: 'sub'; parentPos: number; pos: number; stageTitle: string }
  const [pendingDelete, setPendingDelete] = useState<PendingDelete | null>(null)

  const canSend    = title.trim() !== '' && !creating
  const canSendSub = subTitle.trim() !== ''

  const topLevelStages  = stages.filter(s => s.parent_position === 0)
  const childrenOf = (pos: number) => stages.filter(s => s.parent_position === pos)

  const toggleExpand = (pos: number) => {
    const collapsing = expandedStages.has(pos)
    setExpandedStages(prev => {
      const next = new Set(prev)
      if (collapsing) next.delete(pos)
      else next.add(pos)
      return next
    })
    if (collapsing && addingSubTo === pos) setAddingSubTo(null)
  }

  const startAddSub = (pos: number) => {
    setExpandedStages(prev => new Set([...prev, pos]))
    setAddingSubTo(pos)
    setSubTitle('')
  }

  const handleSend = async () => {
    if (!canSend || !projectId) return
    const t = title.trim()
    const p = position.trim()
    if (p === '') { await appendStage({ projectId, title: t }) }
    else          { await insertStage({ projectId, position: Number(p), title: t }) }
    setTitle('')
    setPosition('')
  }

  const handleSendSub = async (parentPos: number) => {
    if (!canSendSub || !projectId) return
    await appendSubStage({ projectId, parentPosition: parentPos, title: subTitle.trim() })
    setSubTitle('')
    setAddingSubTo(null)
  }

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
      <header className={styles.header}>
        <button className={`${styles.backBtn} ${styles.mobileOnly}`} onClick={() => dispatch(selectProject(null))}>
          <ArrowLeftIcon />
        </button>
        <div className={styles.headerInfo}>
          <InlineEdit
            value={project?.title ?? ''}
            onSave={async (v) => { if (v.trim() && projectId) await renameProject({ id: projectId, title: v.trim() }) }}
            className={styles.headerTitle}
          />
          <span className={styles.headerSub}>
            {topLevelStages.length} {topLevelStages.length === 1 ? 'этап' : topLevelStages.length < 5 ? 'этапа' : 'этапов'}
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
          <div className={styles.stageList}>
            {stagesLoading && <div className={styles.loading}>Загрузка…</div>}
            {!stagesLoading && topLevelStages.length === 0 && (
              <div className={styles.noStages}>
                <ListIcon />
                <p>Нет этапов</p>
                <span>Введите название ниже, чтобы добавить первый</span>
              </div>
            )}
            {topLevelStages.map((stage) => {
              const children  = childrenOf(stage.position)
              const expanded  = expandedStages.has(stage.position)
              const addingHere= addingSubTo === stage.position

              return (
                <React.Fragment key={stage.position}>
                  <div
                    className={`${styles.stageItem} ${stage.completed ? styles.stageCompleted : ''} ${dragPos === stage.position ? styles.stageDragging : ''} ${dragOverPos === stage.position ? styles.stageDragOver : ''}`}
                    onClick={() => dispatch(selectStage({ parentPosition: 0, position: stage.position }))}
                    draggable
                    onDragStart={(e) => { e.stopPropagation(); setDragPos(stage.position) }}
                    onDragOver={(e) => { if (dragPos !== null) { e.preventDefault(); setDragOverPos(stage.position) } }}
                    onDragLeave={() => setDragOverPos((p) => (p === stage.position ? null : p))}
                    onDrop={(e) => { e.preventDefault(); handleStageDrop(stage.position) }}
                    onDragEnd={() => { setDragPos(null); setDragOverPos(null) }}
                  >
                    {(children.length > 0 || expanded) ? (
                      <button
                        className={`${styles.stageChevron} ${styles.stageChevronVisible} ${expanded ? styles.stageChevronOpen : ''}`}
                        onClick={(e) => { e.stopPropagation(); toggleExpand(stage.position) }}
                        title={expanded ? 'Свернуть' : 'Развернуть'}
                      >
                        <ChevronRightIcon />
                      </button>
                    ) : (
                      <span className={styles.stageChevronSpacer} />
                    )}
                    <span className={styles.stageCheck} title={stage.completed ? 'Этап выполнен' : 'Этап не выполнен'}>
                      {stage.completed ? <CheckCircleIcon /> : <CircleIcon />}
                    </span>
                    <div className={styles.stageInfo}>
                      <span className={styles.stageTitle}>{stage.position}. {stage.title}</span>
                      {stage.deadline && (
                        <span className={styles.stageDeadline}>
                          {new Date(stage.deadline).toLocaleDateString('en-GB', { day: '2-digit', month: 'short', year: 'numeric' })}
                        </span>
                      )}
                    </div>
                    <button
                      className={styles.stageAddSub}
                      onClick={(e) => { e.stopPropagation(); startAddSub(stage.position) }}
                      title="Добавить подэтап"
                    >
                      <PlusIcon />
                    </button>
                    <button
                      className={styles.stageDelete}
                      onClick={(e) => { e.stopPropagation(); setPendingDelete({ kind: 'stage', pos: stage.position, stageTitle: stage.title }) }}
                      title="Удалить этап"
                    >
                      <CloseIcon />
                    </button>
                  </div>

                  {(expanded || addingHere) && (
                    <div className={styles.subStageGroup}>
                      {children.map((child) => (
                        <div
                          key={child.position}
                          className={`${styles.stageItem} ${styles.subStageItem} ${child.completed ? styles.stageCompleted : ''} ${dragSub?.parent === stage.position && dragSub?.pos === child.position ? styles.stageDragging : ''} ${dragOverSub?.parent === stage.position && dragOverSub?.pos === child.position ? styles.stageDragOver : ''}`}
                          onClick={() => dispatch(selectStage({ parentPosition: stage.position, position: child.position }))}
                          draggable
                          onDragStart={(e) => { e.stopPropagation(); setDragSub({ parent: stage.position, pos: child.position }) }}
                          onDragOver={(e) => { if (dragSub?.parent === stage.position) { e.preventDefault(); setDragOverSub({ parent: stage.position, pos: child.position }) } }}
                          onDragLeave={() => setDragOverSub((s) => (s?.parent === stage.position && s?.pos === child.position ? null : s))}
                          onDrop={(e) => { e.preventDefault(); handleSubDrop(stage.position, child.position) }}
                          onDragEnd={() => { setDragSub(null); setDragOverSub(null) }}
                        >
                          <span className={styles.subStageIndent} />
                          <span className={styles.stageCheck} title={child.completed ? 'Выполнен' : 'Не выполнен'}>
                            {child.completed ? <CheckCircleIcon /> : <CircleIcon />}
                          </span>
                          <div className={styles.stageInfo}>
                            <span className={styles.stageTitle}>{stage.position}.{child.position}. {child.title}</span>
                            {child.deadline && (
                              <span className={styles.stageDeadline}>
                                {new Date(child.deadline).toLocaleDateString('en-GB', { day: '2-digit', month: 'short', year: 'numeric' })}
                              </span>
                            )}
                          </div>
                          <button
                            className={styles.stageDelete}
                            onClick={(e) => { e.stopPropagation(); setPendingDelete({ kind: 'sub', parentPos: stage.position, pos: child.position, stageTitle: child.title }) }}
                            title="Удалить подэтап"
                          >
                            <CloseIcon />
                          </button>
                        </div>
                      ))}

                      {addingHere && (
                        <div className={styles.subStageInputRow}>
                          <span className={styles.subStageIndent} />
                          <input
                            autoFocus
                            className={styles.subStageInput}
                            placeholder="Новый подэтап…"
                            value={subTitle}
                            onChange={(e) => setSubTitle(e.target.value)}
                            onKeyDown={(e) => {
                              if (e.key === 'Enter') handleSendSub(stage.position)
                              if (e.key === 'Escape') { setAddingSubTo(null); setSubTitle('') }
                            }}
                            onBlur={() => { if (!subTitle.trim()) { setAddingSubTo(null) } }}
                          />
                          <button
                            className={styles.sendBtn}
                            onClick={() => handleSendSub(stage.position)}
                            disabled={!canSendSub}
                          >
                            <SendIcon />
                          </button>
                        </div>
                      )}
                    </div>
                  )}
                </React.Fragment>
              )
            })}
            <div ref={bottomRef} />
          </div>

          <div className={styles.inputRow}>
            <input
              className={styles.posInput}
              type="number"
              placeholder="№"
              min={1}
              value={position}
              onChange={(e) => setPosition(e.target.value)}
              onKeyDown={(e) => { if (e.key === 'Enter') handleSend() }}
            />
            <input
              className={styles.textInput}
              placeholder="Новый этап…"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              onKeyDown={(e) => { if (e.key === 'Enter') handleSend() }}
            />
            <button className={styles.sendBtn} onClick={handleSend} disabled={!canSend}>
              <SendIcon />
            </button>
          </div>
        </>
      )}
    </div>
  )
}