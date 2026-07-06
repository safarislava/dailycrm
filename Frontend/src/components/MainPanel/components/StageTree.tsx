import React, { useState, useRef } from 'react'
import type { Stage } from '../../../types'
import { selectStage } from '../../../store/uiSlice'
import {
  ChevronRightIcon,
  CheckCircleIcon,
  CircleIcon,
  PlusIcon,
  CloseIcon,
  SendIcon,
  ListIcon,
} from './Helpers'
import styles from '../MainPanel.module.scss'

interface StageTreeProps {
  projectId: string
  stagesLoading: boolean
  stages: Stage[]
  setPendingDelete: (deleteObj: any) => void
  reorderStage: (args: { projectId: string; position: number; to: number }) => Promise<any>
  reorderSubStage: (args: { projectId: string; parentPosition: number; position: number; to: number }) => Promise<any>
  appendSubStage: (args: { projectId: string; parentPosition: number; title: string }) => Promise<any>
  dispatch: any
}

export default function StageTree({
  projectId,
  stagesLoading,
  stages,
  setPendingDelete,
  reorderStage,
  reorderSubStage,
  appendSubStage,
  dispatch,
}: StageTreeProps) {
  const [expandedStages, setExpandedStages] = useState<Set<number>>(new Set())
  const [addingSubTo, setAddingSubTo]       = useState<number | null>(null)
  const [subTitle, setSubTitle]             = useState('')
  const bottomRef = useRef<HTMLDivElement>(null)

  // ── Drag-and-drop reordering ───────────────────────────────
  const [dragPos, setDragPos]           = useState<number | null>(null)
  const [dragOverPos, setDragOverPos]   = useState<number | null>(null)
  const [dragSub, setDragSub]           = useState<{ parent: number; pos: number } | null>(null)
  const [dragOverSub, setDragOverSub]   = useState<{ parent: number; pos: number } | null>(null)

  const topLevelStages = stages.filter(s => s.parent_position === 0)
  const childrenOf = (pos: number) => stages.filter(s => s.parent_position === pos)

  const handleStageDrop = async (target: number) => {
    const from = dragPos
    setDragPos(null)
    setDragOverPos(null)
    if (from === null || from === target) return
    await reorderStage({ projectId, position: from, to: target })
  }

  const handleSubDrop = async (parent: number, target: number) => {
    const dragged = dragSub
    setDragSub(null)
    setDragOverSub(null)
    if (!dragged || dragged.parent !== parent || dragged.pos === target) return
    await reorderSubStage({ projectId, parentPosition: parent, position: dragged.pos, to: target })
  }

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

  const handleSendSub = async (parentPos: number) => {
    const text = subTitle.trim()
    if (!text) return
    await appendSubStage({ projectId, parentPosition: parentPos, title: text })
    setSubTitle('')
    setAddingSubTo(null)
  }

  const canSendSub = subTitle.trim() !== ''

  return (
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
  )
}
