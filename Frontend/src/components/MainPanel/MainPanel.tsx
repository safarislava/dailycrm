import React, { useState, useRef, useCallback, useEffect, useLayoutEffect, useMemo } from 'react'
import { useDispatch, useSelector } from 'react-redux'
import type { AppDispatch, RootState } from '../../store'
import type { Comment } from '../../types'
import { selectProject, selectStage } from '../../store/uiSlice'
import { store } from '../../store'
import {
  crmApi,
  useGetProjectsQuery,
  useGetStagesQuery,
  useAppendStageMutation,
  useInsertStageMutation,
  useDeleteStageMutation,
  useReorderStageMutation,
  useReorderSubStageMutation,
  useDeleteProjectMutation,
  useGetDetailedStageQuery,
  useUpdateStageTitleMutation,
  useUpdateStageDeadlineMutation,
  useUpdateStageCostMutation,
  useUpdateGipConfirmedMutation,
  useUpdatePaymentConfirmedMutation,
  useListActsQuery,
  useUploadActMutation,
  useDeleteActMutation,
  useRenameProjectMutation,
  useListAttachmentsQuery,
  useUploadAttachmentMutation,
  useDeleteAttachmentMutation,
  useListCommentsQuery,
  useAddCommentMutation,
  useDeleteCommentMutation,
  useAppendSubStageMutation,
  useDeleteSubStageMutation,
  useGetDetailedSubStageQuery,
  useUpdateSubStageTitleMutation,
  useUpdateSubStageDeadlineMutation,
  useUpdateSubStageCostMutation,
  useUpdateSubStageGipConfirmedMutation,
  useUpdateSubStagePaymentConfirmedMutation,
  useListSubStageActsQuery,
  useUploadSubStageActMutation,
  useDeleteSubStageActMutation,
  useListSubStageAttachmentsQuery,
  useUploadSubStageAttachmentMutation,
  useDeleteSubStageAttachmentMutation,
  useListSubStageCommentsQuery,
  useAddSubStageCommentMutation,
  useDeleteSubStageCommentMutation,
} from '../../store/crmApi'
import ConfirmDeleteModal from '../ConfirmDeleteModal/ConfirmDeleteModal'
import styles from './MainPanel.module.scss'

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

  // ── Detail queries (one fires, one skips) ─────────────────
  const { data: topDetail, isLoading: topDetailLoading } = useGetDetailedStageQuery(
    { projectId: projectId!, position: selectedStage?.position ?? 0 },
    { skip: !projectId || !selectedStage || isSub },
  )
  const { data: subDetail, isLoading: subDetailLoading } = useGetDetailedSubStageQuery(
    { projectId: projectId!, parentPosition: selectedStage?.parentPosition ?? 0, position: selectedStage?.position ?? 0 },
    { skip: !projectId || !selectedStage || !isSub },
  )
  const detail        = isSub ? subDetail : topDetail
  const detailLoading = isSub ? subDetailLoading : topDetailLoading

  // ── Attachments / acts / comments queries ──────────────────
  const actArgs        = { projectId: projectId!, position: selectedStage?.position ?? 0 }
  const subActArgs     = { projectId: projectId!, parentPosition: selectedStage?.parentPosition ?? 0, position: selectedStage?.position ?? 0 }
  const skipTop        = !projectId || !selectedStage || isSub
  const skipSub        = !projectId || !selectedStage || !isSub

  const { data: topActs = [] }         = useListActsQuery(actArgs, { skip: skipTop })
  const { data: subActs = [] }         = useListSubStageActsQuery(subActArgs, { skip: skipSub })
  const acts = isSub ? subActs : topActs

  const { data: topAttachments = [] }  = useListAttachmentsQuery(actArgs, { skip: skipTop })
  const { data: subAttachments = [] }  = useListSubStageAttachmentsQuery(subActArgs, { skip: skipSub })
  const attachments = isSub ? subAttachments : topAttachments

  const { data: topComments = [] }     = useListCommentsQuery(actArgs, { skip: skipTop })
  const { data: subComments = [] }     = useListSubStageCommentsQuery(subActArgs, { skip: skipSub })
  const comments = isSub ? subComments : topComments

  // ── Lazy-loaded older comments ─────────────────────────────
  const COMMENTS_PAGE = 25
  const stageKey = `${projectId}-${selectedStage?.parentPosition ?? 0}-${selectedStage?.position ?? 0}`
  const [olderComments, setOlderComments] = useState<Comment[]>([])
  const [hasMoreComments, setHasMoreComments] = useState(true)
  const [loadingOlderComments, setLoadingOlderComments] = useState(false)
  const commentsScrollRef = useRef<HTMLDivElement>(null)
  const restoreScrollRef = useRef<number | null>(null)
  const initialScrolledKeyRef = useRef<string | null>(null)

  useEffect(() => {
    setOlderComments([])
    setHasMoreComments(true)
    setLoadingOlderComments(false)
    restoreScrollRef.current = null
    initialScrolledKeyRef.current = null
  }, [stageKey])

  const allComments = useMemo(() => {
    const byId = new Map<string, Comment>()
    for (const c of olderComments) byId.set(c.id, c)
    for (const c of comments) byId.set(c.id, c)
    return Array.from(byId.values()).sort((a, b) =>
      a.created_at === b.created_at ? a.id.localeCompare(b.id) : a.created_at.localeCompare(b.created_at),
    )
  }, [olderComments, comments])

  const handleLoadOlderComments = useCallback(async () => {
    if (!projectId || !selectedStage || loadingOlderComments || !hasMoreComments) return
    const oldest = allComments[0]
    if (!oldest) return
    const el = commentsScrollRef.current
    if (el) restoreScrollRef.current = el.scrollHeight - el.scrollTop
    setLoadingOlderComments(true)
    try {
      const page = isSub
        ? await dispatch(
            crmApi.endpoints.listSubStageComments.initiate(
              { projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, before: oldest.id },
              { subscribe: false },
            ),
          ).unwrap()
        : await dispatch(
            crmApi.endpoints.listComments.initiate(
              { projectId, position: selectedStage.position, before: oldest.id },
              { subscribe: false },
            ),
          ).unwrap()
      if (page.length < COMMENTS_PAGE) setHasMoreComments(false)
      setOlderComments((prev) => [...page, ...prev])
    } finally {
      setLoadingOlderComments(false)
    }
  }, [projectId, selectedStage, loadingOlderComments, hasMoreComments, allComments, isSub, dispatch])

  const handleCommentsScroll = useCallback(() => {
    const el = commentsScrollRef.current
    if (el && el.scrollTop <= 48) handleLoadOlderComments()
  }, [handleLoadOlderComments])

  // Keep the viewport anchored when older comments are prepended.
  useLayoutEffect(() => {
    const el = commentsScrollRef.current
    if (el && restoreScrollRef.current !== null) {
      el.scrollTop = el.scrollHeight - restoreScrollRef.current
      restoreScrollRef.current = null
    }
  }, [olderComments])

  // Scroll to the newest comment once when a stage's comments first appear.
  useLayoutEffect(() => {
    const el = commentsScrollRef.current
    if (el && allComments.length > 0 && initialScrolledKeyRef.current !== stageKey) {
      el.scrollTop = el.scrollHeight
      initialScrolledKeyRef.current = stageKey
    }
  }, [stageKey, allComments.length])

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

  // ── Detail mutations (top-level) ───────────────────────────
  const [updateTopTitle]   = useUpdateStageTitleMutation()
  const [updateTopDeadline]= useUpdateStageDeadlineMutation()
  const [updateTopCost]    = useUpdateStageCostMutation()
  const [updateTopGip]     = useUpdateGipConfirmedMutation()
  const [updateTopPayment] = useUpdatePaymentConfirmedMutation()
  const [uploadTopAct, { isLoading: uploadingTopAct }]         = useUploadActMutation()
  const [deleteTopAct]     = useDeleteActMutation()
  const [uploadTopFile, { isLoading: uploadingTopFile }]       = useUploadAttachmentMutation()
  const [deleteTopFile]    = useDeleteAttachmentMutation()
  const [addTopComment, { isLoading: addingTopComment }]       = useAddCommentMutation()
  const [deleteTopComment] = useDeleteCommentMutation()

  // ── Detail mutations (sub-stage) ───────────────────────────
  const [updateSubTitle]   = useUpdateSubStageTitleMutation()
  const [updateSubDeadline]= useUpdateSubStageDeadlineMutation()
  const [updateSubCost]    = useUpdateSubStageCostMutation()
  const [updateSubGip]     = useUpdateSubStageGipConfirmedMutation()
  const [updateSubPayment] = useUpdateSubStagePaymentConfirmedMutation()
  const [uploadSubAct, { isLoading: uploadingSubAct }]         = useUploadSubStageActMutation()
  const [deleteSubAct]     = useDeleteSubStageActMutation()
  const [uploadSubFile, { isLoading: uploadingSubFile }]       = useUploadSubStageAttachmentMutation()
  const [deleteSubFile]    = useDeleteSubStageAttachmentMutation()
  const [addSubComment, { isLoading: addingSubComment }]       = useAddSubStageCommentMutation()
  const [deleteSubComment] = useDeleteSubStageCommentMutation()

  // Unified helpers
  const uploadingAct  = isSub ? uploadingSubAct  : uploadingTopAct
  const uploadingFile = isSub ? uploadingSubFile : uploadingTopFile
  const addingComment = isSub ? addingSubComment : addingTopComment

  const updateTitle   = isSub ? updateSubTitle   : updateTopTitle
  const updateDeadline= isSub ? updateSubDeadline: updateTopDeadline
  const updateCost    = isSub ? updateSubCost    : updateTopCost
  const updateGip     = isSub ? updateSubGip     : updateTopGip
  const updatePayment = isSub ? updateSubPayment : updateTopPayment

  const handleUpdateTitle = async (v: string) => {
    if (!v.trim() || !projectId || !selectedStage) return
    if (isSub) {
      await (updateTitle as typeof updateSubTitle)({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, title: v.trim() })
    } else {
      await (updateTitle as typeof updateTopTitle)({ projectId, position: selectedStage.position, title: v.trim() })
    }
  }

  const handleUpdateDeadline = async (v: string) => {
    if (!projectId || !selectedStage) return
    const deadline = v ? `${v}T00:00:00Z` : null
    if (isSub) {
      await (updateDeadline as typeof updateSubDeadline)({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, deadline })
    } else {
      await (updateDeadline as typeof updateTopDeadline)({ projectId, position: selectedStage.position, deadline })
    }
  }

  const handleUpdateCost = async (v: string) => {
    if (!projectId || !selectedStage) return
    const cost = v ? parseInt(v, 10) : null
    if (isSub) {
      await (updateCost as typeof updateSubCost)({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, cost })
    } else {
      await (updateCost as typeof updateTopCost)({ projectId, position: selectedStage.position, cost })
    }
  }

  const handleToggleGip = async () => {
    if (!projectId || !selectedStage || !detail) return
    if (isSub) {
      await (updateGip as typeof updateSubGip)({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, confirmed: !detail.gip_confirmed })
    } else {
      await (updateGip as typeof updateTopGip)({ projectId, position: selectedStage.position, confirmed: !detail.gip_confirmed })
    }
  }

  const handleTogglePayment = async () => {
    if (!projectId || !selectedStage || !detail) return
    if (isSub) {
      await (updatePayment as typeof updateSubPayment)({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, confirmed: !detail.payment_confirmed })
    } else {
      await (updatePayment as typeof updateTopPayment)({ projectId, position: selectedStage.position, confirmed: !detail.payment_confirmed })
    }
  }

  // ── File inputs ────────────────────────────────────────────
  const actFileInputRef  = useRef<HTMLInputElement>(null)
  const fileInputRef     = useRef<HTMLInputElement>(null)
  const [actUploadError, setActUploadError]   = useState<string | null>(null)
  const [uploadError, setUploadError]         = useState<string | null>(null)

  const handleActFileChange = useCallback(
    async (e: React.ChangeEvent<HTMLInputElement>) => {
      const original = e.target.files?.[0]
      if (!original || !projectId || !selectedStage) return
      setActUploadError(null)
      const buffer = await readFile(original)
      const file = new File([buffer], original.name || 'act', { type: original.type || 'application/octet-stream' })
      let result
      if (isSub) {
        result = await uploadSubAct({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, file })
      } else {
        result = await uploadTopAct({ projectId, position: selectedStage.position, file })
      }
      if (actFileInputRef.current) actFileInputRef.current.value = ''
      if ('error' in result) {
        const status = (result.error as { status?: number })?.status
        if (status === 413) setActUploadError('Файл слишком большой (макс. 50 МБ)')
        else setActUploadError('Не удалось загрузить акт')
      }
    },
    [projectId, selectedStage, isSub, uploadTopAct, uploadSubAct],
  )

  const handleFileChange = useCallback(
    async (e: React.ChangeEvent<HTMLInputElement>) => {
      const original = e.target.files?.[0]
      if (!original || !projectId || !selectedStage) return
      setUploadError(null)
      const buffer = await readFile(original)
      const file = new File([buffer], original.name || 'file', { type: original.type || 'application/octet-stream' })
      let result
      if (isSub) {
        result = await uploadSubFile({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, file })
      } else {
        result = await uploadTopFile({ projectId, position: selectedStage.position, file })
      }
      if (fileInputRef.current) fileInputRef.current.value = ''
      if ('error' in result) {
        const status = (result.error as { status?: number })?.status
        if (status === 413) setUploadError('Файл слишком большой (макс. 50 МБ)')
        else if (status === 400) setUploadError('Неверный формат запроса')
        else setUploadError('Не удалось загрузить файл')
      }
    },
    [projectId, selectedStage, isSub, uploadTopFile, uploadSubFile],
  )

  const handleDeleteAct = (actId: string) => {
    if (!projectId || !selectedStage) return
    if (isSub) {
      deleteSubAct({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, actId })
    } else {
      deleteTopAct({ projectId, position: selectedStage.position, actId })
    }
  }

  const handleDeleteAttachment = (attachmentId: string) => {
    if (!projectId || !selectedStage) return
    if (isSub) {
      deleteSubFile({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, attachmentId })
    } else {
      deleteTopFile({ projectId, position: selectedStage.position, attachmentId })
    }
  }

  const handleDeleteComment = (commentId: string) => {
    if (!projectId || !selectedStage) return
    if (isSub) {
      deleteSubComment({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, commentId })
    } else {
      deleteTopComment({ projectId, position: selectedStage.position, commentId })
    }
    setOlderComments((prev) => prev.filter((c) => c.id !== commentId))
  }

  // ── Stage list state ───────────────────────────────────────
  const [title, setTitle]       = useState('')
  const [position, setPosition] = useState('')
  const [expandedStages, setExpandedStages] = useState<Set<number>>(new Set())
  const [addingSubTo, setAddingSubTo]       = useState<number | null>(null)
  const [subTitle, setSubTitle]             = useState('')
  const bottomRef = useRef<HTMLDivElement>(null)
  const creating  = appending || inserting

  // ── Drag-and-drop reordering ───────────────────────────────
  // Top-level stages are dragged by position; sub-stages also carry their
  // parent so a drop only reorders within the same parent.
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

  const [commentText, setCommentText] = useState('')

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
        <header className={styles.header}>
          <button className={styles.backBtn} onClick={() => dispatch(selectStage(null))}>
            <ArrowLeftIcon />
          </button>
          <div className={styles.headerInfo}>
            <span className={styles.headerTitle}>{isSub ? 'Детали подэтапа' : 'Детали этапа'}</span>
          </div>
          <button
            className={styles.dangerBtn}
            onClick={() => detail && (
              isSub
                ? setPendingDelete({ kind: 'sub', parentPos: selectedStage.parentPosition, pos: selectedStage.position, stageTitle: detail.title })
                : setPendingDelete({ kind: 'stage', pos: selectedStage.position, stageTitle: detail.title })
            )}
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
                <EditableField
                  label="Название"
                  displayValue={detail.title}
                  rawValue={detail.title}
                  onSave={handleUpdateTitle}
                />
                <EditableField
                  label="Срок выполнения"
                  displayValue={detail.deadline
                    ? new Date(detail.deadline).toLocaleDateString('ru-RU', { day: '2-digit', month: 'long', year: 'numeric' })
                    : '—'}
                  rawValue={detail.deadline?.slice(0, 10) ?? ''}
                  type="date"
                  onSave={handleUpdateDeadline}
                />
                <div className={`${styles.field} ${styles.fieldEditable}`} onClick={handleToggleGip}>
                  <span className={styles.fieldLabel}>Выполнение ГИП</span>
                  <span className={styles.fieldValue}>
                    <span className={detail.gip_confirmed ? styles.completedBadge : styles.pendingBadge}>
                      {detail.gip_confirmed ? 'Выполнено' : 'Не выполнено'}
                    </span>
                  </span>
                </div>
              </div>

              <div className={styles.attachmentsSection}>
                <div className={styles.attachmentsHeader}>
                  <span className={styles.attachmentsSectionLabel}>Акты</span>
                  <label className={`${styles.attachUploadBtn} ${uploadingAct ? styles.attachUploadDisabled : ''}`}>
                    {uploadingAct ? <SpinnerIcon /> : <PaperclipIcon />}
                    {uploadingAct ? 'Загрузка…' : 'Загрузить акт'}
                    <input ref={actFileInputRef} type="file" className={styles.fileInputHidden}
                      onChange={handleActFileChange} disabled={uploadingAct} />
                  </label>
                </div>
                {actUploadError && <p className={styles.uploadError}>{actUploadError}</p>}
                {acts.length === 0 && !uploadingAct && <p className={styles.attachmentsEmpty}>Нет актов</p>}
                {acts.map((act) => (
                  <div key={act.id} className={styles.attachItem}>
                    <FileIcon mime={act.mime_type} />
                    <div className={styles.attachInfo}>
                      <button className={styles.attachName} onClick={() => downloadFile(act.download_url, act.filename)}>{act.filename}</button>
                      <span className={styles.attachMeta}>{formatBytes(act.size_bytes)}</span>
                    </div>
                    <button className={styles.attachDeleteBtn} title="Удалить акт" onClick={() => handleDeleteAct(act.id)}>
                      <CloseIcon />
                    </button>
                  </div>
                ))}
              </div>

              <div className={styles.fields}>
                <div className={styles.splitRow}>
                  <EditableField
                    label="Стоимость"
                    displayValue={detail.cost != null ? `${detail.cost.toLocaleString()} ₽` : '—'}
                    rawValue={detail.cost?.toString() ?? ''}
                    type="number"
                    onSave={handleUpdateCost}
                  />
                  <div className={`${styles.field} ${styles.fieldEditable}`} onClick={handleTogglePayment}>
                    <span className={styles.fieldLabel}>Подтверждение оплаты</span>
                    <span className={styles.fieldValue}>
                      <span className={detail.payment_confirmed ? styles.completedBadge : styles.pendingBadge}>
                        {detail.payment_confirmed ? 'Подтверждено' : 'Не подтверждено'}
                      </span>
                    </span>
                  </div>
                </div>
              </div>

              <div className={styles.attachmentsSection}>
                <div className={styles.attachmentsHeader}>
                  <span className={styles.attachmentsSectionLabel}>Файлы</span>
                  <label className={`${styles.attachUploadBtn} ${uploadingFile ? styles.attachUploadDisabled : ''}`} title="Прикрепить файл">
                    {uploadingFile ? <SpinnerIcon /> : <PaperclipIcon />}
                    {uploadingFile ? 'Загрузка…' : 'Прикрепить'}
                    <input ref={fileInputRef} type="file" className={styles.fileInputHidden}
                      onChange={handleFileChange} disabled={uploadingFile} />
                  </label>
                </div>
                {uploadError && <p className={styles.uploadError}>{uploadError}</p>}
                {attachments.length === 0 && !uploadingFile && <p className={styles.attachmentsEmpty}>Нет прикреплённых файлов</p>}
                {attachments.map((a) => (
                  <div key={a.id} className={styles.attachItem}>
                    <FileIcon mime={a.mime_type} />
                    <div className={styles.attachInfo}>
                      <button className={styles.attachName} onClick={() => downloadFile(a.download_url, a.filename)}>{a.filename}</button>
                      <span className={styles.attachMeta}>{formatBytes(a.size_bytes)}</span>
                    </div>
                    <button className={styles.attachDeleteBtn} title="Удалить файл" onClick={() => handleDeleteAttachment(a.id)}>
                      <CloseIcon />
                    </button>
                  </div>
                ))}
              </div>

              <div className={styles.attachmentsSection}>
                <div className={styles.attachmentsHeader}>
                  <span className={styles.attachmentsSectionLabel}>Комментарии</span>
                </div>
                <div className={styles.commentsScroll} ref={commentsScrollRef} onScroll={handleCommentsScroll}>
                  {loadingOlderComments && <div className={styles.commentsLoading}>Загрузка…</div>}
                  {allComments.length === 0 && <p className={styles.attachmentsEmpty}>Нет комментариев</p>}
                  {allComments.map((c) => c.is_system ? (
                    <div key={c.id} className={styles.systemComment}>
                      <span className={styles.systemCommentText}>
                        <span className={styles.systemCommentAuthor}>{c.author}</span>
                        {' · '}{c.text}
                      </span>
                      <span className={styles.systemCommentDate}>
                        {new Date(c.created_at).toLocaleString('ru-RU', { day: '2-digit', month: 'short', hour: '2-digit', minute: '2-digit' })}
                      </span>
                    </div>
                  ) : (
                    <div key={c.id} className={styles.commentBubble}>
                      <div className={styles.commentBubbleHeader}>
                        <span className={styles.commentAuthor}>{c.author}</span>
                        <span className={styles.commentDate}>
                          {new Date(c.created_at).toLocaleString('ru-RU', { day: '2-digit', month: 'short', hour: '2-digit', minute: '2-digit' })}
                        </span>
                        <button className={styles.commentDeleteBtn} title="Удалить" onClick={() => handleDeleteComment(c.id)}>
                          <CloseIcon />
                        </button>
                      </div>
                      <p className={styles.commentText}>{c.text}</p>
                    </div>
                  ))}
                </div>
                <div className={styles.commentInputRow}>
                  <textarea
                    className={styles.commentInput}
                    placeholder="Написать комментарий…"
                    value={commentText}
                    onChange={(e) => setCommentText(e.target.value)}
                    onKeyDown={(e) => {
                      if (e.key === 'Enter' && !e.shiftKey) {
                        e.preventDefault()
                        if (!commentText.trim() || addingComment) return
                        if (isSub) {
                          addSubComment({ projectId: projectId!, parentPosition: selectedStage.parentPosition, position: selectedStage.position, text: commentText.trim() })
                        } else {
                          addTopComment({ projectId: projectId!, position: selectedStage.position, text: commentText.trim() })
                        }
                        setCommentText('')
                      }
                    }}
                    rows={1}
                  />
                  <button
                    className={styles.sendBtn}
                    disabled={!commentText.trim() || addingComment}
                    onClick={() => {
                      if (!commentText.trim() || addingComment) return
                      if (isSub) {
                        addSubComment({ projectId: projectId!, parentPosition: selectedStage.parentPosition, position: selectedStage.position, text: commentText.trim() })
                      } else {
                        addTopComment({ projectId: projectId!, position: selectedStage.position, text: commentText.trim() })
                      }
                      setCommentText('')
                    }}
                  >
                    <SendIcon />
                  </button>
                </div>
              </div>
            </div>
          )}
          {!detailLoading && !detail && <div className={styles.loading}>Нет данных</div>}
        </div>
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
        <button className={styles.dangerBtn} onClick={() => setPendingDelete({ kind: 'project' })} title="Удалить проект">
          <TrashIcon />
        </button>
      </header>

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
                <button
                  className={`${styles.stageChevron} ${(children.length > 0 || expanded) ? styles.stageChevronVisible : ''} ${expanded ? styles.stageChevronOpen : ''}`}
                  onClick={(e) => { e.stopPropagation(); toggleExpand(stage.position) }}
                  title={expanded ? 'Свернуть' : 'Развернуть'}
                >
                  <ChevronRightIcon />
                </button>
                <span className={styles.stageCheck} title={stage.completed ? 'Этап выполнен' : 'Этап не выполнен'}>
                  {stage.completed ? <CheckCircleIcon /> : <CircleIcon />}
                </span>
                <div className={styles.stageInfo}>
                  <span className={styles.stageTitle}>{stage.title}</span>
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
                        <span className={styles.stageTitle}>{child.title}</span>
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
    </div>
  )
}

