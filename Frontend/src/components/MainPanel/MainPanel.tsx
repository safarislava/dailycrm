import { useState, useRef } from 'react'
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
  useUpdateStageTitleMutation,
  useUpdateStageDeadlineMutation,
  useUpdateStageDescriptionMutation,
  useUpdateStageCostMutation,
  useRenameProjectMutation,
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

  const [updateTitle]       = useUpdateStageTitleMutation()
  const [updateDeadline]    = useUpdateStageDeadlineMutation()
  const [updateDescription] = useUpdateStageDescriptionMutation()
  const [updateCost]        = useUpdateStageCostMutation()
  const [renameProject]     = useRenameProjectMutation()

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
        <p className={styles.emptyTitle}>Выберите проект</p>
        <p className={styles.emptyHint}>Выберите проект из списка, чтобы просмотреть его этапы</p>
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
            <span className={styles.headerTitle}>Детали этапа</span>
          </div>
          <button
            className={styles.dangerBtn}
            onClick={() => handleDeleteStage(Number(stagePos))}
            title="Удалить этап"
          >
            <TrashIcon />
          </button>
        </header>

        <div className={styles.detailScroll}>
          {detailLoading && <div className={styles.loading}>Загрузка…</div>}
          {!detailLoading && detail && (
            <div className={styles.detailCard}>
              <div className={styles.fields}>
                <Field label="Позиция" value={String(detail.stage.position)} />
                <EditableField
                  label="Название"
                  displayValue={detail.stage.title}
                  rawValue={detail.stage.title}
                  onSave={async (v) => {
                    if (v.trim()) await updateTitle({ projectId: projectId!, position: Number(stagePos), title: v.trim() })
                  }}
                />
                <EditableField
                  label="Дедлайн"
                  displayValue={detail.stage.deadline
                    ? new Date(detail.stage.deadline).toLocaleDateString('en-GB', {
                        day: '2-digit', month: 'short', year: 'numeric',
                      })
                    : '—'}
                  rawValue={detail.stage.deadline?.slice(0, 10) ?? ''}
                  type="date"
                  onSave={async (v) => {
                    await updateDeadline({
                      projectId: projectId!,
                      position: Number(stagePos),
                      deadline: v ? `${v}T00:00:00` : null,
                    })
                  }}
                />
                <EditableField
                  label="Описание"
                  displayValue={detail.description ?? '—'}
                  rawValue={detail.description ?? ''}
                  multiline
                  onSave={async (v) => {
                    await updateDescription({
                      projectId: projectId!,
                      position: Number(stagePos),
                      description: v.trim() || null,
                    })
                  }}
                />
                <EditableField
                  label="Стоимость"
                  displayValue={detail.cost != null ? `${detail.cost.toLocaleString()} ₽` : '—'}
                  rawValue={detail.cost?.toString() ?? ''}
                  type="number"
                  onSave={async (v) => {
                    await updateCost({
                      projectId: projectId!,
                      position: Number(stagePos),
                      cost: v ? parseInt(v, 10) : null,
                    })
                  }}
                />
              </div>
            </div>
          )}
          {!detailLoading && !detail && (
            <div className={styles.loading}>Нет данных</div>
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
          <InlineEdit
            value={project?.title ?? ''}
            onSave={async (v) => {
              if (v.trim() && projectId) await renameProject({ id: projectId, title: v.trim() })
            }}
            className={styles.headerTitle}
          />
          <span className={styles.headerSub}>
            {stages.length} {stages.length === 1 ? 'этап' : stages.length < 5 ? 'этапа' : 'этапов'}
          </span>
        </div>
        <button className={styles.dangerBtn} onClick={handleDeleteProject} title="Удалить проект">
          <TrashIcon />
        </button>
      </header>

      <div className={styles.stageList}>
        {stagesLoading && <div className={styles.loading}>Загрузка…</div>}
        {!stagesLoading && stages.length === 0 && (
          <div className={styles.noStages}>
            <ListIcon />
            <p>Нет этапов</p>
            <span>Введите название ниже, чтобы добавить первый</span>
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
              title="Удалить этап"
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

// ── InlineEdit — однострочный редактор для хедера ──────
function InlineEdit({
  value,
  onSave,
  className,
}: {
  value: string
  onSave: (value: string) => Promise<void>
  className?: string
}) {
  const [editing, setEditing] = useState(false)
  const [draft, setDraft] = useState('')
  const cancelled = useRef(false)

  const startEdit = () => {
    setDraft(value)
    setEditing(true)
  }

  const handleBlur = async () => {
    if (cancelled.current) {
      cancelled.current = false
      return
    }
    setEditing(false)
    await onSave(draft)
  }

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') (e.target as HTMLElement).blur()
    if (e.key === 'Escape') {
      cancelled.current = true
      ;(e.target as HTMLElement).blur()
    }
  }

  return editing ? (
    <input
      autoFocus
      className={`${className ?? ''} ${styles.inlineInput}`}
      value={draft}
      onChange={(e) => setDraft(e.target.value)}
      onBlur={handleBlur}
      onKeyDown={handleKeyDown}
    />
  ) : (
    <span className={`${className ?? ''} ${styles.inlineValue}`} onClick={startEdit} title="Переименовать">
      {value}
      <PencilIcon />
    </span>
  )
}

// ── Sub-components ─────────────────────────────────────
function Field({ label, value }: { label: string; value: string }) {
  return (
    <div className={styles.field}>
      <span className={styles.fieldLabel}>{label}</span>
      <span className={styles.fieldValue}>{value}</span>
    </div>
  )
}

function EditableField({
  label,
  displayValue,
  rawValue,
  onSave,
  type = 'text',
  multiline = false,
}: {
  label: string
  displayValue: string
  rawValue: string
  onSave: (value: string) => Promise<void>
  type?: 'text' | 'number' | 'date'
  multiline?: boolean
}) {
  const [editing, setEditing] = useState(false)
  const [draft, setDraft] = useState('')
  const cancelled = useRef(false)

  const startEdit = () => {
    setDraft(rawValue)
    setEditing(true)
  }

  const handleBlur = async () => {
    if (cancelled.current) {
      cancelled.current = false
      return
    }
    setEditing(false)
    await onSave(draft)
  }

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    if (e.key === 'Enter' && !multiline) {
      ;(e.target as HTMLElement).blur()
    }
    if (e.key === 'Escape') {
      cancelled.current = true
      ;(e.target as HTMLElement).blur()
    }
  }

  return (
    <div
      className={`${styles.field} ${styles.fieldEditable}`}
      onClick={!editing ? startEdit : undefined}
    >
      <span className={styles.fieldLabel}>{label}</span>
      {editing ? (
        multiline ? (
          <textarea
            autoFocus
            className={styles.fieldInput}
            value={draft}
            onChange={(e) => setDraft(e.target.value)}
            onBlur={handleBlur}
            onKeyDown={handleKeyDown}
            rows={3}
          />
        ) : (
          <input
            autoFocus
            type={type}
            className={styles.fieldInput}
            value={draft}
            onChange={(e) => setDraft(e.target.value)}
            onBlur={handleBlur}
            onKeyDown={handleKeyDown}
          />
        )
      ) : (
        <span className={styles.fieldValue}>{displayValue}</span>
      )}
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
function PencilIcon() {
  return (
    <svg className={styles.pencilIcon} width="12" height="12" viewBox="0 0 24 24" fill="none">
      <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"
        stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
      <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L13 14l-4 1 1-4 8.5-8.5Z"
        stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
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