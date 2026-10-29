import React from 'react'
import type {Stage} from '../../../../types'
import {selectStage} from '../../../../store/uiSlice'
import {formatNumber} from '../Helpers'
import styles from '../../MainPanel.module.scss'

interface ExecutionMatrixProps {
  sortedStagesForDashboard: Stage[]
  getStageLabel: (stage: Stage) => string
  dispatch: any
}

export default function ExecutionMatrix({
  sortedStagesForDashboard,
  getStageLabel,
  dispatch,
}: ExecutionMatrixProps) {
  return (
    <div className={styles.matrixWrapper}>
      <div className={styles.matrixScroll}>
        <table className={styles.matrixTable}>
          <thead>
            <tr>
              <th className={styles.matrixRowHeader}>Стадия</th>
              {sortedStagesForDashboard.map((stage) => {
                const label = getStageLabel(stage)
                return (
                  <th
                    key={label}
                    className={styles.matrixColHeader}
                    title={`${label}. ${stage.title}`}
                    onClick={() =>
                      dispatch(
                        selectStage({
                          parentPosition: stage.parent_position,
                          position: stage.position,
                        }),
                      )
                    }
                  >
                    {label}
                  </th>
                )
              })}
            </tr>
          </thead>
          <tbody>
            <tr>
              <td className={styles.matrixRowHeader}>Выполнение</td>
              {sortedStagesForDashboard.map((stage) => (
                <td key={getStageLabel(stage)} className={styles.matrixCell}>
                  <span
                    className={`${styles.matrixDot} ${stage.gip_confirmed ? styles.dotCompleted : styles.dotPending}`}
                    title={`Выполнение: ${stage.gip_confirmed ? 'Выполнено' : 'Не выполнено'}`}
                  />
                </td>
              ))}
            </tr>
            <tr>
              <td className={styles.matrixRowHeader}>Аванс</td>
              {sortedStagesForDashboard.map((stage) => {
                  const payment = stage.payments?.find((p) => p.kind === 'first_advance')
                  const hasCost = payment?.cost != null
                  const confirmed = payment?.confirmed ?? false
                return (
                  <td key={getStageLabel(stage)} className={styles.matrixCell}>
                      {hasCost ? (
                      <span
                        className={`${styles.matrixDot} ${confirmed ? styles.dotCompleted : styles.dotPending}`}
                        title={`Аванс: ${formatNumber(payment.cost)} ₽ - ${confirmed ? 'Подтвержден' : 'Не подтвержден'}`}
                      />
                    ) : (
                      <span className={styles.dotNotRequired} title="Аванс не предусмотрен">
                        -
                      </span>
                      )}
                  </td>
                )
              })}
            </tr>
            <tr>
                <td className={styles.matrixRowHeader}>Второй аванс</td>
                {sortedStagesForDashboard.map((stage) => {
                    const payment = stage.payments?.find((p) => p.kind === 'second_advance')
                    const hasCost = payment?.cost != null
                    const confirmed = payment?.confirmed ?? false
                    return (
                        <td key={getStageLabel(stage)} className={styles.matrixCell}>
                            {hasCost ? (
                                <span
                                    className={`${styles.matrixDot} ${confirmed ? styles.dotCompleted : styles.dotPending}`}
                                    title={`Второй аванс: ${formatNumber(payment.cost)} ₽ - ${confirmed ? 'Подтвержден' : 'Не подтвержден'}`}
                                />
                            ) : (
                                <span className={styles.dotNotRequired} title="Второй аванс не предусмотрен">
                        -
                      </span>
                    )}
                  </td>
                )
              })}
            </tr>
            <tr>
              <td className={styles.matrixRowHeader}>Оплата</td>
                {sortedStagesForDashboard.map((stage) => {
                    const payment = stage.payments?.find((p) => p.kind === 'final')
                    const confirmed = payment?.confirmed ?? false
                    return (
                        <td key={getStageLabel(stage)} className={styles.matrixCell}>
                    <span
                        className={`${styles.matrixDot} ${confirmed ? styles.dotCompleted : styles.dotPending}`}
                        title={`Оплата: ${payment?.cost != null ? `${formatNumber(payment.cost)} ₽` : '—'} - ${confirmed ? 'Подтвержден' : 'Не подтвержден'}`}
                    />
                        </td>
                    )
                })}
            </tr>
            <tr>
              <td className={styles.matrixRowHeader}>Акт</td>
              {sortedStagesForDashboard.map((stage) => (
                <td key={getStageLabel(stage)} className={styles.matrixCell}>
                  <span
                    className={`${styles.matrixDot} ${stage.has_act ? styles.dotCompleted : styles.dotPending}`}
                    title={`Акт сдачи-приемки: ${stage.has_act ? 'Загружен' : 'Не загружен'}`}
                  />
                </td>
              ))}
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  )
}