// ── InlineEdit ─────────────────────────────────────────────
function InlineEdit({ value, onSave, className }: {
  value: string
  onSave: (value: string) => Promise<void>
  className?: string
}) {
  const [editing, setEditing] = useState(false)
  const [draft, setDraft]     = useState('')
  const cancelled             = useRef(false)

  const startEdit = () => { setDraft(value); setEditing(true) }

  const handleBlur = async () => {
    if (cancelled.current) { cancelled.current = false; return }
    setEditing(false)
    await onSave(draft)
  }

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter')  (e.target as HTMLElement).blur()
    if (e.key === 'Escape') { cancelled.current = true; (e.target as HTMLElement).blur() }
  }

  return editing ? (
    <input autoFocus className={`${className ?? ''} ${styles.inlineInput}`}
      value={draft} onChange={(e) => setDraft(e.target.value)}
      onBlur={handleBlur} onKeyDown={handleKeyDown} />
  ) : (
    <span className={`${className ?? ''} ${styles.inlineValue}`} onClick={startEdit} title="Переименовать">
      {value}<PencilIcon />
    </span>
  )
}

// ── EditableField ──────────────────────────────────────────
function EditableField({ label, displayValue, rawValue, onSave, type = 'text', multiline = false }: {
  label: string
  displayValue: string
  rawValue: string
  onSave: (value: string) => Promise<void>
  type?: 'text' | 'number' | 'date'
  multiline?: boolean
}) {
  const [editing, setEditing] = useState(false)
  const [draft, setDraft]     = useState('')
  const cancelled             = useRef(false)

  const startEdit = () => { setDraft(rawValue); setEditing(true) }

  const handleBlur = async () => {
    if (cancelled.current) { cancelled.current = false; return }
    setEditing(false)
    await onSave(draft)
  }

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    if (e.key === 'Enter' && !multiline) (e.target as HTMLElement).blur()
    if (e.key === 'Escape') { cancelled.current = true; (e.target as HTMLElement).blur() }
  }

  return (
    <div className={`${styles.field} ${styles.fieldEditable}`} onClick={!editing ? startEdit : undefined}>
      <span className={styles.fieldLabel}>{label}</span>
      {editing ? (
        multiline ? (
          <textarea autoFocus className={styles.fieldInput} value={draft}
            onChange={(e) => setDraft(e.target.value)} onBlur={handleBlur} onKeyDown={handleKeyDown} rows={3} />
        ) : (
          <input autoFocus type={type} className={styles.fieldInput} value={draft}
            onChange={(e) => setDraft(e.target.value)} onBlur={handleBlur} onKeyDown={handleKeyDown} />
        )
      ) : (
        <span className={styles.fieldValue}>{displayValue}</span>
      )}
    </div>
  )
}

