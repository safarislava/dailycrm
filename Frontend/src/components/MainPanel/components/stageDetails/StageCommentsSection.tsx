import React, { useState, useRef, useCallback, useEffect, useLayoutEffect, useMemo } from 'react'
import { useDispatch } from 'react-redux'
import type { AppDispatch } from '../../../../store'
import type { Comment } from '../../../../types'
import {
  stageCommentsApi,
  useAddCommentMutation,
  useDeleteCommentMutation,
  usePinCommentMutation,
  useAddSubStageCommentMutation,
  useDeleteSubStageCommentMutation,
  usePinSubStageCommentMutation,
} from '../../../../store/crmApi'
import { CloseIcon, SendIcon, PinIcon } from '../Helpers'
import styles from '../../MainPanel.module.scss'

interface StageCommentsSectionProps {
  projectId: string
  selectedStage: { parentPosition: number; position: number }
  isSub: boolean
    isArchived?: boolean
  comments: Comment[]
  initialPinnedComments: Comment[]
}

export default function StageCommentsSection({
  projectId,
  selectedStage,
  isSub,
                                                 isArchived = false,
  comments,
  initialPinnedComments,
}: StageCommentsSectionProps) {

  const dispatch = useDispatch<AppDispatch>()

  const [addTopComment, { isLoading: addingTopComment }] = useAddCommentMutation()
  const [deleteTopComment] = useDeleteCommentMutation()
  const [pinTopComment] = usePinCommentMutation()

  const [addSubComment, { isLoading: addingSubComment }] = useAddSubStageCommentMutation()
  const [deleteSubComment] = useDeleteSubStageCommentMutation()
  const [pinSubComment] = usePinSubStageCommentMutation()

  const addingComment = isSub ? addingSubComment : addingTopComment

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
            stageCommentsApi.endpoints.listSubStageComments.initiate(
              { projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, before: oldest.id },
              { subscribe: false },
            ),
          ).unwrap()
        : await dispatch(
            stageCommentsApi.endpoints.listComments.initiate(
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

  useLayoutEffect(() => {
    const el = commentsScrollRef.current
    if (el && restoreScrollRef.current !== null) {
      el.scrollTop = el.scrollHeight - restoreScrollRef.current
      restoreScrollRef.current = null
    }
  }, [olderComments])

  useLayoutEffect(() => {
    const el = commentsScrollRef.current
    if (el && allComments.length > 0 && initialScrolledKeyRef.current !== stageKey) {
      el.scrollTop = el.scrollHeight
      initialScrolledKeyRef.current = stageKey
    }
  }, [stageKey, allComments.length])

  useLayoutEffect(() => {
    const el = commentsScrollRef.current
    if (el && pendingScrollToBottomRef.current) {
      el.scrollTop = el.scrollHeight
      pendingScrollToBottomRef.current = false
    }
  }, [allComments])

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
      prev.map((c) => (c.id === commentId ? { ...c, is_pinned: pinned } : c)),
    )
  }

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
    <div className={styles.attachmentsSection}>
      <div className={styles.attachmentsHeader}>
        <span className={styles.attachmentsSectionLabel}>Комментарии</span>
      </div>

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
                      {new Date(c.created_at).toLocaleString('ru-RU', {
                        day: '2-digit',
                        month: 'short',
                        hour: '2-digit',
                        minute: '2-digit',
                      })}
                    </span>
                      {!isArchived && (
                          <button
                              className={`${styles.commentPinBtn} ${styles.commentPinned}`}
                              title="Открепить"
                              onClick={() => handleTogglePinComment(c.id, false)}
                          >
                              <PinIcon filled/>
                          </button>
                      )}
                      {!isArchived && (
                          <button
                              className={styles.commentDeleteBtn}
                              title="Удалить"
                              onClick={() => handleDeleteComment(c.id)}
                          >
                              <CloseIcon/>
                          </button>
                      )}

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

        {allComments
          .filter((c) => !c.is_pinned)
          .map((c) =>
            c.is_system ? (
              <div key={c.id} className={styles.systemComment}>
                <span className={styles.systemCommentText}>
                  <span className={styles.systemCommentAuthor}>{c.author}</span>
                  {' · '}
                  {c.text}
                </span>
                <span className={styles.systemCommentDate}>
                  {new Date(c.created_at).toLocaleString('ru-RU', {
                    day: '2-digit',
                    month: 'short',
                    hour: '2-digit',
                    minute: '2-digit',
                  })}
                </span>
              </div>
            ) : (
              <div key={c.id} className={styles.commentBubble}>
                <div className={styles.commentBubbleHeader}>
                  <span className={styles.commentAuthor}>{c.author}</span>
                  <span className={styles.commentDate}>
                    {new Date(c.created_at).toLocaleString('ru-RU', {
                      day: '2-digit',
                      month: 'short',
                      hour: '2-digit',
                      minute: '2-digit',
                    })}
                  </span>
                    {!isArchived && (
                        <button
                            className={styles.commentPinBtn}
                            title="Закрепить"
                            onClick={() => handleTogglePinComment(c.id, true)}
                        >
                            <PinIcon/>
                        </button>
                    )}
                    {!isArchived && (
                        <button
                            className={styles.commentDeleteBtn}
                            title="Удалить"
                            onClick={() => handleDeleteComment(c.id)}
                        >
                            <CloseIcon/>
                        </button>
                    )}
                </div>
                <p className={styles.commentText}>{c.text}</p>
              </div>
            ),
          )}
      </div>

        {!isArchived && (
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
                    <SendIcon/>
                </button>
            </div>
        )}
    </div>
  )
}

