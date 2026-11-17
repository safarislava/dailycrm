import React from 'react'
import type {DetailedStage} from '../../../../types'
import {ArrowLeftIcon, TrashIcon} from '../Helpers'
import styles from '../../MainPanel.module.scss'

interface StageHeaderProps {
  isSub: boolean
  selectedStage: { parentPosition: number; position: number }
  detail: DetailedStage | undefined
    isArchived?: boolean
  onBack: () => void
  setPendingDelete: (deleteObj: any) => void
}

export default function StageHeader({
  isSub,
  selectedStage,
  detail,
                                        isArchived = false,
  onBack,
  setPendingDelete,
}: StageHeaderProps) {
  return (
    <header className={styles.header}>
      <button
        className={styles.backBtn}
        onClick={onBack}
        title="Назад к списку этапов"
      >
        <ArrowLeftIcon />
      </button>
      <div className={styles.headerInfo}>
        <span className={styles.headerTitle}>
          {isSub
            ? `Детали подэтапа ${selectedStage.parentPosition}.${selectedStage.position}`
            : `Детали этапа ${selectedStage.position}`}
        </span>
      </div>
        {!isArchived && (
            <button
                className={styles.dangerBtn}
                onClick={() =>
                    detail &&
                    (isSub
                        ? setPendingDelete({
                            kind: 'sub',
                            parentPos: selectedStage.parentPosition,
                            pos: selectedStage.position,
                            stageTitle: detail.title,
                        })
                        : setPendingDelete({
                            kind: 'stage',
                            pos: selectedStage.position,
                            stageTitle: detail.title,
                        }))
                }
                title="Удалить этап"
            >
                <TrashIcon/>
            </button>
        )}
    </header>
  )
}

