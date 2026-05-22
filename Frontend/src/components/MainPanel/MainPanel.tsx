import { useState, useEffect, useRef } from 'react'
import { useDispatch, useSelector } from 'react-redux'
import type { AppDispatch, RootState } from '../../store'
import { selectProject, selectStage } from '../../store/uiSlice'
import {
  useGetProjectsQuery,
  useGetStagesQuery,
  useAppendStageMutation,
  useInsertStageMutation,
  useDeleteStageMutation,
  useDeleteProjectMutation,
  useGetDetailedStageQuery,
} from '../../store/crmApi'
import styles from './MainPanel.module.scss'

export default function MainPanel() {
  const dispatch = useDispatch<AppDispatch>()
  const projectId    = useSelector((s: RootState) => s.ui.selectedProjectId)
  const stagePos     = useSelector((s: RootState) => s.ui.selectedStageId)

  const { data: projects = [] } = useGetProjectsQuery()
  const project = projects.find((p) => p.id === projectId)

  const { data: stages = [], isLoading: stagesLoading } = useGetStagesQuery(
    projectId!, { skip: !projectId },
  )
  const { data: detail, isLoading: detailLoading } = useGetDetailedStageQuery(
    { projectId: projectId!, position: Number(stagePos) },
    { skip: !projectId || stagePos === null },
  )

  const [appendStage, { isLoading: appending }] = useAppendStageMutation()
  const [insertStage, { isLoading: inserting }] = useInsertStageMutation()
  const [deleteStage]   = useDeleteStageMutation()
  const [deleteProject] = useDeleteProjectMutation()

  const [title, setTitle]       = useState('')
  const [position, setPosition] = useState('')
  const bottomRef = useRef<HTMLDivElement>(null)
  const creating = appending || inserting

  const canSend = title.trim() !== '' && !creating

  const handleSend = async () => {
    if (!canSend || !projectId) return
    const t = title.trim()
    const p = position.trim()
    if (p === '') {
      await appendStage({ projectId, title: t })
    } else {
      await insertStage({ projectId, position: Number(p), title: t })
    }
    setTitle('')
    setPosition('')
  }

  const handleDeleteStage = async (pos: number) => {
    if (!projectId) return
    await deleteStage({ projectId, position: pos })
    if (stagePos !== null && Number(stagePos) === pos) dispatch(selectStage(null))
  }

  const handleDeleteProject = async () => {
    if (!projectId) return
    await deleteProject(projectId)
    dispatch(selectProject(null))
  }

  // ── Empty state ────────────────────────────────────────
  if (!projectId) {
    return (
      <div className={styles.empty}>
        <div className={styles.emptyIcon}><FolderIcon /></div>
        <p className={styles.emptyTitle}>Select a project</p>
        <p className={styles.emptyHint}>Choose a project from the list to view its stages</p>
      </div>
    )
  }

  // ── Stage detail ───────────────────────────────────────
  if (stagePos !== null) {
    return (
      <div className={styles.panel}>
        <header className={styles.header}>
          <button className={styles.backBtn} onClick={() => dispatch(selectStage(null))}>
            <ArrowLeftIcon />
          </button>
          <div className={styles.headerInfo}>
            <span className={styles.headerTitle}>Stage details</span>
          </div>
          <button
            className={styles.dangerBtn}
            onClick={() => handleDeleteStage(Number(stagePos))}
            title="Delete stage"
          >
            <TrashIcon />
          </button>
        </header>

        <div className={styles.detailScroll}>
          {detailLoading && <div className={styles.loading}>Loading…</div>}
          {!detailLoading && detail && (
            <div className={styles.detailCard}>
              <h2 className={styles.detailName}>{detail.stage.title}</h2>
              <div className={styles.fields}>
                <Field label="Position" value={String(detail.stage.position)} />
                <Field
                  label="Deadline"
                  value={detail.stage.deadline
                    ? new Date(detail.stage.deadline).toLocaleDateString('en-GB', {
                        day: '2-digit', month: 'short', year: 'numeric',
                      })
                    : '—'}
                />
                <Field label="Description" value={detail.description ?? '—'} />
                <Field
                  label="Cost"
                  value={detail.cost != null ? `$${detail.cost.toLocaleString()}` : '—'}
                />
              </div>
            </div>
          )}
          {!detailLoading && !detail && (
            <div className={styles.loading}>No data available</div>
          )}
        </div>
      </div>
    )
  }

  // ── Stages list ────────────────────────────────────────
  return (
    <div className={styles.panel}>
      <header className={styles.header}>
        <button
          className={`${styles.backBtn} ${styles.mobileOnly}`}
          onClick={() => dispatch(selectProject(null))}
        >
          <ArrowLeftIcon />
        </button>
        <div className={styles.headerInfo}>
          <span className={styles.headerTitle}>{project?.title}</span>
          <span className={styles.headerSub}>
            {stages.length} stage{stages.length !== 1 ? 's' : ''}
          </span>
        </div>
        <button className={styles.dangerBtn} onClick={handleDeleteProject} title="Delete project">
          <TrashIcon />
        </button>
      </header>

      <div className={styles.stageList}>
        {stagesLoading && <div className={styles.loading}>Loading…</div>}
        {!stagesLoading && stages.length === 0 && (
          <div className={styles.noStages}>
            <ListIcon />
            <p>No stages yet</p>
            <span>Type below to add the first one</span>
          </div>
        )}
        {stages.map((stage) => (
          <div
            key={stage.position}
            className={styles.stageItem}
            onClick={() => dispatch(selectStage(String(stage.position)))}
          >
            <span className={styles.stagePos}>{stage.position}</span>
            <div className={styles.stageInfo}>
              <span className={styles.stageTitle}>{stage.title}</span>
              {stage.deadline && (
                <span className={styles.stageDeadline}>
                  {new Date(stage.deadline).toLocaleDateString('en-GB', {
                    day: '2-digit', month: 'short', year: 'numeric',
                  })}
                </span>
              )}
            </div>
            <button
              className={styles.stageDelete}
              onClick={(e) => { e.stopPropagation(); handleDeleteStage(stage.position) }}
              title="Delete stage"
            >
              <CloseIcon />
            </button>
          </div>
        ))}
        <div ref={bottomRef} />
      </div>

      <div className={styles.inputRow}>
        <input
          className={styles.posInput}
          type="number"
          placeholder="# opt"
          min={1}
          value={position}
          onChange={(e) => setPosition(e.target.value)}
          onKeyDown={(e) => { if (e.key === 'Enter') handleSend() }}
        />
        <input
          className={styles.textInput}
          placeholder="New stage…"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
          onKeyDown={(e) => { if (e.key === 'Enter') handleSend() }}
        />
        <button
          className={styles.sendBtn}
          onClick={handleSend}
          disabled={!canSend}
        >
          <SendIcon />
        </button>
      </div>
    </div>
  )
}

