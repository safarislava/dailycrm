import React, {useMemo, useState} from 'react'
import type {Stage} from '../../../../types'
import {selectStage} from '../../../../store/uiSlice'
import {formatNumber, parseCost} from '../Helpers'
import styles from '../../MainPanel.module.scss'

const toFloat = (val: number | string | null | undefined): number => parseCost(val) ?? 0

const getDonutPath = (startPercent: number, endPercent: number, r_in: number, r_out: number) => {
  const percent = endPercent - startPercent

  const getCoordinatesForPercent = (p: number, r: number) => {
    const x = 50 + r * Math.cos(2 * Math.PI * (p - 0.25))
    const y = 50 + r * Math.sin(2 * Math.PI * (p - 0.25))
    return [x, y]
  }

  if (percent >= 0.9999) {
    return (
      `M 50 ${50 - r_out} ` +
      `A ${r_out} ${r_out} 0 0 1 50 ${50 + r_out} ` +
      `A ${r_out} ${r_out} 0 0 1 50 ${50 - r_out} Z ` +
      `M 50 ${50 - r_in} ` +
      `A ${r_in} ${r_in} 0 0 0 50 ${50 + r_in} ` +
      `A ${r_in} ${r_in} 0 0 0 50 ${50 - r_in} Z`
    )
  }

  const [startX_out, startY_out] = getCoordinatesForPercent(startPercent, r_out)
  const [endX_out, endY_out] = getCoordinatesForPercent(endPercent, r_out)
  const [startX_in, startY_in] = getCoordinatesForPercent(startPercent, r_in)
  const [endX_in, endY_in] = getCoordinatesForPercent(endPercent, r_in)

  const largeArcFlag = percent > 0.5 ? 1 : 0

  return (
    `M ${startX_out} ${startY_out} ` +
    `A ${r_out} ${r_out} 0 ${largeArcFlag} 1 ${endX_out} ${endY_out} ` +
    `L ${endX_in} ${endY_in} ` +
    `A ${r_in} ${r_in} 0 ${largeArcFlag} 0 ${startX_in} ${startY_in} ` +
    `Z`
  )
}

interface BudgetDonutChartProps {
  stages: Stage[]
  sortedStagesForDashboard: Stage[]
  getStageLabel: (stage: Stage) => string
  dispatch: any
}

