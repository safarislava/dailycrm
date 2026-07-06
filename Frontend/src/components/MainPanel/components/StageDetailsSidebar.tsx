import React, { useState, useRef, useCallback, useEffect, useLayoutEffect, useMemo } from 'react'
import { useDispatch, useSelector } from 'react-redux'
import type { AppDispatch, RootState } from '../../../store'
import type { Comment, Stage } from '../../../types'
import { selectStage } from '../../../store/uiSlice'
import {
  crmApi,
  useGetDetailedStageQuery,
  useGetDetailedSubStageQuery,
  useListActsQuery,
  useListSubStageActsQuery,
  useListAttachmentsQuery,
  useListSubStageAttachmentsQuery,
  useListCommentsQuery,
  useListSubStageCommentsQuery,
  useListPinnedCommentsQuery,
  useListPinnedSubStageCommentsQuery,
  useUpdateStageTitleMutation,
  useUpdateStageDeadlineMutation,
  useUpdateAdvanceCostMutation,
  useUpdateFinalCostMutation,
  useUpdateGipConfirmedMutation,
  useUpdateAdvanceConfirmedMutation,
  useUpdateFinalConfirmedMutation,
  useUploadActMutation,
  useDeleteActMutation,
  useUploadAttachmentMutation,
  useDeleteAttachmentMutation,
  useAddCommentMutation,
  useDeleteCommentMutation,
  usePinCommentMutation,
  useUpdateSubStageTitleMutation,
  useUpdateSubStageDeadlineMutation,
  useUpdateSubStageAdvanceCostMutation,
  useUpdateSubStageFinalCostMutation,
  useUpdateSubStageGipConfirmedMutation,
  useUpdateSubStageAdvanceConfirmedMutation,
  useUpdateSubStageFinalConfirmedMutation,
  useUploadSubStageActMutation,
  useDeleteSubStageActMutation,
  useUploadSubStageAttachmentMutation,
  useDeleteSubStageAttachmentMutation,
  useAddSubStageCommentMutation,
  useDeleteSubStageCommentMutation,
  usePinSubStageCommentMutation,
} from '../../../store/crmApi'
import {
  InlineEdit,
  EditableField,
  downloadFile,
  readFile,
  formatBytes,
  ArrowLeftIcon,
  TrashIcon,
  PaperclipIcon,
  CloseIcon,
  FileIcon,
  SpinnerIcon,
  SendIcon,
  PinIcon,
} from './Helpers'
import styles from '../MainPanel.module.scss'

interface StageDetailsSidebarProps {
  projectId: string
  selectedStage: { parentPosition: number; position: number }
  isSub: boolean
  setPendingDelete: (deleteObj: any) => void
}