// ── Helpers ────────────────────────────────────────────────
async function downloadFile(url: string, filename: string) {
  const token = store.getState().auth.accessToken
  const res = await fetch(url, { headers: token ? { Authorization: `Bearer ${token}` } : {} })
  if (!res.ok) return
  const blob = await res.blob()
  const blobUrl = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = blobUrl
  a.download = filename
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  setTimeout(() => URL.revokeObjectURL(blobUrl), 10000)
}

function readFile(file: File): Promise<ArrayBuffer> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.onload  = () => resolve(reader.result as ArrayBuffer)
    reader.onerror = () => reject(reader.error)
    reader.readAsArrayBuffer(file)
  })
}

function formatBytes(bytes: number): string {
  if (bytes < 1024)       return `${bytes} B`
  if (bytes < 1_048_576)  return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / 1_048_576).toFixed(1)} MB`
}

// ── Icons ──────────────────────────────────────────────────
function ArrowLeftIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
      <path d="M15 18l-6-6 6-6" stroke="currentColor" strokeWidth="2.2" strokeLinecap="round" strokeLinejoin="round"/>
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
      <path d="M22 2 15 22 11 13 2 9l20-7Z" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
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
      <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
      <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L13 14l-4 1 1-4 8.5-8.5Z" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
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
function CircleIcon() {
  return (
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
      <circle cx="12" cy="12" r="9" stroke="currentColor" strokeWidth="2"/>
    </svg>
  )
}
function CheckCircleIcon() {
  return (
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
      <circle cx="12" cy="12" r="9" stroke="currentColor" strokeWidth="2"/>
      <path d="m8 12 3 3 5-5" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
    </svg>
  )
}
function PaperclipIcon() {
  return (
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none">
      <path d="M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48"
        stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
    </svg>
  )
}
function FileIcon({ mime }: { mime: string }) {
  const isImage = mime.startsWith('image/')
  const isPdf   = mime === 'application/pdf'
  const color   = isImage ? '#65aadd' : isPdf ? '#e53935' : '#708499'
  return (
    <svg width="28" height="28" viewBox="0 0 24 24" fill="none" style={{ flexShrink: 0, color }}>
      <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8Z" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round"/>
      <path d="M14 2v6h6" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round"/>
    </svg>
  )
}
function SpinnerIcon() {
  return (
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" style={{ animation: 'spin 0.8s linear infinite' }}>
      <circle cx="12" cy="12" r="9" stroke="currentColor" strokeWidth="2.5" strokeDasharray="40 20" strokeLinecap="round"/>
    </svg>
  )
}
function ChevronRightIcon() {
  return (
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none">
      <path d="M9 18l6-6-6-6" stroke="currentColor" strokeWidth="2.2" strokeLinecap="round" strokeLinejoin="round"/>
    </svg>
  )
}
function PlusIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 24 24" fill="none">
      <path d="M12 5v14M5 12h14" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round"/>
    </svg>
  )
}