// ── Small sub-component ────────────────────────────────
function Field({ label, value }: { label: string; value: string }) {
  return (
    <div className={styles.field}>
      <span className={styles.fieldLabel}>{label}</span>
      <span className={styles.fieldValue}>{value}</span>
    </div>
  )
}

// ── Icons ──────────────────────────────────────────────
function ArrowLeftIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
      <path d="M15 18l-6-6 6-6" stroke="currentColor" strokeWidth="2.2"
        strokeLinecap="round" strokeLinejoin="round"/>
    </svg>
  )
}
function TrashIcon() {
  return (
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
      <polyline points="3 6 5 6 21 6" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
      <path d="M19 6l-1 14H6L5 6" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
      <path d="M10 11v6M14 11v6" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
      <path d="M9 6V4h6v2" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
    </svg>
  )
}
function SendIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
      <path d="M22 2 11 13" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
      <path d="M22 2 15 22 11 13 2 9l20-7Z" stroke="currentColor" strokeWidth="2"
        strokeLinecap="round" strokeLinejoin="round"/>
    </svg>
  )
}
function CloseIcon() {
  return (
    <svg width="10" height="10" viewBox="0 0 24 24" fill="none">
      <path d="M18 6 6 18M6 6l12 12" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round"/>
    </svg>
  )
}
function FolderIcon() {
  return (
    <svg width="64" height="64" viewBox="0 0 24 24" fill="none">
      <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2Z"
        stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"/>
    </svg>
  )
}
function ListIcon() {
  return (
    <svg width="40" height="40" viewBox="0 0 24 24" fill="none">
      <line x1="8" y1="6" x2="21" y2="6" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round"/>
      <line x1="8" y1="12" x2="21" y2="12" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round"/>
      <line x1="8" y1="18" x2="21" y2="18" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round"/>
      <circle cx="3" cy="6" r="1.2" fill="currentColor"/>
      <circle cx="3" cy="12" r="1.2" fill="currentColor"/>
      <circle cx="3" cy="18" r="1.2" fill="currentColor"/>
    </svg>
  )
}