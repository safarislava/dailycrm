import React, {useEffect, useState} from 'react'
import type {DetailedStage} from '../../../../types'
import {EditableField, formatNumber, parseCost} from '../Helpers'
import PaymentCard from './PaymentCard'
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
  onAddTranche: (kind: 'first_advance' | 'second_advance' | 'final', amount: number) => Promise<void>
  onDeleteTranche: (kind: 'first_advance' | 'second_advance' | 'final', trancheId: string) => Promise<void>
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
                                          onAddTranche,
                                          onDeleteTranche,
}: StageFinancialsProps) {
  const [addingFirstAdvance, setAddingFirstAdvance] = useState(false)
  const [addingSecondAdvance, setAddingSecondAdvance] = useState(false)

  useEffect(() => {
    setAddingFirstAdvance(false)
    setAddingSecondAdvance(false)
  }, [detail.position, detail.parent_position, detail.project_id])

  const payments = detail.payments ?? []
  const hasCost = payments.some((p) => p.cost != null && parseCost(p.cost) != null)
  const totalCost = payments.reduce((acc, p) => acc + (parseCost(p.cost) ?? 0), 0)
  const totalPaid = payments.reduce(
      (acc, p) =>
          acc +
          (p.tranches ?? []).reduce((tSum, t) => tSum + (parseCost(t.amount) ?? 0), 0),
      0
  )

  const isAllFullyPaid = hasCost && totalCost > 0 && totalPaid >= totalCost

  const firstAdvance = payments.find((p) => p.kind === 'first_advance')
  const secondAdvance = payments.find((p) => p.kind === 'second_advance')
  const finalPayment = payments.find((p) => p.kind === 'final')

  const hasFirstAdvance =
      addingFirstAdvance ||
      (firstAdvance != null && (firstAdvance.cost != null || (firstAdvance.tranches?.length ?? 0) > 0 || firstAdvance.confirmed))

  const hasSecondAdvance =
      hasFirstAdvance &&
      (addingSecondAdvance ||
          (secondAdvance != null && (secondAdvance.cost != null || (secondAdvance.tranches?.length ?? 0) > 0 || secondAdvance.confirmed)))

  const handleRemoveFirstAdvance = async () => {
    setAddingFirstAdvance(false)
    if (firstAdvance?.confirmed) {
      onToggleAdvancePayment()
    }
    await onUpdateAdvanceCost('')
  }

  const handleRemoveSecondAdvance = async () => {
    setAddingSecondAdvance(false)
    if (secondAdvance?.confirmed) {
      onToggleSecondAdvancePayment()
    }
    await onUpdateSecondAdvanceCost('')
  }

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
        <div className={styles.totalCostCard}>
          <div className={styles.totalCostHeader}>
            <span className={styles.fieldLabel}>Общая сумма этапа</span>
            {isAllFullyPaid && (
                <span className={styles.completedBadge}>Оплачен полностью</span>
            )}
            {!isAllFullyPaid && totalPaid > 0 && (
                <span className={styles.partialBadge}>
                Оплачен частично
              </span>
            )}
          </div>
          <span className={styles.totalCostValue}>
            {hasCost && totalCost > 0 ? (
                <>
                  <span className={styles.totalCostLabel}>Оплачено </span>
                  <span className={styles.totalCostAmount}>{formatNumber(totalPaid)} ₽</span>
                  <span className={styles.totalCostLabel}> из </span>
                  <span className={styles.totalCostAmount}>{formatNumber(totalCost)} ₽</span>
                </>
            ) : (
                '—'
            )}
          </span>
          {hasCost && totalCost > 0 && (
              <div className={styles.stageProgressBar}>
                <div
                    className={styles.stageProgressFill}
                    style={{width: `${Math.min(100, (totalPaid / totalCost) * 100)}%`}}
                />
              </div>
          )}
        </div>

        {hasFirstAdvance && (
            <PaymentCard
                title="Аванс"
                payment={firstAdvance}
                onUpdateCost={onUpdateAdvanceCost}
                onRemovePayment={!hasSecondAdvance ? handleRemoveFirstAdvance : undefined}
                onAddTranche={(amount) => onAddTranche('first_advance', amount)}
                onDeleteTranche={(id) => onDeleteTranche('first_advance', id)}
                onToggleConfirmed={onToggleAdvancePayment}
            />
        )}

        {hasSecondAdvance && (
            <PaymentCard
                title="Второй аванс"
                payment={secondAdvance}
                onUpdateCost={onUpdateSecondAdvanceCost}
                onRemovePayment={handleRemoveSecondAdvance}
                onAddTranche={(amount) => onAddTranche('second_advance', amount)}
                onDeleteTranche={(id) => onDeleteTranche('second_advance', id)}
                onToggleConfirmed={onToggleSecondAdvancePayment}
            />
        )}

        {!hasFirstAdvance && (
            <button
                type="button"
                className={styles.addPaymentBtn}
                onClick={() => setAddingFirstAdvance(true)}
            >
              + Добавить аванс
            </button>
        )}

        {hasFirstAdvance && !hasSecondAdvance && (
            <button
                type="button"
                className={styles.addPaymentBtn}
                onClick={() => setAddingSecondAdvance(true)}
            >
              + Добавить второй аванс
            </button>
        )}

        <PaymentCard
            title="Окончательная оплата"
            payment={finalPayment}
            onUpdateCost={onUpdateFinalCost}
            onAddTranche={(amount) => onAddTranche('final', amount)}
            onDeleteTranche={(id) => onDeleteTranche('final', id)}
            onToggleConfirmed={onToggleFinalPayment}
        />
      </div>
    </>
  )
}
