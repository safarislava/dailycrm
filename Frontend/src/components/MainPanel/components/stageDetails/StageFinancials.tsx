import React, {useEffect, useState} from 'react'
import type {DetailedStage} from '../../../../types'
import {EditableField, formatNumber, parseCost, validateCost} from '../Helpers'
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
  const [addingFirstAdvance, setAddingFirstAdvance] = useState(false)
  const [addingSecondAdvance, setAddingSecondAdvance] = useState(false)

  useEffect(() => {
    setAddingFirstAdvance(false)
    setAddingSecondAdvance(false)
  }, [detail.position, detail.parent_position, detail.project_id])

  const payments = detail.payments ?? []
  const hasCost = payments.some((p) => p.cost != null && parseCost(p.cost) != null)
  const totalCost = payments.reduce((acc, p) => acc + (parseCost(p.cost) ?? 0), 0)
  const confirmedCost = payments.reduce(
      (acc, p) => acc + (p.confirmed ? (parseCost(p.cost) ?? 0) : 0),
      0
  )

  const firstAdvance = payments.find((p) => p.kind === 'first_advance')
  const secondAdvance = payments.find((p) => p.kind === 'second_advance')
  const finalPayment = payments.find((p) => p.kind === 'final')

  const hasFirstAdvance =
      addingFirstAdvance ||
      (firstAdvance != null && (firstAdvance.cost != null || firstAdvance.confirmed))

  const hasSecondAdvance =
      hasFirstAdvance &&
      (addingSecondAdvance ||
          (secondAdvance != null && (secondAdvance.cost != null || secondAdvance.confirmed)))

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
            {hasCost && totalCost > 0 && confirmedCost === totalCost && (
                <span className={styles.completedBadge}>Оплачен полностью</span>
            )}
          </div>
          <span className={styles.totalCostValue}>
            {hasCost ? `${formatNumber(totalCost)} ₽` : '—'}
          </span>
          {hasCost && totalCost > 0 && confirmedCost > 0 && confirmedCost < totalCost && (
              <span className={styles.totalCostSub}>
              Подтверждено: {formatNumber(confirmedCost)} ₽
            </span>
          )}
        </div>

        {hasFirstAdvance && (
            <div className={styles.paymentBlock}>
              <div className={styles.paymentBlockHeader}>
                <span className={styles.paymentBlockTitle}>Аванс</span>
                {!hasSecondAdvance && (
                    <button
                        type="button"
                        className={styles.removePaymentBtn}
                        onClick={handleRemoveFirstAdvance}
                    >
                      Удалить
                    </button>
                )}
              </div>
              {firstAdvance?.cost != null ? (
              <div className={styles.splitRow}>
                <EditableField
                    label="Сумма аванса"
                    displayValue={`${formatNumber(firstAdvance.cost)} ₽`}
                    rawValue={formatNumber(firstAdvance.cost)}
                    type="text"
                    validate={validateCost}
                    onSave={onUpdateAdvanceCost}
                />
                <div
                    className={`${styles.field} ${styles.fieldEditable}`}
                    onClick={onToggleAdvancePayment}
                >
                  <span className={styles.fieldLabel}>Подтверждение</span>
                  <span className={styles.fieldValue}>
                    <span
                      className={
                        firstAdvance.confirmed ? styles.completedBadge : styles.pendingBadge
                      }
                    >
                      {firstAdvance.confirmed ? 'Подтверждено' : 'Не подтверждено'}
                    </span>
                  </span>
                </div>
              </div>
              ) : (
                  <EditableField
                      label="Сумма аванса"
                      displayValue="—"
                      rawValue=""
                      type="text"
                      validate={validateCost}
                      onSave={onUpdateAdvanceCost}
                  />
              )}
            </div>
        )}

        {hasSecondAdvance && (
            <div className={styles.paymentBlock}>
              <div className={styles.paymentBlockHeader}>
                <span className={styles.paymentBlockTitle}>Второй аванс</span>
                <button
                    type="button"
                    className={styles.removePaymentBtn}
                    onClick={handleRemoveSecondAdvance}
                >
                  Удалить
                </button>
              </div>
              {secondAdvance?.cost != null ? (
              <div className={styles.splitRow}>
                <EditableField
                    label="Сумма второго аванса"
                    displayValue={`${formatNumber(secondAdvance.cost)} ₽`}
                    rawValue={formatNumber(secondAdvance.cost)}
                    type="text"
                    validate={validateCost}
                    onSave={onUpdateSecondAdvanceCost}
                />
                <div
                    className={`${styles.field} ${styles.fieldEditable}`}
                    onClick={onToggleSecondAdvancePayment}
                >
                  <span className={styles.fieldLabel}>Подтверждение</span>
                  <span className={styles.fieldValue}>
                    <span
                      className={
                        secondAdvance.confirmed ? styles.completedBadge : styles.pendingBadge
                      }
                    >
                      {secondAdvance.confirmed ? 'Подтверждено' : 'Не подтверждено'}
                    </span>
                  </span>
                </div>
              </div>
              ) : (
                  <EditableField
                      label="Сумма второго аванса"
                      displayValue="—"
                      rawValue=""
                      type="text"
                      validate={validateCost}
                      onSave={onUpdateSecondAdvanceCost}
                  />
              )}
            </div>
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

        <div className={styles.paymentBlock}>
          <div className={styles.paymentBlockHeader}>
            <span className={styles.paymentBlockTitle}>Окончательная оплата</span>
          </div>
          {finalPayment?.cost != null ? (
              <div className={styles.splitRow}>
                <EditableField
                    label="Сумма оплаты"
                    displayValue={`${formatNumber(finalPayment.cost)} ₽`}
                    rawValue={formatNumber(finalPayment.cost)}
                type="text"
                validate={validateCost}
                onSave={onUpdateFinalCost}
                />
                <div
                className={`${styles.field} ${styles.fieldEditable}`}
                onClick={onToggleFinalPayment}
                >
                  <span className={styles.fieldLabel}>Подтверждение</span>
                  <span className={styles.fieldValue}>
                  <span
                      className={
                        finalPayment.confirmed ? styles.completedBadge : styles.pendingBadge
                      }
                  >
                    {finalPayment.confirmed ? 'Подтверждено' : 'Не подтверждено'}
                  </span>
                </span>
                </div>
              </div>
          ) : (
              <EditableField
                  label="Сумма оплаты"
                  displayValue="—"
                  rawValue=""
                  type="text"
                  validate={validateCost}
                  onSave={onUpdateFinalCost}
              />
          )}
        </div>
      </div>
    </>
  )
}
