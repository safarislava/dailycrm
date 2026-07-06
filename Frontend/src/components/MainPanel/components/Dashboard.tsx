import React, { useState, useMemo } from 'react'
import type { Stage } from '../../../types'
import { selectStage } from '../../../store/uiSlice'
import styles from '../MainPanel.module.scss'

const getDonutPath = (startPercent: number, endPercent: number, r_in: number, r_out: number) => {
  const percent = endPercent - startPercent;
  
  const getCoordinatesForPercent = (p: number, r: number) => {
    const x = 50 + r * Math.cos(2 * Math.PI * (p - 0.25))
    const y = 50 + r * Math.sin(2 * Math.PI * (p - 0.25))
    return [x, y]
  }

  if (percent >= 0.9999) {
    return `M 50 ${50 - r_out} ` +
           `A ${r_out} ${r_out} 0 0 1 50 ${50 + r_out} ` +
           `A ${r_out} ${r_out} 0 0 1 50 ${50 - r_out} Z ` +
           `M 50 ${50 - r_in} ` +
           `A ${r_in} ${r_in} 0 0 0 50 ${50 + r_in} ` +
           `A ${r_in} ${r_in} 0 0 0 50 ${50 - r_in} Z`;
  }

  const [startX_out, startY_out] = getCoordinatesForPercent(startPercent, r_out)
  const [endX_out, endY_out] = getCoordinatesForPercent(endPercent, r_out)
  const [startX_in, startY_in] = getCoordinatesForPercent(startPercent, r_in)
  const [endX_in, endY_in] = getCoordinatesForPercent(endPercent, r_in)
  
  const largeArcFlag = percent > 0.5 ? 1 : 0

  return `M ${startX_out} ${startY_out} ` +
         `A ${r_out} ${r_out} 0 ${largeArcFlag} 1 ${endX_out} ${endY_out} ` +
         `L ${endX_in} ${endY_in} ` +
         `A ${r_in} ${r_in} 0 ${largeArcFlag} 0 ${startX_in} ${startY_in} ` +
         `Z`;
}

interface DashboardProps {
  projectId: string | null
  stagesLoading: boolean
  stages: Stage[]
  sortedStagesForDashboard: Stage[]
  getStageLabel: (stage: Stage) => string
  dispatch: any
}

