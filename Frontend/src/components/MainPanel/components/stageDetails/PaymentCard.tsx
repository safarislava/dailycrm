import React, {useState} from 'react'
import type {StagePayment} from '../../../../types'
import {ChevronRightIcon, EditableField, formatNumber, formatRawCost, parseCost, validateCost} from '../Helpers'
import styles from '../../MainPanel.module.scss'

interface PaymentCardProps {
    title: string
    payment?: StagePayment
    onUpdateCost: (cost: string) => Promise<void>
    onRemovePayment?: () => Promise<void>
    onAddTranche: (amount: number) => Promise<void>
    onDeleteTranche: (trancheId: string) => Promise<void>
    onToggleConfirmed?: () => void
}

export default function PaymentCard({
                                        title,
                                        payment,
                                        onUpdateCost,
                                        onRemovePayment,
                                        onAddTranche,
                                        onDeleteTranche,
                                        onToggleConfirmed,
                                    }: PaymentCardProps) {
    const [showPartForm, setShowPartForm] = useState(false)
    const [partAmount, setPartAmount] = useState('')
    const [partError, setPartError] = useState<string | null>(null)
    const [isSubmitting, setIsSubmitting] = useState(false)
    const [isHistoryExpanded, setIsHistoryExpanded] = useState(false)

    const rawCost = payment?.cost
    const cost = parseCost(rawCost) ?? 0
    const tranches = payment?.tranches ?? []
    const paidAmount = Math.round(tranches.reduce((sum, t) => sum + (parseCost(t.amount) ?? 0), 0) * 100) / 100
    const remaining = Math.max(0, Math.round((cost - paidAmount) * 100) / 100)
    const isFullyPaid = (payment?.confirmed || (cost > 0 && remaining <= 0)) && cost > 0
    const isPartiallyPaid = paidAmount > 0 && !isFullyPaid

    const validateCostWithPaid = (val: string): string | null => {
        const basicError = validateCost(val)
        if (basicError) return basicError
        const parsed = parseCost(val) ?? 0
        if (parsed < paidAmount) {
            return `Сумма не может быть меньше уже внесенных оплат (${formatNumber(paidAmount)} ₽)`
        }
        return null
    }

    const handlePayRemaining = async () => {
        if (remaining <= 0 || isSubmitting) return
        try {
            setIsSubmitting(true)
            await onAddTranche(remaining)
        } catch (err: any) {
            alert(err?.data?.message || err?.message || 'Ошибка внесения оплаты')
        } finally {
            setIsSubmitting(false)
        }
    }

    const handlePartSubmit = async (e?: React.FormEvent) => {
        if (e) e.preventDefault()
        const err = validateCost(partAmount)
        if (err) {
            setPartError(err)
            return
        }
        const amount = parseCost(partAmount)
        if (amount == null || amount <= 0) {
            setPartError('Введите сумму больше 0')
            return
        }
        if (amount > remaining) {
            setPartError(`Сумма не может превышать остаток (${formatNumber(remaining)} ₽)`)
            return
        }
        try {
            setIsSubmitting(true)
            setPartError(null)
            await onAddTranche(amount)
            setShowPartForm(false)
            setPartAmount('')
        } catch (err: any) {
            setPartError(err?.data?.message || err?.message || 'Ошибка внесения оплаты')
        } finally {
            setIsSubmitting(false)
        }
    }

    const handleRemove = async () => {
        if (tranches.length > 0) {
            alert('Нельзя удалить платеж с внесенными траншами. Сначала удалите историю оплат.')
            return
        }
        if (onRemovePayment) {
            await onRemovePayment()
        }
    }

    return (
        <div className={styles.paymentBlock}>
            <div className={styles.paymentBlockHeader}>
                <span className={styles.paymentBlockTitle}>{title}</span>
                <div className={styles.paymentHeaderActions}>
                    {cost > 0 && remaining > 0 && !showPartForm && (
                        <button
                            type="button"
                            className={styles.addPartHeaderBtn}
                            onClick={() => {
                                setPartAmount(formatRawCost(remaining))
                                setPartError(null)
                                setShowPartForm(true)
                            }}
                        >
                            + Внести часть
                        </button>
                    )}
                    {onRemovePayment && (
                        <button
                            type="button"
                            className={styles.removePaymentBtn}
                            onClick={handleRemove}
                        >
                            Удалить
                        </button>
                    )}
                </div>
            </div>

            {rawCost != null ? (
                <div className={styles.splitRow}>
                    <EditableField
                        label="Плановая сумма"
                        displayValue={`${formatNumber(cost)} ₽`}
                        rawValue={formatRawCost(cost)}
                        type="text"
                        validate={validateCostWithPaid}
                        stripSpaces
                        onSave={onUpdateCost}
                    />
                    <div
                        className={`${styles.field} ${cost > 0 ? styles.fieldEditable : ''}`}
                        onClick={() => {
                            if (cost > 0 && remaining > 0) {
                                handlePayRemaining()
                            } else if (cost > 0) {
                                setIsHistoryExpanded(!isHistoryExpanded)
                            }
                        }}
                        title={cost > 0 && remaining > 0 ? `Оплатить остаток ${formatNumber(remaining)} ₽` : 'История платежей'}
                    >
                        <span className={styles.fieldLabel}>Статус оплаты</span>
                        <span className={styles.fieldValue}>
              {isFullyPaid && (
                  <span className={styles.completedBadge}>Оплачен</span>
              )}
                            {isPartiallyPaid && (
                                <span
                                    className={styles.partialBadge}
                                    title={`Оплачено ${formatNumber(paidAmount)} из ${formatNumber(cost)} ₽ (Остаток: ${formatNumber(remaining)} ₽)`}
                                >
                  Оплачено {formatNumber(paidAmount)} ₽
                </span>
                            )}
                            {!isFullyPaid && !isPartiallyPaid && (
                                <span className={styles.pendingBadge}>Не оплачен</span>
                            )}
            </span>
                    </div>
                </div>
            ) : (
                <EditableField
                    label="Плановая сумма"
                    displayValue="—"
                    rawValue=""
                    type="text"
                    validate={validateCost}
                    stripSpaces
                    onSave={onUpdateCost}
                />
            )}

            {showPartForm && (
                <div>
                    <form className={styles.partForm} onSubmit={handlePartSubmit}>
                        <input
                            type="text"
                            autoFocus
                            className={styles.partInput}
                            value={partAmount}
                            placeholder={`До ${formatNumber(remaining)} ₽`}
                            onChange={(e) => {
                                setPartAmount(e.target.value.replace(/\s/g, ''))
                                setPartError(null)
                            }}
                        />
                        <button
                            type="submit"
                            className={styles.partSubmitBtn}
                            disabled={isSubmitting}
                        >
                            Внести
                        </button>
                        <button
                            type="button"
                            className={styles.partCancelBtn}
                            onClick={() => {
                                setShowPartForm(false)
                                setPartError(null)
                            }}
                        >
                            ✕
                        </button>
                    </form>
                    {partError && <div className={styles.partError}>{partError}</div>}
                </div>
            )}

            {cost > 0 && (
                <div className={styles.trancheHistorySection}>
                    <div
                        className={`${styles.trancheHistoryHeader} ${isHistoryExpanded ? styles.trancheHistoryHeaderOpen : ''}`}
                        onClick={() => setIsHistoryExpanded(!isHistoryExpanded)}
                    >
                        <div className={styles.trancheHistoryTitleRow}>
              <span className={`${styles.trancheChevron} ${isHistoryExpanded ? styles.trancheChevronOpen : ''}`}>
                <ChevronRightIcon/>
              </span>
                            <span className={styles.trancheHistoryLabel}>История платежей</span>
                            <span className={styles.trancheHistoryCount}>{tranches.length}</span>
                        </div>
                    </div>

                    {isHistoryExpanded && (
                        <div className={styles.trancheHistoryList}>
                            {tranches.length === 0 ? (
                                <div className={styles.trancheEmpty}>Нет внесенных платежей</div>
                            ) : (
                                tranches.map((t) => {
                                    const dateStr = t.created_at
                                        ? new Date(t.created_at).toLocaleDateString('ru-RU', {
                                            day: '2-digit',
                                            month: '2-digit',
                                            year: 'numeric',
                                            hour: '2-digit',
                                            minute: '2-digit',
                                        })
                                        : ''
                                    return (
                                        <div key={t.id} className={styles.trancheItem}>
                                            <div className={styles.trancheItemInfo}>
                        <span className={styles.trancheAmount}>
                          +{formatNumber(t.amount)} ₽
                        </span>
                                                <span className={styles.trancheDate}>{dateStr}</span>
                                            </div>
                                            <button
                                                type="button"
                                                className={styles.trancheDeleteBtn}
                                                title="Удалить платеж"
                                                onClick={(e) => {
                                                    e.stopPropagation()
                                                    onDeleteTranche(t.id)
                                                }}
                                            >
                                                ×
                                            </button>
                                        </div>
                                    )
                                })
                            )}
                        </div>
                    )}
                </div>
            )}
        </div>
    )
}