export default function StageDetailsSidebar({
  projectId,
  selectedStage,
  isSub,
  setPendingDelete,
}: StageDetailsSidebarProps) {
  const dispatch = useDispatch<AppDispatch>()

  // ── Detail queries (one fires, one skips) ─────────────────
  const { data: topDetail, isLoading: topDetailLoading } = useGetDetailedStageQuery(
    { projectId, position: selectedStage.position },
    { skip: isSub },
  )
  const { data: subDetail, isLoading: subDetailLoading } = useGetDetailedSubStageQuery(
    { projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position },
    { skip: !isSub },
  )
  const detail        = isSub ? subDetail : topDetail
  const detailLoading = isSub ? subDetailLoading : topDetailLoading

  // ── Attachments / acts / comments queries ──────────────────
  const actArgs        = { projectId, position: selectedStage.position }
  const subActArgs     = { projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position }
  const skipTop        = isSub
  const skipSub        = !isSub

  const { data: topActs = [] }         = useListActsQuery(actArgs, { skip: skipTop })
  const { data: subActs = [] }         = useListSubStageActsQuery(subActArgs, { skip: skipSub })
  const acts = isSub ? subActs : topActs

  const { data: topAttachments = [] }  = useListAttachmentsQuery(actArgs, { skip: skipTop })
  const { data: subAttachments = [] }  = useListSubStageAttachmentsQuery(subActArgs, { skip: skipSub })
  const attachments = isSub ? subAttachments : topAttachments

  const { data: topComments = [] }     = useListCommentsQuery(actArgs, { skip: skipTop })
  const { data: subComments = [] }     = useListSubStageCommentsQuery(subActArgs, { skip: skipSub })
  const comments = isSub ? subComments : topComments

  const { data: topPinnedComments = [] } = useListPinnedCommentsQuery(actArgs, { skip: skipTop })
  const { data: subPinnedComments = [] } = useListPinnedSubStageCommentsQuery(subActArgs, { skip: skipSub })
  const initialPinnedComments = isSub ? subPinnedComments : topPinnedComments

  // ── Lazy-loaded older comments ─────────────────────────────
  const COMMENTS_PAGE = 25
  const stageKey = `${projectId}-${selectedStage.parentPosition}-${selectedStage.position}`
  const [olderComments, setOlderComments] = useState<Comment[]>([])
  const [hasMoreComments, setHasMoreComments] = useState(true)
  const [loadingOlderComments, setLoadingOlderComments] = useState(false)
  const commentsScrollRef = useRef<HTMLDivElement>(null)
  const restoreScrollRef = useRef<number | null>(null)
  const initialScrolledKeyRef = useRef<string | null>(null)
  const pendingScrollToBottomRef = useRef(false)

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
    if (loadingOlderComments || !hasMoreComments) return
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

  // Scroll to the bottom once the comment just sent lands in the list.
  useLayoutEffect(() => {
    const el = commentsScrollRef.current
    if (el && pendingScrollToBottomRef.current) {
      el.scrollTop = el.scrollHeight
      pendingScrollToBottomRef.current = false
    }
  }, [allComments])

  // ── Detail mutations (top-level) ───────────────────────────
  const [updateTopTitle]   = useUpdateStageTitleMutation()
  const [updateTopDeadline]= useUpdateStageDeadlineMutation()
  const [updateTopAdvanceCost] = useUpdateAdvanceCostMutation()
  const [updateTopFinalCost]   = useUpdateFinalCostMutation()
  const [updateTopGip]     = useUpdateGipConfirmedMutation()
  const [updateTopAdvanceConfirmed] = useUpdateAdvanceConfirmedMutation()
  const [updateTopFinalConfirmed]   = useUpdateFinalConfirmedMutation()
  const [uploadTopAct, { isLoading: uploadingTopAct }]         = useUploadActMutation()
  const [deleteTopAct]     = useDeleteActMutation()
  const [uploadTopFile, { isLoading: uploadingTopFile }]       = useUploadAttachmentMutation()
  const [deleteTopFile]    = useDeleteAttachmentMutation()
  const [addTopComment, { isLoading: addingTopComment }]       = useAddCommentMutation()
  const [deleteTopComment] = useDeleteCommentMutation()
  const [pinTopComment] = usePinCommentMutation()

  // ── Detail mutations (sub-stage) ───────────────────────────
  const [updateSubTitle]   = useUpdateSubStageTitleMutation()
  const [updateSubDeadline]= useUpdateSubStageDeadlineMutation()
  const [updateSubAdvanceCost] = useUpdateSubStageAdvanceCostMutation()
  const [updateSubFinalCost]   = useUpdateSubStageFinalCostMutation()
  const [updateSubGip]     = useUpdateSubStageGipConfirmedMutation()
  const [updateSubAdvanceConfirmed] = useUpdateSubStageAdvanceConfirmedMutation()
  const [updateSubFinalConfirmed]   = useUpdateSubStageFinalConfirmedMutation()
  const [uploadSubAct, { isLoading: uploadingSubAct }]         = useUploadSubStageActMutation()
  const [deleteSubAct]     = useDeleteSubStageActMutation()
  const [uploadSubFile, { isLoading: uploadingSubFile }]       = useUploadSubStageAttachmentMutation()
  const [deleteSubFile]    = useDeleteSubStageAttachmentMutation()
  const [addSubComment, { isLoading: addingSubComment }]       = useAddSubStageCommentMutation()
  const [deleteSubComment] = useDeleteSubStageCommentMutation()
  const [pinSubComment] = usePinSubStageCommentMutation()

  // Unified helpers
  const uploadingAct  = isSub ? uploadingSubAct  : uploadingTopAct
  const uploadingFile = isSub ? uploadingSubFile : uploadingTopFile
  const addingComment = isSub ? addingSubComment : addingTopComment

  const updateTitle   = isSub ? updateSubTitle   : updateTopTitle
  const updateDeadline= isSub ? updateSubDeadline: updateTopDeadline
  const updateAdvanceCost      = isSub ? updateSubAdvanceCost      : updateTopAdvanceCost
  const updateFinalCost        = isSub ? updateSubFinalCost        : updateTopFinalCost
  const updateGip               = isSub ? updateSubGip               : updateTopGip
  const updateAdvanceConfirmed = isSub ? updateSubAdvanceConfirmed : updateTopAdvanceConfirmed
  const updateFinalConfirmed   = isSub ? updateSubFinalConfirmed   : updateTopFinalConfirmed

  const handleUpdateTitle = async (v: string) => {
    if (!v.trim()) return
    if (isSub) {
      await (updateTitle as typeof updateSubTitle)({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, title: v.trim() })
    } else {
      await (updateTitle as typeof updateTopTitle)({ projectId, position: selectedStage.position, title: v.trim() })
    }
  }

  const handleUpdateDeadline = async (v: string) => {
    const deadline = v ? `${v}T00:00:00Z` : null
    if (isSub) {
      await (updateDeadline as typeof updateSubDeadline)({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, deadline })
    } else {
      await (updateDeadline as typeof updateTopDeadline)({ projectId, position: selectedStage.position, deadline })
    }
  }

  const handleUpdateAdvanceCost = async (v: string) => {
    const cost = v ? parseInt(v, 10) : null
    if (isSub) {
      await (updateAdvanceCost as typeof updateSubAdvanceCost)({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, cost })
    } else {
      await (updateAdvanceCost as typeof updateTopAdvanceCost)({ projectId, position: selectedStage.position, cost })
    }
  }

  const handleUpdateFinalCost = async (v: string) => {
    const cost = v ? parseInt(v, 10) : null
    if (isSub) {
      await (updateFinalCost as typeof updateSubFinalCost)({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, cost })
    } else {
      await (updateFinalCost as typeof updateTopFinalCost)({ projectId, position: selectedStage.position, cost })
    }
  }

  const handleToggleGip = async () => {
    if (!detail) return
    if (isSub) {
      await (updateGip as typeof updateSubGip)({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, confirmed: !detail.gip_confirmed })
    } else {
      await (updateGip as typeof updateTopGip)({ projectId, position: selectedStage.position, confirmed: !detail.gip_confirmed })
    }
  }

  const handleToggleAdvancePayment = async () => {
    if (!detail) return
    if (isSub) {
      await (updateAdvanceConfirmed as typeof updateSubAdvanceConfirmed)({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, confirmed: !detail.advance_confirmed })
    } else {
      await (updateAdvanceConfirmed as typeof updateTopAdvanceConfirmed)({ projectId, position: selectedStage.position, confirmed: !detail.advance_confirmed })
    }
  }

  const handleToggleFinalPayment = async () => {
    if (!detail) return
    if (isSub) {
      await (updateFinalConfirmed as typeof updateSubFinalConfirmed)({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, confirmed: !detail.final_confirmed })
    } else {
      await (updateFinalConfirmed as typeof updateTopFinalConfirmed)({ projectId, position: selectedStage.position, confirmed: !detail.final_confirmed })
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
      if (!original) return
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
      if (!original) return
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
    if (isSub) {
      deleteSubAct({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, actId })
    } else {
      deleteTopAct({ projectId, position: selectedStage.position, actId })
    }
  }

  const handleDeleteAttachment = (attachmentId: string) => {
    if (isSub) {
      deleteSubFile({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, attachmentId })
    } else {
      deleteTopFile({ projectId, position: selectedStage.position, attachmentId })
    }
  }

  const handleDeleteComment = (commentId: string) => {
    if (isSub) {
      deleteSubComment({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, commentId })
    } else {
      deleteTopComment({ projectId, position: selectedStage.position, commentId })
    }
    setOlderComments((prev) => prev.filter((c) => c.id !== commentId))
  }

  const handleTogglePinComment = async (commentId: string, pinned: boolean) => {
    if (isSub) {
      await pinSubComment({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, commentId, pinned })
    } else {
      await pinTopComment({ projectId, position: selectedStage.position, commentId, pinned })
    }
    setOlderComments((prev) =>
      prev.map((c) => (c.id === commentId ? { ...c, is_pinned: pinned } : c))
    )
  }

  // ── Comments send input ────────────────────────────────────
  const [commentText, setCommentText] = useState('')

  const handleSendComment = async () => {
    const text = commentText.trim()
    if (!text || addingComment) return
    setCommentText('')
    pendingScrollToBottomRef.current = true
    if (isSub) {
      await addSubComment({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, text })
    } else {
      await addTopComment({ projectId, position: selectedStage.position, text })
    }
  }

  return (
    <>
      <header className={styles.header}>
        <button
          className={`${styles.backBtn} ${styles.mobileOnly}`}
          onClick={() => dispatch(selectStage(null))}
        >
          <ArrowLeftIcon />
        </button>
        <div className={styles.headerInfo}>
          <span className={styles.headerTitle}>
            {isSub ? `Детали подэтапа ${selectedStage.parentPosition}.${selectedStage.position}` : `Детали этапа ${selectedStage.position}`}
          </span>
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
                <span className={styles.fieldLabel}>Выполнение</span>
                <span className={styles.fieldValue}>
                  <span className={detail.gip_confirmed ? styles.completedBadge : styles.pendingBadge}>
                    {detail.gip_confirmed ? 'Выполнено' : 'Не выполнено'}
                  </span>
                </span>
              </div>
            </div>

            <div className={styles.attachmentsSection}>
              <div className={styles.attachmentsHeader}>
                <div className={styles.attachmentsHeaderLeft}>
                  <span className={styles.attachmentsSectionLabel}>Акты</span>
                  {acts.length > 0 && <span className={styles.completedBadge}>Акт загружен</span>}
                </div>
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
                  label="Аванс"
                  displayValue={detail.advance_cost != null ? `${detail.advance_cost.toLocaleString()} ₽` : '—'}
                  rawValue={detail.advance_cost?.toString() ?? ''}
                  type="number"
                  onSave={handleUpdateAdvanceCost}
                />
                <div
                  className={`${styles.field} ${detail.advance_cost != null ? styles.fieldEditable : ''}`}
                  onClick={detail.advance_cost != null ? handleToggleAdvancePayment : undefined}
                >
                  <span className={styles.fieldLabel}>Подтверждение аванса</span>
                  <span className={styles.fieldValue}>
                    {detail.advance_cost == null ? (
                      <span className={styles.pendingBadge}>Не требуется</span>
                    ) : (
                      <span className={detail.advance_confirmed ? styles.completedBadge : styles.pendingBadge}>
                        {detail.advance_confirmed ? 'Подтверждено' : 'Не подтверждено'}
                      </span>
                    )}
                  </span>
                </div>
              </div>
              <div className={styles.splitRow}>
                <EditableField
                  label="Окончательная оплата"
                  displayValue={detail.final_cost != null ? `${detail.final_cost.toLocaleString()} ₽` : '—'}
                  rawValue={detail.final_cost?.toString() ?? ''}
                  type="number"
                  onSave={handleUpdateFinalCost}
                />
                <div className={`${styles.field} ${styles.fieldEditable}`} onClick={handleToggleFinalPayment}>
                  <span className={styles.fieldLabel}>Подтверждение оплаты</span>
                  <span className={styles.fieldValue}>
                    <span className={detail.final_confirmed ? styles.completedBadge : styles.pendingBadge}>
                      {detail.final_confirmed ? 'Подтверждено' : 'Не подтверждено'}
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

              {/* Pinned comments */}
              {initialPinnedComments.length > 0 && (
                <div className={styles.pinnedCommentsContainer}>
                  <div className={styles.pinnedScroll}>
                    {initialPinnedComments.map((c) => (
                      <div key={`pinned-${c.id}`} className={styles.pinnedComment}>
                        <PinIcon filled className={styles.pinnedCommentIcon} />
                        <div className={styles.pinnedCommentBody}>
                          <div className={styles.commentBubbleHeader}>
                            <span className={styles.commentAuthor}>{c.author}</span>
                            <span className={styles.commentDate}>
                              {new Date(c.created_at).toLocaleString('ru-RU', { day: '2-digit', month: 'short', hour: '2-digit', minute: '2-digit' })}
                            </span>
                            <button className={`${styles.commentPinBtn} ${styles.commentPinned}`} title="Открепить" onClick={() => handleTogglePinComment(c.id, false)}>
                              <PinIcon filled />
                            </button>
                            <button className={styles.commentDeleteBtn} title="Удалить" onClick={() => handleDeleteComment(c.id)}>
                              <CloseIcon />
                            </button>
                          </div>
                          <p className={styles.commentText}>{c.text}</p>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              )}

              <div className={styles.commentsScroll} ref={commentsScrollRef} onScroll={handleCommentsScroll}>
                {loadingOlderComments && <div className={styles.commentsLoading}>Загрузка…</div>}
                {allComments.length === 0 && <p className={styles.attachmentsEmpty}>Нет комментариев</p>}

                {/* Regular feed */}
                {allComments.filter((c) => !c.is_pinned).map((c) => c.is_system ? (
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
                      <button className={styles.commentPinBtn} title="Закрепить" onClick={() => handleTogglePinComment(c.id, true)}>
                        <PinIcon />
                      </button>
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
                      handleSendComment()
                    }
                  }}
                  rows={1}
                />
                <button
                  className={styles.sendBtn}
                  disabled={!commentText.trim() || addingComment}
                  onClick={handleSendComment}
                >
                  <SendIcon />
                </button>
              </div>
            </div>
          </div>
        )}
        {!detailLoading && !detail && <div className={styles.loading}>Нет данных</div>}
      </div>
    </>
  )
}