export default function BudgetDonutChart({
  stages,
  sortedStagesForDashboard,
  getStageLabel,
  dispatch,
}: BudgetDonutChartProps) {
  const [hoveredSlice, setHoveredSlice] = useState<{
    label: string
    title: string
    cost: number
    percent: number
  } | null>(null)
  const [hoveredSliceId, setHoveredSliceId] = useState<string | null>(null)

  const projectTotalCost = useMemo(() => {
      const sum = stages.reduce(
          (acc, s) => acc + (s.payments ?? []).reduce((pAcc, p) => pAcc + toFloat(p.cost), 0),
          0
      )
    return Math.round(sum * 100) / 100
  }, [stages])

  const projectConfirmedCost = useMemo(() => {
      const sum = stages.reduce(
          (acc, s) =>
              acc +
              (s.payments ?? []).reduce(
                  (pAcc, p) => pAcc + (p.confirmed ? toFloat(p.cost) : 0),
                  0
              ),
          0
      )
    return Math.round(sum * 100) / 100
  }, [stages])

  const pieChartSlices = useMemo(() => {
    if (projectTotalCost === 0) return { outer: [], inner: [] }

    const stageItems = sortedStagesForDashboard
        .map((s) => {
            const payments = s.payments ?? []
            const total = payments.reduce((acc, p) => acc + toFloat(p.cost), 0)
            const isConfirmed =
                payments.length > 0 && payments.every((p) => !toFloat(p.cost) || p.confirmed)
            return {
                id: `${s.parent_position}-${s.position}-stage`,
                label: getStageLabel(s),
                title: `${getStageLabel(s)}. ${s.title}`,
                cost: Math.round(total * 100) / 100,
                parent_position: s.parent_position,
                position: s.position,
                isConfirmed,
            }
        })
      .filter((item) => item.cost > 0)

      const paymentLabel = (kind: string) => {
          switch (kind) {
              case 'first_advance':
                  return 'аванс'
              case 'second_advance':
                  return '2-й аванс'
              case 'final':
                  return 'стоимость'
              default:
                  return kind
          }
      }

    const paymentItems: Array<{
      id: string
      label: string
      title: string
      cost: number
      isConfirmed: boolean
      parent_position: number
      position: number
    }> = []

    for (const s of sortedStagesForDashboard) {
        for (const p of s.payments ?? []) {
            const cost = toFloat(p.cost)
            if (cost > 0) {
                const suffix = paymentLabel(p.kind)
                paymentItems.push({
                    id: `${s.parent_position}-${s.position}-${p.kind}`,
                    label: `${getStageLabel(s)} (${suffix})`,
                    title: `${getStageLabel(s)}. ${s.title} (${suffix})`,
                    cost,
                    isConfirmed: p.confirmed,
                    parent_position: s.parent_position,
                    position: s.position,
                })
            }
      }
    }

    const stageColorMap = new Map<string, string>()
    stageItems.forEach((item, idx) => {
      stageColorMap.set(item.id.replace('-stage', ''), `hsl(${(idx * 137.5) % 360}, 65%, 55%)`)
    })

    let outerCumulative = 0
    const outerSlices = stageItems.map((item) => {
      const percent = item.cost / projectTotalCost
      const startPercent = outerCumulative
      const endPercent = outerCumulative + percent
      outerCumulative = endPercent

      const d = getDonutPath(startPercent, endPercent, 38, 48)
      const color = item.isConfirmed ? 'var(--chart-confirmed)' : 'var(--chart-unconfirmed)'

      return {
        ...item,
        d,
        color,
        percent: Math.round(percent * 100),
      }
    })

    let innerCumulative = 0
    const innerSlices = paymentItems.map((item) => {
      const percent = item.cost / projectTotalCost
      const startPercent = innerCumulative
      const endPercent = innerCumulative + percent
      innerCumulative = endPercent

      const d = getDonutPath(startPercent, endPercent, 24, 36)
      const stageKey = `${item.parent_position}-${item.position}`
      const baseColor = stageColorMap.get(stageKey) || '#ccc'
      const color = item.isConfirmed ? baseColor : 'var(--chart-unconfirmed)'

      return {
        ...item,
        d,
        color,
        percent: Math.round(percent * 100),
      }
    })

    return { outer: outerSlices, inner: innerSlices }
  }, [sortedStagesForDashboard, projectTotalCost, getStageLabel])

  if (projectTotalCost === 0) {
    return <div className={styles.noChart}>Стоимость этапов не указана</div>
  }

  return (
    <div className={styles.chartWrapper}>
      <div className={styles.chartContainer}>
        <svg viewBox="0 0 100 100" className={styles.pieSvg}>
          {pieChartSlices.outer.map((slice) => {
            const isHovered =
              slice.id === hoveredSliceId ||
              (hoveredSliceId && hoveredSliceId.startsWith(`${slice.parent_position}-${slice.position}-`))
            return (
              <path
                key={slice.id}
                d={slice.d}
                fill={slice.color}
                className={`${styles.pieSliceOuter} ${isHovered ? styles.hovered : ''}`}
                onMouseEnter={() => {
                  setHoveredSlice({ label: slice.label, title: slice.title, cost: slice.cost, percent: slice.percent })
                  setHoveredSliceId(slice.id)
                }}
                onMouseLeave={() => {
                  setHoveredSlice(null)
                  setHoveredSliceId(null)
                }}
                onClick={() =>
                  dispatch(
                    selectStage({
                      parentPosition: slice.parent_position,
                      position: slice.position,
                    }),
                  )
                }
              />
            )
          })}
          <circle cx="50" cy="50" r="38" className={styles.pieHole} />
          {pieChartSlices.inner.map((slice) => {
            const isHovered = slice.id === hoveredSliceId
            return (
              <path
                key={slice.id}
                d={slice.d}
                fill={slice.color}
                className={`${styles.pieSlice} ${isHovered ? styles.hovered : ''}`}
                onMouseEnter={() => {
                  setHoveredSlice({ label: slice.label, title: slice.title, cost: slice.cost, percent: slice.percent })
                  setHoveredSliceId(slice.id)
                }}
                onMouseLeave={() => {
                  setHoveredSlice(null)
                  setHoveredSliceId(null)
                }}
                onClick={() =>
                  dispatch(
                    selectStage({
                      parentPosition: slice.parent_position,
                      position: slice.position,
                    }),
                  )
                }
              />
            )
          })}
          <circle cx="50" cy="50" r="24" className={styles.pieHole} />
        </svg>
      </div>

      <div className={styles.hoverInfo}>
        {hoveredSlice ? (
          <>
            <div className={styles.hoverTitle} title={hoveredSlice.title}>
              {hoveredSlice.title}
            </div>
            <div className={styles.hoverCost}>
              {formatNumber(hoveredSlice.cost)} ₽ ({hoveredSlice.percent}%)
            </div>
          </>
        ) : (
          <div className={styles.hoverPlaceholder}>Наведите на сектор для деталей</div>
        )}
      </div>

      <div className={styles.chartStats}>
        Подтверждено: <span className={styles.statsPaid}>{formatNumber(projectConfirmedCost)} ₽</span> из{' '}
        <span className={styles.statsTotal}>{formatNumber(projectTotalCost)} ₽</span>
      </div>
    </div>
  )
}
