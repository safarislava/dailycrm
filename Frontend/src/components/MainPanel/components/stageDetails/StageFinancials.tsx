import React from 'react'
import type {DetailedStage} from '../../../../types'
import {EditableField, formatNumber, validateCost} from '../Helpers'
import styles from '../../MainPanel.module.scss'

interface StageFinancialsProps {
  detail: DetailedStage
  onUpdateTitle: (title: string) => Promise<void>
  onUpdateDeadline: (deadline: string) => Promise<void>
  onToggleGip: () => void
  onUpdateAdvanceCost: (cost: string) => Promise<void>
  onToggleAdvancePayment: () => void
  onUpdateSecondAdvanceCost: (cost: string) => Promise<void>
  onToggleSecondAdvancePayment: () => void
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
                                          onUpdateSecondAdvanceCost,
                                          onToggleSecondAdvancePayment,
  onUpdateFinalCost,
  onToggleFinalPayment,
}: StageFinancialsProps) {
  const firstAdvance = detail.payments?.find((p) => p.kind === 'first_advance')
  const secondAdvance = detail.payments?.find((p) => p.kind === 'second_advance')
  const finalPayment = detail.payments?.find((p) => p.kind === 'final')

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
              firstAdvance?.cost != null ? `${formatNumber(firstAdvance.cost)} ₽` : '—'
            }
            rawValue={firstAdvance?.cost != null ? formatNumber(firstAdvance.cost) : ''}
            type="text"
            validate={validateCost}
            onSave={onUpdateAdvanceCost}
          />
          <div
              className={`${styles.field} ${firstAdvance?.cost != null ? styles.fieldEditable : ''}`}
              onClick={firstAdvance?.cost != null ? onToggleAdvancePayment : undefined}
          >
            <span className={styles.fieldLabel}>Подтверждение аванса</span>
            <span className={styles.fieldValue}>
              {firstAdvance?.cost == null ? (
                  <span className={styles.pendingBadge}>Не требуется</span>
              ) : (
                  <span
                      className={
                        firstAdvance.confirmed ? styles.completedBadge : styles.pendingBadge
                      }
                  >
                  {firstAdvance.confirmed ? 'Подтверждено' : 'Не подтверждено'}
                </span>
              )}
            </span>
          </div>
        </div>

        <div className={styles.splitRow}>
          <EditableField
              label="Второй аванс"
              displayValue={
                secondAdvance?.cost != null ? `${formatNumber(secondAdvance.cost)} ₽` : '—'
              }
              rawValue={secondAdvance?.cost != null ? formatNumber(secondAdvance.cost) : ''}
              type="text"
              validate={validateCost}
              onSave={onUpdateSecondAdvanceCost}
          />
          <div
              className={`${styles.field} ${secondAdvance?.cost != null ? styles.fieldEditable : ''}`}
              onClick={secondAdvance?.cost != null ? onToggleSecondAdvancePayment : undefined}
          >
            <span className={styles.fieldLabel}>Подтверждение второго аванса</span>
            <span className={styles.fieldValue}>
              {secondAdvance?.cost == null ? (
                <span className={styles.pendingBadge}>Не требуется</span>
              ) : (
                  <span
                      className={
                        secondAdvance.confirmed ? styles.completedBadge : styles.pendingBadge
                      }
                  >
                  {secondAdvance.confirmed ? 'Подтверждено' : 'Не подтверждено'}
                </span>
              )}
            </span>
          </div>
        </div>

        <div className={styles.splitRow}>
          <EditableField
            label="Окончательная оплата"
            displayValue={
              finalPayment?.cost != null ? `${formatNumber(finalPayment.cost)} ₽` : '—'
            }
            rawValue={finalPayment?.cost != null ? formatNumber(finalPayment.cost) : ''}
            type="text"
            validate={validateCost}
            onSave={onUpdateFinalCost}
          />
          <div className={`${styles.field} ${styles.fieldEditable}`} onClick={onToggleFinalPayment}>
            <span className={styles.fieldLabel}>Подтверждение оплаты</span>
            <span className={styles.fieldValue}>
              <span
                  className={
                    finalPayment?.confirmed ? styles.completedBadge : styles.pendingBadge
                  }
              >
                {finalPayment?.confirmed ? 'Подтверждено' : 'Не подтверждено'}
              </span>
            </span>
          </div>
        </div>
      </div>
    </>
  )
}
