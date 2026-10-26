import React from 'react'
import type {DetailedStage} from '../../../../types'
import {EditableField, formatNumber} from '../Helpers'
import styles from '../../MainPanel.module.scss'

interface StageFinancialsProps {
  detail: DetailedStage
  onUpdateTitle: (title: string) => Promise<void>
  onUpdateDeadline: (deadline: string) => Promise<void>
  onToggleGip: () => void
  onUpdateAdvanceCost: (cost: string) => Promise<void>
  onToggleAdvancePayment: () => void
  onUpdateFinalCost: (cost: string) => Promise<void>
  onToggleFinalPayment: () => void
}

export default function StageFinancials({
  detail,
  onUpdateTitle,
  onUpdateDeadline,
  onToggleGip,
  onUpdateAdvanceCost,
  onToggleAdvancePayment,
  onUpdateFinalCost,
  onToggleFinalPayment,
}: StageFinancialsProps) {
  return (
    <>
      <div className={styles.fields}>
        <EditableField
          label="Название"
          displayValue={detail.title}
          rawValue={detail.title}
          onSave={onUpdateTitle}
        />
        <EditableField
          label="Срок выполнения"
          displayValue={
            detail.deadline
              ? new Date(detail.deadline).toLocaleDateString('ru-RU', {
                  day: '2-digit',
                  month: 'long',
                  year: 'numeric',
                })
              : '—'
          }
          rawValue={detail.deadline?.slice(0, 10) ?? ''}
          type="date"
          onSave={onUpdateDeadline}
        />
        <div className={`${styles.field} ${styles.fieldEditable}`} onClick={onToggleGip}>
          <span className={styles.fieldLabel}>Выполнение</span>
          <span className={styles.fieldValue}>
            <span className={detail.gip_confirmed ? styles.completedBadge : styles.pendingBadge}>
              {detail.gip_confirmed ? 'Выполнено' : 'Не выполнено'}
            </span>
          </span>
        </div>
      </div>

      <div className={styles.fields}>
        <div className={styles.splitRow}>
          <EditableField
            label="Аванс"
            displayValue={
              detail.advance_cost != null ? `${formatNumber(detail.advance_cost)} ₽` : '—'
            }
            rawValue={detail.advance_cost?.toString() ?? ''}
            type="number"
            onSave={onUpdateAdvanceCost}
          />
          <div
            className={`${styles.field} ${detail.advance_cost != null ? styles.fieldEditable : ''}`}
            onClick={detail.advance_cost != null ? onToggleAdvancePayment : undefined}
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
            displayValue={
              detail.final_cost != null ? `${formatNumber(detail.final_cost)} ₽` : '—'
            }
            rawValue={detail.final_cost?.toString() ?? ''}
            type="number"
            onSave={onUpdateFinalCost}
          />
          <div className={`${styles.field} ${styles.fieldEditable}`} onClick={onToggleFinalPayment}>
            <span className={styles.fieldLabel}>Подтверждение оплаты</span>
            <span className={styles.fieldValue}>
              <span className={detail.final_confirmed ? styles.completedBadge : styles.pendingBadge}>
                {detail.final_confirmed ? 'Подтверждено' : 'Не подтверждено'}
              </span>
            </span>
          </div>
        </div>
      </div>
    </>
  )
}