export default function Dashboard({
  projectId,
  stagesLoading,
  stages,
  sortedStagesForDashboard,
  getStageLabel,
  dispatch,
}: DashboardProps) {
  const [matrixOpen, setMatrixOpen] = useState(true)
  const [budgetOpen, setBudgetOpen] = useState(true)
  const [hoveredSlice, setHoveredSlice] = useState<{ label: string; title: string; cost: number; percent: number } | null>(null)
  const [hoveredSliceId, setHoveredSliceId] = useState<string | null>(null)

  const projectTotalCost = useMemo(() => {
    return stages.reduce((acc, s) => acc + (s.advance_cost ?? 0) + (s.final_cost ?? 0), 0)
  }, [stages])

  const projectConfirmedCost = useMemo(() => {
    return stages.reduce((acc, s) => {
      const adv = s.advance_confirmed ? (s.advance_cost ?? 0) : 0
      const fin = s.final_confirmed ? (s.final_cost ?? 0) : 0
      return acc + adv + fin
    }, 0)
  }, [stages])

  const pieChartSlices = useMemo(() => {
    if (projectTotalCost === 0) return { outer: [], inner: [] }
    
    // 1. Build cost items for the outer ring (stages)
    const stageItems = sortedStagesForDashboard
      .map(s => ({
        id: `${s.parent_position}-${s.position}-stage`,
        label: getStageLabel(s),
        title: `${getStageLabel(s)}. ${s.title}`,
        cost: (s.advance_cost ?? 0) + (s.final_cost ?? 0),
        parent_position: s.parent_position,
        position: s.position,
        isConfirmed: (!s.advance_cost || s.advance_confirmed) && s.final_confirmed
      }))
      .filter(item => item.cost > 0)

    // 2. Build cost items for the inner ring (payments)
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
      if (s.advance_cost && s.advance_cost > 0) {
        paymentItems.push({
          id: `${s.parent_position}-${s.position}-advance`,
          label: `${getStageLabel(s)} (аванс)`,
          title: `${getStageLabel(s)}. ${s.title} (аванс)`,
          cost: s.advance_cost,
          isConfirmed: s.advance_confirmed,
          parent_position: s.parent_position,
          position: s.position
        })
      }
      if (s.final_cost && s.final_cost > 0) {
        paymentItems.push({
          id: `${s.parent_position}-${s.position}-final`,
          label: `${getStageLabel(s)} (стоимость)`,
          title: `${getStageLabel(s)}. ${s.title} (стоимость)`,
          cost: s.final_cost,
          isConfirmed: s.final_confirmed,
          parent_position: s.parent_position,
          position: s.position
        })
      }
    }

    // Map stages to HSL colors to ensure alignment of colors between inner segments
    const stageColorMap = new Map<string, string>()
    stageItems.forEach((item, idx) => {
      stageColorMap.set(item.id.replace('-stage', ''), `hsl(${(idx * 137.5) % 360}, 65%, 55%)`)
    })

    // Calculate outer slices
    let outerCumulative = 0
    const outerSlices = stageItems.map(item => {
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
        percent: Math.round(percent * 100)
      }
    })

    // Calculate inner slices
    let innerCumulative = 0
    const innerSlices = paymentItems.map(item => {
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
        percent: Math.round(percent * 100)
      }
    })

    return { outer: outerSlices, inner: innerSlices }
  }, [sortedStagesForDashboard, projectTotalCost, getStageLabel])

  if (!projectId || stagesLoading || stages.length === 0) {
    return null
  }

  return (
    <div className={styles.tabContentDashboard}>
      {/* Panel 1: Checkpoint Matrix */}
      <div className={styles.dashboardContainer}>
        <div className={styles.dashboardHeader} onClick={() => setMatrixOpen(o => !o)}>
          <span className={styles.dashboardHeaderTitle}>Матрица выполнения</span>
          <span className={styles.dashboardHeaderToggle}>
            {matrixOpen ? 'Свернуть ▲' : 'Развернуть ▼'}
          </span>
        </div>
        {matrixOpen && (
          <div className={styles.dashboardBody}>
            <div className={styles.matrixWrapper}>
              <div className={styles.matrixScroll}>
                <table className={styles.matrixTable}>
                  <thead>
                    <tr>
                      <th className={styles.matrixRowHeader}>Стадия</th>
                      {sortedStagesForDashboard.map(stage => {
                        const label = getStageLabel(stage)
                        return (
                          <th
                            key={label}
                            className={styles.matrixColHeader}
                            title={`${label}. ${stage.title}`}
                            onClick={() => dispatch(selectStage({ parentPosition: stage.parent_position, position: stage.position }))}
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
                      {sortedStagesForDashboard.map(stage => (
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
                      {sortedStagesForDashboard.map(stage => {
                        const hasAdvance = stage.advance_cost != null
                        const confirmed = stage.advance_confirmed
                        return (
                          <td key={getStageLabel(stage)} className={styles.matrixCell}>
                            {hasAdvance ? (
                              <span
                                className={`${styles.matrixDot} ${confirmed ? styles.dotCompleted : styles.dotPending}`}
                                title={`Аванс: ${stage.advance_cost?.toLocaleString()} ₽ - ${confirmed ? 'Подтвержден' : 'Не подтвержден'}`}
                              />
                            ) : (
                              <span className={styles.dotNotRequired} title="Аванс не предусмотрен">-</span>
                            )}
                          </td>
                        )
                      })}
                    </tr>
                    <tr>
                      <td className={styles.matrixRowHeader}>Оплата</td>
                      {sortedStagesForDashboard.map(stage => (
                        <td key={getStageLabel(stage)} className={styles.matrixCell}>
                          <span
                            className={`${styles.matrixDot} ${stage.final_confirmed ? styles.dotCompleted : styles.dotPending}`}
                            title={`Оплата: ${stage.final_cost != null ? `${stage.final_cost.toLocaleString()} ₽` : '—'} - ${stage.final_confirmed ? 'Подтвержден' : 'Не подтвержден'}`}
                          />
                        </td>
                      ))}
                    </tr>
                    <tr>
                      <td className={styles.matrixRowHeader}>Акт</td>
                      {sortedStagesForDashboard.map(stage => (
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
          </div>
        )}
      </div>

      {/* Panel 2: Budget Distribution */}
      <div className={styles.dashboardContainer}>
        <div className={styles.dashboardHeader} onClick={() => setBudgetOpen(o => !o)}>
          <span className={styles.dashboardHeaderTitle}>Распределение бюджета</span>
          <span className={styles.dashboardHeaderToggle}>
            {budgetOpen ? 'Свернуть ▲' : 'Развернуть ▼'}
          </span>
        </div>
        {budgetOpen && (
          <div className={styles.dashboardBody}>
            <div className={styles.chartWrapper}>
              {projectTotalCost > 0 ? (
                <>
                  <div className={styles.chartContainer}>
                    <svg viewBox="0 0 100 100" className={styles.pieSvg}>
                      {/* Outer Ring: Stages */}
                      {pieChartSlices.outer.map(slice => {
                        const isHovered = slice.id === hoveredSliceId || (hoveredSliceId && hoveredSliceId.startsWith(`${slice.parent_position}-${slice.position}-`))
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
                            onClick={() => dispatch(selectStage({ parentPosition: slice.parent_position, position: slice.position }))}
                          />
                        )
                      })}
                      
                      {/* Separation Ring Gap (background color) */}
                      <circle cx="50" cy="50" r="38" className={styles.pieHole} />

                      {/* Inner Ring: Payments */}
                      {pieChartSlices.inner.map(slice => {
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
                            onClick={() => dispatch(selectStage({ parentPosition: slice.parent_position, position: slice.position }))}
                          />
                        )
                      })}

                      {/* Doughnut Hole */}
                      <circle cx="50" cy="50" r="24" className={styles.pieHole} />
                    </svg>
                  </div>
                  
                  {/* Hover details outside the chart */}
                  <div className={styles.hoverInfo}>
                    {hoveredSlice ? (
                      <>
                        <div className={styles.hoverTitle} title={hoveredSlice.title}>{hoveredSlice.title}</div>
                        <div className={styles.hoverCost}>
                          {hoveredSlice.cost.toLocaleString()} ₽ ({hoveredSlice.percent}%)
                        </div>
                      </>
                    ) : (
                      <div className={styles.hoverPlaceholder}>
                        Наведите на сектор для деталей
                      </div>
                    )}
                  </div>

                  <div className={styles.chartStats}>
                    Подтверждено: <span className={styles.statsPaid}>{projectConfirmedCost.toLocaleString()} ₽</span> из <span className={styles.statsTotal}>{projectTotalCost.toLocaleString()} ₽</span>
                  </div>
                </>
              ) : (
                <div className={styles.noChart}>
                  Стоимость этапов не указана
                </div>
              )}
            </div>
          </div>
        )}
      </div>
    </div>
  )
}
